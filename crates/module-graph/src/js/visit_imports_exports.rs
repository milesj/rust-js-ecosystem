use super::stats::JavaScriptStats;
use crate::module::*;
use oxc::ast::ast::{
    Argument, BindingPattern, BindingPatternKind, CallExpression, Declaration,
    ExportAllDeclaration, ExportDefaultDeclaration, ExportDefaultDeclarationKind,
    ExportNamedDeclaration, Expression, ImportDeclaration, ImportDeclarationSpecifier,
    ImportExpression, ModuleDeclaration, Statement, StaticMemberExpression,
    TSImportEqualsDeclaration, TSModuleReference, VariableDeclarator,
};
use oxc::ast::{AstKind, Visit};
use oxc::span::{Atom, Span};
use rustc_hash::FxHashSet;
use std::marker::PhantomData;

pub struct ExtractImportsExports<'ast, 'module> {
    pub module: &'module mut Module,
    pub stats: &'module mut JavaScriptStats,
    pub extracted_dynamic_imports: FxHashSet<Span>,
    pub extracted_requires: FxHashSet<Span>,
    pub ast: PhantomData<&'ast ()>,
}

// TODO non-literal paths
impl<'ast, 'module> Visit<'ast> for ExtractImportsExports<'ast, 'module> {
    fn enter_node(&mut self, kind: AstKind<'ast>) {
        match kind {
            AstKind::Program(program) => {
                for stmt in &program.body {
                    if let Statement::ModuleDeclaration(decl) = &stmt {
                        if let ModuleDeclaration::ImportDeclaration(_) = &**decl {
                            self.stats.import_statements += 1;
                        } else {
                            self.stats.export_statements += 1;
                        }

                        continue;
                    }

                    if let Statement::ExpressionStatement(expr) = &stmt {
                        if let Expression::AwaitExpression(aw) = &expr.expression {
                            if let Expression::ImportExpression(_) = &aw.argument {
                                // Handle in other methods
                                continue;
                            }
                        }
                    }

                    self.stats.other_statements += 1;
                }
            }
            // export =
            AstKind::ModuleDeclaration(ModuleDeclaration::TSExportAssignment(export)) => {
                self.module.exports.push(Export {
                    kind: ExportKind::Modern,
                    span: Some(export.span),
                    symbols: vec![ExportedSymbol {
                        kind: ExportedKind::Default,
                        symbol_id: None,
                        name: Atom::from("default"),
                    }],
                    ..Export::default()
                });
            }
            _ => {}
        };
    }

    // require()
    fn visit_call_expression(&mut self, require: &CallExpression<'ast>) {
        if require.callee.is_specific_id("require") && require.arguments.len() == 1 {
            if let Argument::Expression(Expression::StringLiteral(source)) = &require.arguments[0] {
                if !self.extracted_requires.contains(&require.span) {
                    self.extracted_requires.insert(require.span);

                    self.module.imports.push(Import {
                        kind: ImportKind::SyncStatic,
                        module_id: 0,
                        source_request: source.value.clone(),
                        span: require.span,
                        type_only: false,
                        symbols: vec![],
                    });
                    self.stats.require_count += 1;
                }
            };
        }
    }

    // export *
    // export * as name
    // export type *
    // export type * as name
    fn visit_export_all_declaration(&mut self, export: &ExportAllDeclaration<'ast>) {
        let mut record = Export {
            kind: ExportKind::Modern,
            span: Some(export.span),
            source: Some(export.source.value.clone()),
            type_only: export.export_kind.is_type(),
            ..Export::default()
        };

        let kind = if export.export_kind.is_type() {
            ExportedKind::NamespaceType
        } else {
            ExportedKind::Namespace
        };

        if let Some(namespace) = &export.exported {
            record.symbols.push(ExportedSymbol {
                kind,
                symbol_id: None,
                name: namespace.name().to_owned(),
            });
        } else {
            record.symbols.push(ExportedSymbol {
                kind,
                symbol_id: None,
                name: Atom::from("*"),
            });
        }

        self.module.exports.push(record);
    }

    // export default
    fn visit_export_default_declaration(&mut self, export: &ExportDefaultDeclaration<'ast>) {
        let mut record = Export {
            kind: ExportKind::Modern,
            span: Some(export.span),
            type_only: export.declaration.is_typescript_syntax(),
            ..Export::default()
        };

        let ident = match &export.declaration {
            ExportDefaultDeclarationKind::ClassDeclaration(decl) => decl.id.as_ref(),
            ExportDefaultDeclarationKind::FunctionDeclaration(decl) => decl.id.as_ref(),
            ExportDefaultDeclarationKind::Expression(Expression::Identifier(ident)) => {
                record.symbols.push(ExportedSymbol {
                    kind: ExportedKind::Default,
                    symbol_id: None,
                    name: ident.name.clone(),
                });

                None
            }
            // This doesn't work...
            ExportDefaultDeclarationKind::TSEnumDeclaration(decl) => Some(&decl.id),
            ExportDefaultDeclarationKind::TSInterfaceDeclaration(decl) => Some(&decl.id),
            _ => {
                return;
            }
        };

        if record.symbols.is_empty() {
            if let Some(ident) = ident {
                record.symbols.push(ExportedSymbol {
                    kind: if export.declaration.is_typescript_syntax() {
                        ExportedKind::DefaultType
                    } else {
                        ExportedKind::Default
                    },
                    symbol_id: ident.symbol_id.clone().into_inner(),
                    name: ident.name.clone(),
                });
            } else {
                record.symbols.push(ExportedSymbol {
                    kind: ExportedKind::Default,
                    symbol_id: None,
                    name: Atom::from("default"),
                });
            }
        }

        self.module.exports.push(record);
        self.stats.exports_default = true;
    }

    // export { name }
    // export { type name }
    // export type { name }
    // export const name
    // export let name
    // export type name
    fn visit_export_named_declaration(&mut self, export: &ExportNamedDeclaration<'ast>) {
        let mut record = Export {
            kind: ExportKind::Modern,
            span: Some(export.span),
            type_only: export.is_typescript_syntax(),
            ..Export::default()
        };

        if let Some(decl) = &export.declaration {
            match decl {
                Declaration::VariableDeclaration(vars) => {
                    for var in &vars.declarations {
                        export_binding_pattern(&var.id, &mut record.symbols);
                    }
                }
                Declaration::FunctionDeclaration(d) => {
                    let id = d.id.as_ref().unwrap();

                    record.symbols.push(ExportedSymbol {
                        kind: ExportedKind::Value,
                        symbol_id: id.symbol_id.clone().into_inner(),
                        name: id.name.clone(),
                    });
                }
                Declaration::ClassDeclaration(d) => {
                    let id = d.id.as_ref().unwrap();

                    record.symbols.push(ExportedSymbol {
                        kind: ExportedKind::Value,
                        symbol_id: id.symbol_id.clone().into_inner(),
                        name: id.name.clone(),
                    });
                }
                Declaration::TSTypeAliasDeclaration(d) => {
                    record.symbols.push(ExportedSymbol {
                        kind: ExportedKind::ValueType,
                        symbol_id: d.id.symbol_id.clone().into_inner(),
                        name: d.id.name.clone(),
                    });
                }
                Declaration::TSInterfaceDeclaration(d) => {
                    record.symbols.push(ExportedSymbol {
                        kind: ExportedKind::ValueType,
                        symbol_id: d.id.symbol_id.clone().into_inner(),
                        name: d.id.name.clone(),
                    });
                }
                Declaration::TSEnumDeclaration(d) => {
                    record.symbols.push(ExportedSymbol {
                        kind: ExportedKind::ValueType,
                        symbol_id: d.id.symbol_id.clone().into_inner(),
                        name: d.id.name.clone(),
                    });
                }
                Declaration::TSModuleDeclaration(d) => {
                    record.symbols.push(ExportedSymbol {
                        kind: ExportedKind::ValueType,
                        symbol_id: None,
                        name: d.id.name().to_owned(),
                    });
                }
                Declaration::TSImportEqualsDeclaration(d) => {
                    record.symbols.push(ExportedSymbol {
                        kind: ExportedKind::ValueType,
                        symbol_id: d.id.symbol_id.clone().into_inner(),
                        name: d.id.name.clone(),
                    });
                }
                _ => {}
            };
        }

        for specifier in &export.specifiers {
            record.symbols.push(ExportedSymbol {
                kind: if export.export_kind.is_type() || specifier.export_kind.is_type() {
                    ExportedKind::ValueType
                } else {
                    ExportedKind::Value
                },
                symbol_id: None, // Is this correct?
                name: specifier.local.name().to_owned(),
            });
        }

        if let Some(source) = &export.source {
            record.source = Some(source.value.to_owned());
        }

        if !record.symbols.is_empty() {
            self.module.exports.push(record);
        }
    }

    // import
    // import default
    // import type default
    // import { name, type T }
    // import type { T }
    // import * as ns
    // import type * as ns
    fn visit_import_declaration(&mut self, import: &ImportDeclaration<'ast>) {
        let mut record = Import {
            kind: ImportKind::AsyncStatic,
            module_id: 0,
            source_request: import.source.value.clone(),
            span: import.span,
            type_only: import.import_kind.is_type(),
            symbols: vec![],
        };

        if let Some(specifiers) = &import.specifiers {
            for specifier in specifiers {
                match specifier {
                    ImportDeclarationSpecifier::ImportSpecifier(spec) => {
                        let mut value = ImportedSymbol::from_binding(
                            if import.import_kind.is_type() || spec.import_kind.is_type() {
                                ImportedKind::ValueType
                            } else {
                                ImportedKind::Value
                            },
                            &spec.local,
                        );

                        value.source_name = Some(spec.imported.name().to_owned());

                        record.symbols.push(value);
                    }
                    ImportDeclarationSpecifier::ImportDefaultSpecifier(spec) => {
                        record.symbols.push(ImportedSymbol::from_binding(
                            if import.import_kind.is_type() {
                                ImportedKind::DefaultType
                            } else {
                                ImportedKind::Default
                            },
                            &spec.local,
                        ));
                    }
                    ImportDeclarationSpecifier::ImportNamespaceSpecifier(spec) => {
                        record.symbols.push(ImportedSymbol::from_binding(
                            if import.import_kind.is_type() {
                                ImportedKind::NamespaceType
                            } else {
                                ImportedKind::Namespace
                            },
                            &spec.local,
                        ));
                    }
                };
            }
        }

        if !record.symbols.is_empty() {
            self.module.imports.push(record);
        }
    }

    // import()
    fn visit_import_expression(&mut self, import: &ImportExpression<'ast>) {
        if let Expression::StringLiteral(source) = &import.source {
            if !self.extracted_dynamic_imports.contains(&import.span) {
                self.extracted_dynamic_imports.insert(import.span);

                self.module.imports.push(Import {
                    kind: ImportKind::AsyncDynamic,
                    module_id: 0,
                    source_request: source.value.clone(),
                    span: import.span,
                    type_only: false,
                    symbols: vec![],
                });
                self.stats.dynamic_import_count += 1;
            }
        };
    }

    // exports.name
    // module.exports
    fn visit_static_member_expression(&mut self, expr: &StaticMemberExpression<'ast>) {
        let mut record = Export {
            kind: ExportKind::Legacy,
            span: Some(expr.span),
            ..Export::default()
        };

        // named
        if expr.object.is_specific_id("exports") && !expr.property.name.is_empty() {
            record.symbols.push(ExportedSymbol {
                kind: ExportedKind::Value,
                symbol_id: None,
                name: expr.property.name.clone(),
            });
        }
        // default
        else if expr.object.is_specific_id("module") && expr.property.name == "exports" {
            record.symbols.push(ExportedSymbol {
                kind: ExportedKind::Default,
                symbol_id: None,
                name: Atom::from("default"),
            });
        }

        if !record.symbols.is_empty() {
            self.module.exports.push(record);
        }
    }

    // import foo =
    fn visit_ts_import_equals_declaration(&mut self, decl: &TSImportEqualsDeclaration<'ast>) {
        if let TSModuleReference::ExternalModuleReference(ext_module) = &*decl.module_reference {
            self.module.imports.push(Import {
                kind: ImportKind::SyncStatic,
                module_id: 0,
                source_request: ext_module.expression.value.clone(),
                span: decl.span,
                symbols: vec![ImportedSymbol {
                    kind: ImportedKind::Default,
                    source_name: None,
                    symbol_id: decl.id.symbol_id.clone().into_inner(),
                    name: decl.id.name.clone(),
                }],
                type_only: decl.import_kind.is_type(),
            });
        }
    }

    // { .. } = await import()
    // { .. } = require()
    fn visit_variable_declarator(&mut self, decl: &VariableDeclarator<'ast>) {
        let Some(init) = &decl.init else {
            return;
        };

        // import()
        if let Some(import) = extract_dynamic_import_from_expression(init) {
            if let Expression::StringLiteral(source) = &import.source {
                if !self.extracted_dynamic_imports.contains(&import.span) {
                    self.extracted_dynamic_imports.insert(import.span);

                    let mut record = Import {
                        kind: ImportKind::AsyncDynamic,
                        module_id: 0,
                        source_request: source.value.clone(),
                        span: import.span,
                        type_only: false,
                        symbols: vec![],
                    };

                    import_binding_pattern(&decl.id, &mut record.symbols);

                    self.module.imports.push(record);
                    self.stats.dynamic_import_count += 1;
                }
            };
        }

        // require()
        // if let Some(require) = extract_require_from_expression(init) {
        //     if let Argument::Expression(Expression::StringLiteral(source)) = &require.arguments[0] {
        //         if !self.extracted_requires.contains(&require.span) {
        //             self.extracted_requires.insert(require.span);

        //             let mut record = Import {
        //                 kind: ImportKind::SyncStatic,
        //                 module_id: 0,
        //                 source_request: source.value.clone(),
        //                 span: require.span,
        //                 type_only: false,
        //                 symbols: vec![],
        //             };

        //             import_binding_pattern(&decl.id, &mut record.symbols, 0);

        //             self.module.imports.push(record);
        //             self.stats.require_count += 1;
        //         }
        //     };
        // }
    }
}

