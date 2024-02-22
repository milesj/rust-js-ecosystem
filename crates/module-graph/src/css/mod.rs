use crate::module::*;
use crate::module_graph_error::ModuleGraphError;
use lightningcss::css_modules::{Config, CssModuleReference};
use lightningcss::printer::PrinterOptions;
use lightningcss::stylesheet::{ParserOptions, StyleSheet};
use oxc::span::{Atom, Span};
use oxc::syntax::symbol::SymbolId;
use oxc_resolver::PackageJson as ResolvedPackageJson;
use rustc_hash::FxHashMap;
use starbase_utils::fs;
use std::sync::Arc;

#[derive(Debug)]
pub struct CssModule {
    pub exports: FxHashMap<String, String>,
    pub source: Arc<String>,
}

impl SourceParser for CssModule {
    fn parse_into_module(
        module: &mut Module,
        _package_json: Option<Arc<ResolvedPackageJson>>,
    ) -> Result<Source, ModuleGraphError> {
        let source = fs::read_file(&module.path)?;
        let filename = fs::file_name(&module.path);
        let mut exports_hashes = FxHashMap::default();

        let mut symbol_count: isize = -1;
        let mut create_symbol = || {
            symbol_count += 1;
            Some(SymbolId::new(symbol_count as usize))
        };

        // Determine if this is a CSS module
        if filename.contains(".module.") {
            let options = ParserOptions {
                filename,
                css_modules: Some(Config::default()),
                ..ParserOptions::default()
            };

            // The only way to retrive CSS modules info is to stringify it...
            let css = StyleSheet::parse(&source, options)
                .unwrap()
                .to_css(PrinterOptions::default())
                .unwrap();

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
                            symbol_id: create_symbol(),
                            name: Atom::from(source_name.as_str()),
                        }],
                        ..Export::default()
                    });

                    map_module_import(export_info.composes);
                    exports_hashes.insert(source_name, export_info.name);
                }
            }
        }

        Ok(Source::Css(Box::new(CssModule {
            exports: exports_hashes,
            source: Arc::new(source),
        })))
    }
}
