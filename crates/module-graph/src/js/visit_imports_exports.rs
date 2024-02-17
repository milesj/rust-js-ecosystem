use crate::module::*;
use oxc::ast::ast::{
    Argument, BindingPattern, BindingPatternKind, CallExpression, Declaration,
    ExportAllDeclaration, ExportDefaultDeclaration, ExportDefaultDeclarationKind,
    ExportNamedDeclaration, Expression, ImportDeclaration, ImportDeclarationSpecifier,
    ImportExpression, StaticMemberExpression, VariableDeclarator,
};
use oxc::ast::Visit;
use oxc::span::{Atom, Span};
use rustc_hash::FxHashSet;
use std::cell::Cell;

pub struct ExtractImportsExports<'module> {
    pub module: &'module mut Module,
    pub extracted_dynamic_imports: FxHashSet<Span>,
    pub extracted_requires: FxHashSet<Span>,
}

// TODO non-literal paths
impl<'module> Visit<'module> for ExtractImportsExports<'module> {
    // require()
    fn visit_call_expression(&mut self, require: &CallExpression<'module>) {
        if require.callee.is_specific_id("require") && require.arguments.len() == 1 {
            if let Argument::Expression(Expression::StringLiteral(source)) = &require.arguments[0] {
                if !self.extracted_requires.contains(&require.span) {
                    self.extracted_requires.insert(require.span);

                    self.module.imports.push(Import {
                        kind: ImportKind::SyncStatic,
                        module_id: 0,
                        source: source.value.clone(),
                        span: require.span,
                        type_only: false,
                        symbols: vec![],
                    });
                }
            };
        }
    }

    // export *
    // export * as name
    // export type *
    // export type * as name
    fn visit_export_all_declaration(&mut self, export: &ExportAllDeclaration<'module>) {
        let mut record = Export {
            kind: ExportKind::Module,
            module_id: 0,
            source: None,
            span: export.span,
            symbols: vec![],
        };

        let kind = if export.export_kind.is_type() {
            ExportedValueKind::StarType
        } else {
            ExportedValueKind::Star
        };

        if let Some(namespace) = &export.exported {
            record.symbols.push(ExportedSymbol {
                kind,
                symbol_id: Cell::default(),
                name: namespace.name().to_owned(),
            });
        } else {
            record.symbols.push(ExportedSymbol {
                kind,
                symbol_id: Cell::default(),
                name: Atom::from(""),
            });
        }

        self.module.exports.push(record);
    }

    // export default
    fn visit_export_default_declaration(&mut self, export: &ExportDefaultDeclaration<'module>) {
        let mut record = Export {
            kind: ExportKind::Module,
            module_id: 0,
            source: None,
            span: export.span,
            symbols: vec![],
        };

        let ident = match &export.declaration {
            ExportDefaultDeclarationKind::ClassDeclaration(decl) => decl.id.as_ref(),
            ExportDefaultDeclarationKind::FunctionDeclaration(decl) => decl.id.as_ref(),
            ExportDefaultDeclarationKind::TSEnumDeclaration(decl) => Some(&decl.id),
            ExportDefaultDeclarationKind::TSInterfaceDeclaration(decl) => Some(&decl.id),
            _ => {
                return;
            }
        };

        if let Some(ident) = ident {
            record.symbols.push(ExportedSymbol {
                kind: if export.declaration.is_typescript_syntax() {
                    ExportedValueKind::DefaultType
                } else {
                    ExportedValueKind::Default
                },
                symbol_id: ident.symbol_id.clone(),
                name: ident.name.clone(),
            });
        } else {
            record.symbols.push(ExportedSymbol {
                kind: ExportedValueKind::Default,
                symbol_id: Cell::default(),
                name: Atom::from("default"),
            });
        }

        self.module.exports.push(record);
    }

    // export { name }
    // export { type name }
    // export const name
    // export let name
    // export type name
    fn visit_export_named_declaration(&mut self, export: &ExportNamedDeclaration<'module>) {
        let mut record = Export {
            kind: ExportKind::Module,
            module_id: 0,
            source: None,
            span: export.span,
            symbols: vec![],
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
                        kind: ExportedValueKind::Value,
                        symbol_id: id.symbol_id.clone(),
                        name: id.name.clone(),
                    });
                }
                Declaration::ClassDeclaration(d) => {
                    let id = d.id.as_ref().unwrap();

                    record.symbols.push(ExportedSymbol {
                        kind: ExportedValueKind::Value,
                        symbol_id: id.symbol_id.clone(),
                        name: id.name.clone(),
                    });
                }
                Declaration::TSTypeAliasDeclaration(d) => {
                    record.symbols.push(ExportedSymbol {
                        kind: ExportedValueKind::ValueType,
                        symbol_id: d.id.symbol_id.clone(),
                        name: d.id.name.clone(),
                    });
                }
                Declaration::TSInterfaceDeclaration(d) => {
                    record.symbols.push(ExportedSymbol {
                        kind: ExportedValueKind::ValueType,
                        symbol_id: d.id.symbol_id.clone(),
                        name: d.id.name.clone(),
                    });
                }
                Declaration::TSEnumDeclaration(d) => {
                    record.symbols.push(ExportedSymbol {
                        kind: ExportedValueKind::ValueType,
                        symbol_id: d.id.symbol_id.clone(),
                        name: d.id.name.clone(),
                    });
                }
                Declaration::TSModuleDeclaration(d) => {
                    record.symbols.push(ExportedSymbol {
                        kind: ExportedValueKind::ValueType,
                        symbol_id: Cell::default(),
                        name: d.id.name().to_owned(),
                    });
                }
                Declaration::TSImportEqualsDeclaration(d) => {
                    record.symbols.push(ExportedSymbol {
                        kind: ExportedValueKind::ValueType,
                        symbol_id: d.id.symbol_id.clone(),
                        name: d.id.name.clone(),
                    });
                }
                _ => {}
            };
        }

        for specifier in &export.specifiers {
            record.symbols.push(ExportedSymbol {
                kind: if export.export_kind.is_type() || specifier.export_kind.is_type() {
                    ExportedValueKind::ValueType
                } else {
                    ExportedValueKind::Value
                },
                symbol_id: Cell::default(), // Is this correct?
                name: specifier.local.name().to_owned(),
            });
        }

        if let Some(source) = &export.source {
            record.source = Some(source.value.to_owned());
        }

        self.module.exports.push(record);
    }

    // import
    // import default
    // import type default
    // import { name, type T }
    // import type { T }
    // import * as ns
    // import type * as ns
    fn visit_import_declaration(&mut self, import: &ImportDeclaration<'module>) {
        let mut record = Import {
            kind: ImportKind::AsyncStatic,
            module_id: 0,
            source: import.source.value.clone(),
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
                                ImportedValueKind::ValueType
                            } else {
                                ImportedValueKind::Value
                            },
                            &spec.local,
                        );

                        value.source_name = Some(spec.imported.name().to_owned());

                        record.symbols.push(value);
                    }
                    ImportDeclarationSpecifier::ImportDefaultSpecifier(spec) => {
                        record.symbols.push(ImportedSymbol::from_binding(
                            if import.import_kind.is_type() {
                                ImportedValueKind::DefaultType
                            } else {
                                ImportedValueKind::Default
                            },
                            &spec.local,
                        ));
                    }
                    ImportDeclarationSpecifier::ImportNamespaceSpecifier(spec) => {
                        record.symbols.push(ImportedSymbol::from_binding(
                            if import.import_kind.is_type() {
                                ImportedValueKind::StarType
                            } else {
                                ImportedValueKind::Star
                            },
                            &spec.local,
                        ));
                    }
                };
            }
        }

        self.module.imports.push(record);
    }

    // import()
    fn visit_import_expression(&mut self, import: &ImportExpression<'module>) {
        if let Expression::StringLiteral(source) = &import.source {
            if !self.extracted_dynamic_imports.contains(&import.span) {
                self.extracted_dynamic_imports.insert(import.span);

                self.module.imports.push(Import {
                    kind: ImportKind::AsyncDynamic,
                    module_id: 0,
                    source: source.value.clone(),
                    span: import.span,
                    type_only: false,
                    symbols: vec![],
                });
            }
        };
    }

    // exports.name
    // module.exports
    fn visit_static_member_expression(&mut self, expr: &StaticMemberExpression<'module>) {
        let mut record = Export {
            kind: ExportKind::Legacy,
            module_id: 0,
            source: None,
            span: expr.span,
            symbols: vec![],
        };

        // named
        if expr.object.is_specific_id("exports") && !expr.property.name.is_empty() {
            record.symbols.push(ExportedSymbol {
                kind: ExportedValueKind::Value,
                symbol_id: Cell::default(),
                name: expr.property.name.clone(),
            });
        }
        // default
        else if expr.object.is_specific_id("module") && expr.property.name == "exports" {
            record.symbols.push(ExportedSymbol {
                kind: ExportedValueKind::Default,
                symbol_id: Cell::default(),
                name: Atom::from("default"),
            });
        }

        if !record.symbols.is_empty() {
            self.module.exports.push(record);
        }
    }

    // { .. } = await import()
    // { .. } = require()
    fn visit_variable_declarator(&mut self, decl: &VariableDeclarator<'module>) {
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
                        source: source.value.clone(),
                        span: import.span,
                        type_only: false,
                        symbols: vec![],
                    };

                    import_binding_pattern(&decl.id, &mut record.symbols, 0);

                    self.module.imports.push(record);
                }
            };
        }

        // require()
        if let Some(require) = extract_require_from_expression(init) {
            if let Argument::Expression(Expression::StringLiteral(source)) = &require.arguments[0] {
                if !self.extracted_requires.contains(&require.span) {
                    self.extracted_requires.insert(require.span);

                    let mut record = Import {
                        kind: ImportKind::SyncStatic,
                        module_id: 0,
                        source: source.value.clone(),
                        span: require.span,
                        type_only: false,
                        symbols: vec![],
                    };

                    import_binding_pattern(&decl.id, &mut record.symbols, 0);

                    self.module.imports.push(record);
                }
            };
        }
    }
}

