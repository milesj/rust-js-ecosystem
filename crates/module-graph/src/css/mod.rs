use crate::module::*;
use crate::module_graph_error::ModuleGraphError;
use crate::types::FxIndexMap;
use lightningcss::css_modules::{Config, CssModuleReference};
use lightningcss::error::ParserError;
use lightningcss::printer::PrinterOptions;
use lightningcss::stylesheet::{ParserOptions, StyleSheet};
use once_cell::sync::OnceCell;
use oxc::span::{Atom, Span};
use oxc_resolver::PackageJson as ResolvedPackageJson;
use starbase_utils::fs;
use std::collections::BTreeMap;
use std::fmt;
use std::mem;
use std::sync::Arc;

pub struct CssModule {
    pub exports: FxIndexMap<String, String>,
    pub file_name: String,
    pub sheet: OnceCell<StyleSheet<'static, 'static>>,
    pub source: Arc<String>,
}

impl CssModule {
    fn is_module(&self) -> bool {
        self.file_name.ends_with(".module.css")
    }

    fn get_style_sheet(
        &self,
    ) -> Result<&StyleSheet<'static, 'static>, lightningcss::error::Error<ParserError>> {
        let source = Arc::clone(&self.source);
        let filename = self.file_name.clone();
        let is_module = self.is_module();

        self.sheet.get_or_try_init(move || {
            let sheet = StyleSheet::parse(
                &source,
                ParserOptions {
                    filename,
                    css_modules: is_module.then(Config::default),
                    ..ParserOptions::default()
                },
            )
            .unwrap(); // TODO

            Ok(
                unsafe {
                    mem::transmute::<StyleSheet<'_, '_>, StyleSheet<'static, 'static>>(sheet)
                },
            )
        })
    }
}

impl ModuleSource for CssModule {
    fn kind(&self) -> SourceKind {
        SourceKind::Css
    }

    fn source(&self) -> &[u8] {
        self.source.as_bytes()
    }

    fn load(
        module: &mut Module,
        _package_json: Option<Arc<ResolvedPackageJson>>,
    ) -> Result<Self, ModuleGraphError> {
        Ok(CssModule {
            exports: FxIndexMap::default(),
            file_name: fs::file_name(&module.path),
            sheet: OnceCell::new(),
            source: Arc::new(fs::read_file(&module.path)?),
        })
    }

    fn parse(&mut self, module: &mut Module) -> Result<(), ModuleGraphError> {
        if !self.is_module() {
            return Ok(());
        }

        let mut exports_hashes = BTreeMap::default();
        let css = self
            .get_style_sheet()
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
                        symbol_id: None,
                        name: Atom::from(source_name.as_str()),
                    }],
                    ..Export::default()
                });

                map_module_import(export_info.composes);
                exports_hashes.insert(source_name, export_info.name);
            }
        }

        // TODO: this is for snapshots
        module
            .imports
            .sort_by(|a, d| a.source_request.cmp(&d.source_request));
        module
            .exports
            .sort_by(|a, d| a.symbols[0].name.cmp(&d.symbols[0].name));

        self.exports = FxIndexMap::from_iter(exports_hashes);

        Ok(())
    }
}

impl fmt::Debug for CssModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CssModule")
            .field("exports", &self.exports)
            .field("file_name", &self.file_name)
            .field("source", &self.source)
            .finish()
    }
}
