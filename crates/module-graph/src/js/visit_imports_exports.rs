use crate::module::*;
use oxc::ast::ast::{
    Argument, BindingPatternKind, CallExpression, Expression, ImportDeclaration,
    ImportDeclarationSpecifier, ImportExpression, VariableDeclarator,
};
use oxc::ast::Visit;
use oxc::span::Span;
use rustc_hash::FxHashSet;

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
                        source: source.value.clone(),
                        span: require.span,
                        values: vec![],
                    });
                }
            };
        }
    }

    // import {} ...
    fn visit_import_declaration(&mut self, import: &ImportDeclaration<'module>) {
        let mut record = Import {
            kind: ImportKind::AsyncStatic,
            source: import.source.value.clone(),
            span: import.span,
            values: vec![],
        };

        if let Some(specifiers) = &import.specifiers {
            for specifier in specifiers {
                match specifier {
                    ImportDeclarationSpecifier::ImportSpecifier(spec) => {
                        record.values.push(ImportedValue {
                            kind: if spec.import_kind.is_type() || import.import_kind.is_type() {
                                ImportedValueKind::Type
                            } else {
                                ImportedValueKind::Value
                            },
                            source_name: Some(spec.imported.name().to_owned()),
                            name: spec.local.name.clone(),
                        });
                    }
                    ImportDeclarationSpecifier::ImportDefaultSpecifier(spec) => {
                        record.values.push(ImportedValue {
                            kind: ImportedValueKind::Default,
                            source_name: None,
                            name: spec.local.name.clone(),
                        });
                    }
                    ImportDeclarationSpecifier::ImportNamespaceSpecifier(spec) => {
                        record.values.push(ImportedValue {
                            kind: ImportedValueKind::Star,
                            source_name: None,
                            name: spec.local.name.clone(),
                        });
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
                    source: source.value.clone(),
                    span: import.span,
                    values: vec![],
                });
            }
        };
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

                    self.module.imports.push(Import {
                        kind: ImportKind::AsyncDynamic,
                        source: source.value.clone(),
                        span: import.span,
                        values: extract_imported_from_variable_declarator(decl),
                    });
                }
            };
        }

        // require()
        if let Some(require) = extract_require_from_expression(init) {
            if let Argument::Expression(Expression::StringLiteral(source)) = &require.arguments[0] {
                if !self.extracted_requires.contains(&require.span) {
                    self.extracted_requires.insert(require.span);

                    self.module.imports.push(Import {
                        kind: ImportKind::SyncStatic,
                        source: source.value.clone(),
                        span: require.span,
                        values: extract_imported_from_variable_declarator(decl),
                    });
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

fn extract_imported_from_variable_declarator<'expr, 'module>(
    decl: &'expr VariableDeclarator<'module>,
) -> Vec<ImportedValue> {
    let mut imports = vec![];

    match &decl.id.kind {
        // foo = import()
        BindingPatternKind::BindingIdentifier(ident) => {
            imports.push(ImportedValue {
                kind: ImportedValueKind::Star,
                source_name: None,
                name: ident.name.clone(),
            });
        }

        // { a, b, ...rest } = import()
        BindingPatternKind::ObjectPattern(pattern) => {
            for property in &pattern.properties {
                if let BindingPatternKind::BindingIdentifier(ident) = &property.value.kind {
                    let source_name = if property.key.is_specific_id(&ident.name) {
                        None
                    } else {
                        property.key.name()
                    };

                    imports.push(ImportedValue {
                        kind: if property.key.is_specific_id("default") {
                            ImportedValueKind::Default
                        } else {
                            ImportedValueKind::Value
                        },
                        source_name,
                        name: ident.name.clone(),
                    });
                }
            }

            if let Some(rest) = &pattern.rest {
                if let BindingPatternKind::BindingIdentifier(ident) = &rest.argument.kind {
                    imports.push(ImportedValue {
                        kind: ImportedValueKind::Star,
                        source_name: None,
                        name: ident.name.clone(),
                    });
                }
            }
        }
        _ => {}
    };

    imports
}