fn extract_require_from_expression<'expr, 'ast>(
    expr: &'expr Expression<'ast>,
) -> Option<&'expr CallExpression<'ast>> {
    if let Expression::CallExpression(outer) = expr {
        if outer.callee.is_specific_id("require") && outer.arguments.len() == 1 {
            return Some(outer);
        }
    }

    None
}

fn extract_dynamic_import_from_expression<'expr, 'ast>(
    expr: &'expr Expression<'ast>,
) -> Option<&'expr ImportExpression<'ast>> {
    if let Expression::AwaitExpression(outer) = expr {
        if let Expression::ImportExpression(inner) = &outer.argument {
            return Some(inner);
        }
    }

    None
}

fn import_binding_pattern(binding: &BindingPattern, list: &mut Vec<ImportedSymbol>) {
    match &binding.kind {
        // foo = import()
        BindingPatternKind::BindingIdentifier(ident) => {
            list.push(ImportedSymbol {
                kind: ImportedKind::Namespace,
                source_name: None,
                symbol_id: ident.symbol_id.clone().into_inner(),
                name: ident.name.clone(),
            });
        }

        // { a, b, ...rest } = import()
        BindingPatternKind::ObjectPattern(object) => {
            for prop in &object.properties {
                let kind = if prop.key.is_specific_id("default") {
                    ImportedKind::Default
                } else {
                    ImportedKind::Value
                };

                match &prop.value.kind {
                    BindingPatternKind::BindingIdentifier(ident) => {
                        list.push(ImportedSymbol {
                            kind,
                            source_name: if prop.key.is_specific_id(&ident.name) {
                                None
                            } else {
                                prop.key.name()
                            },
                            symbol_id: ident.symbol_id.clone().into_inner(),
                            name: ident.name.clone(),
                        });
                    }
                    _ => {
                        list.push(ImportedSymbol {
                            kind,
                            source_name: None,
                            symbol_id: None,
                            name: prop.key.name().clone().unwrap(),
                        });
                    }
                };
            }

            if let Some(rest) = &object.rest {
                if let BindingPatternKind::BindingIdentifier(ident) = &rest.argument.kind {
                    list.push(ImportedSymbol {
                        kind: ImportedKind::Namespace,
                        source_name: None,
                        symbol_id: None,
                        name: ident.name.clone(),
                    });
                }
            }
        }

        // [a, b] = import()
        BindingPatternKind::ArrayPattern(_) => {
            // Not possible
        }

        // { a = 1 } = import()
        BindingPatternKind::AssignmentPattern(assign) => {
            import_binding_pattern(&assign.left, list);
        }
    };
}

fn export_binding_pattern(binding: &BindingPattern, list: &mut Vec<ExportedSymbol>) {
    match &binding.kind {
        BindingPatternKind::BindingIdentifier(ident) => {
            list.push(ExportedSymbol {
                kind: ExportedKind::Value,
                symbol_id: ident.symbol_id.clone().into_inner(),
                name: ident.name.clone(),
            });
        }
        BindingPatternKind::ObjectPattern(object) => {
            for prop in &object.properties {
                export_binding_pattern(&prop.value, list);
            }

            if let Some(rest) = &object.rest {
                export_binding_pattern(&rest.argument, list);
            }
        }
        BindingPatternKind::ArrayPattern(array) => {
            for item in array.elements.iter().flatten() {
                export_binding_pattern(item, list);
            }

            if let Some(rest) = &array.rest {
                export_binding_pattern(&rest.argument, list);
            }
        }
        BindingPatternKind::AssignmentPattern(assign) => {
            export_binding_pattern(&assign.left, list);
        }
    };
}
