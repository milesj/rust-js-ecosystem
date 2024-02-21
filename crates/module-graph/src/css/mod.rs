use crate::module::*;
use crate::module_graph_error::ModuleGraphError;
use lightningcss::css_modules::{Config, CssModuleReference};
use lightningcss::printer::PrinterOptions;
use lightningcss::stylesheet::{ParserOptions, StyleSheet};
use mediatype::names::TEXT;
use mediatype::{MediaTypeBuf, Name};
use oxc::span::{Atom, Span};
use oxc::syntax::symbol::SymbolId;
use starbase_utils::fs;
use std::sync::Arc;

#[derive(Debug)]
pub struct CssModule {
    pub mime_type: MediaTypeBuf,
    pub source: Arc<String>,
}

impl SourceParser for CssModule {
    fn parse_into_module(module: &mut Module) -> Result<Source, ModuleGraphError> {
        let source = fs::read_file(&module.path)?;
        let filename = fs::file_name(&module.path);

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

            // Extract imports
            if let Some(imports) = css.references {
                for import in imports.values() {
                    if let CssModuleReference::Dependency { name, specifier } = import {
                        module.imports.push(Import {
                            kind: ImportKind::SyncStatic,
                            module_id: 0,
                            source: Atom::from(specifier.as_str()),
                            span: Span::default(),
                            symbols: vec![ImportedSymbol {
                                kind: ImportedKind::Value,
                                source_name: None,
                                symbol_id: None,
                                name: Atom::from(name.as_str()),
                            }],
                            type_only: false,
                        });
                    }
                }
            }

            // Extract exports
            if let Some(exports) = css.exports {
                for source_name in exports.keys() {
                    module.exports.push(Export {
                        kind: ExportKind::Native,
                        symbols: vec![ExportedSymbol {
                            kind: ExportedKind::Value,
                            symbol_id: create_symbol(),
                            name: Atom::from(source_name.as_str()),
                        }],
                        ..Export::default()
                    });
                }
            }
        }

        Ok(Source::Css(Box::new(CssModule {
            mime_type: MediaTypeBuf::new(TEXT, Name::new("css").unwrap()),
            source: Arc::new(source),
        })))
    }
}
