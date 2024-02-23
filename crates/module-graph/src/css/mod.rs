use crate::module::*;
use crate::module_graph_error::ModuleGraphError;
use crate::types::FxIndexMap;
use lightningcss::css_modules::{Config, CssModuleReference};
use lightningcss::printer::PrinterOptions;
use lightningcss::stylesheet::{ParserOptions, StyleSheet};
use oxc::span::{Atom, Span};
use oxc_resolver::PackageJson as ResolvedPackageJson;
use starbase_utils::fs;
use std::collections::BTreeMap;
use std::fmt;
use std::mem;
use std::sync::Arc;

pub struct CssModule {
    pub exports: FxIndexMap<String, String>,
    pub sheet: Arc<StyleSheet<'static, 'static>>,
    pub source: Arc<String>,
}

impl fmt::Debug for CssModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CssModule")
            .field("exports", &self.exports)
            .field("source", &self.source)
            .finish()
    }
}

impl SourceParser for CssModule {
    fn parse_into_module(
        module: &mut Module,
        _package_json: Option<Arc<ResolvedPackageJson>>,
    ) -> Result<Source, ModuleGraphError> {
        let source = Arc::new(fs::read_file(&module.path)?);
        let filename = fs::file_name(&module.path);
        let is_module = filename.contains(".module.");
        let mut exports_hashes = BTreeMap::default();

        let sheet = StyleSheet::parse(
            &source,
            ParserOptions {
                filename,
                css_modules: is_module.then(|| Config::default()),
                ..ParserOptions::default()
            },
        )
        .unwrap();

        // Determine if this is a CSS module
        if is_module {
            let css = sheet.to_css(PrinterOptions::default()).unwrap();

            let mut map_module_import = |imports: Vec<CssModuleReference>| {
                for import in imports {
                    if let CssModuleReference::Dependency { name, specifier } = import {
                        let symbol = ImportedSymbol {
                            kind: ImportedKind::Value,
                            source_name: None,
                            symbol_id: None,
                            name: Atom::from(name.as_str()),
                        };

                        if let Some(existing_import) = module
                            .imports
                            .iter_mut()
                            .find(|i| i.source_request == specifier)
                        {
                            existing_import.symbols.push(symbol);
                            continue;
                        }

                        module.imports.push(Import {
                            kind: ImportKind::SyncStatic,
                            module_id: 0,
                            source_request: Atom::from(specifier.as_str()),
                            span: Span::default(),
                            symbols: vec![symbol],
                            type_only: false,
                        });
                    }
                }
            };

            // Extract imports
            if let Some(imports) = css.references {
                map_module_import(imports.into_values().collect());
            }

            // Extract exports
            if let Some(exports) = css.exports {
                for (source_name, export_info) in exports {
                    module.exports.push(Export {
                        kind: ExportKind::Native,
                        symbols: vec![ExportedSymbol {
                            kind: ExportedKind::Value,
                            symbol_id: None,
                            name: Atom::from(source_name.as_str()),
                        }],
                        ..Export::default()
                    });

                    map_module_import(export_info.composes);
                    exports_hashes.insert(source_name, export_info.name);
                }
            }
        }

        // TODO: this is for snapshots
        module
            .imports
            .sort_by(|a, d| a.source_request.cmp(&d.source_request));
        module
            .exports
            .sort_by(|a, d| a.symbols[0].name.cmp(&d.symbols[0].name));

        Ok(Source::Css(Box::new(CssModule {
            exports: FxIndexMap::from_iter(exports_hashes),
            sheet: Arc::new(unsafe {
                mem::transmute::<StyleSheet<'_, '_>, StyleSheet<'static, 'static>>(sheet)
            }),
            source,
        })))
    }
}