fn extract_require_from_expression<'expr, 'module>(
    expr: &'expr Expression<'module>,
) -> Option<&'expr CallExpression<'module>> {
    if let Expression::CallExpression(outer) = expr {
        if outer.callee.is_specific_id("require") && outer.arguments.len() == 1 {
            return Some(&outer);
        }
    }

    None
}

fn extract_dynamic_import_from_expression<'expr, 'module>(
    expr: &'expr Expression<'module>,
) -> Option<&'expr ImportExpression<'module>> {
    if let Expression::AwaitExpression(outer) = expr {
        if let Expression::ImportExpression(inner) = &outer.argument {
            return Some(&inner);
        }
    }

    None
}

fn import_binding_pattern(binding: &BindingPattern, list: &mut Vec<ImportedSymbol>, depth: usize) {
    match &binding.kind {
        // foo = import()
        BindingPatternKind::BindingIdentifier(ident) => {
            list.push(ImportedSymbol {
                kind: if depth == 0 {
                    ImportedValueKind::Star
                } else {
                    ImportedValueKind::Value
                },
                source_name: None,
                symbol_id: ident.symbol_id.clone(),
                name: ident.name.clone(),
            });
        }

        // { a, b, ...rest } = import()
        BindingPatternKind::ObjectPattern(object) => {
            for prop in &object.properties {
                if let BindingPatternKind::BindingIdentifier(ident) = &prop.value.kind {
                    let source_name = if prop.key.is_specific_id(&ident.name) {
                        None
                    } else {
                        prop.key.name()
                    };

                    list.push(ImportedSymbol {
                        kind: if depth == 0 && prop.key.is_specific_id("default") {
                            ImportedValueKind::Default
                        } else {
                            ImportedValueKind::Value
                        },
                        source_name,
                        symbol_id: ident.symbol_id.clone(),
                        name: ident.name.clone(),
                    });
                } else {
                    import_binding_pattern(&prop.value, list, depth + 1);
                }
            }

            if let Some(rest) = &object.rest {
                import_binding_pattern(&rest.argument, list, depth);
            }
        }

        // [a, b] = import()
        BindingPatternKind::ArrayPattern(array) => {
            for item in array.elements.iter().flatten() {
                import_binding_pattern(item, list, depth + 1);
            }

            if let Some(rest) = &array.rest {
                import_binding_pattern(&rest.argument, list, depth);
            }
        }

        // { a = 1 } = import()
        BindingPatternKind::AssignmentPattern(assign) => {
            import_binding_pattern(&assign.left, list, depth);
        }
    };
}

fn export_binding_pattern(binding: &BindingPattern, list: &mut Vec<ExportedSymbol>) {
    match &binding.kind {
        BindingPatternKind::BindingIdentifier(ident) => {
            list.push(ExportedSymbol {
                kind: ExportedValueKind::Value,
                symbol_id: ident.symbol_id.clone(),
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