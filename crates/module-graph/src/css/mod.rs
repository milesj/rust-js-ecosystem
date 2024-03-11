mod css_error;

use crate::atom::AtomStr;
use crate::module::*;
use crate::module_graph_error::ModuleGraphError;
use crate::types::FxIndexMap;
use lightningcss::css_modules::{Config, CssModuleReference};
use lightningcss::printer::PrinterOptions;
use lightningcss::stylesheet::{ParserOptions, StyleSheet};
use nodejs_package_json::PackageJson;
use oxc::span::Span;
use starbase_utils::fs;
use std::collections::BTreeMap;
use std::fmt;
use std::mem;
use std::sync::Arc;

pub use css_error::CssModuleError;

pub struct CssModule {
    pub exports: FxIndexMap<String, String>,
    pub module: bool,
    pub sheet: StyleSheet<'static, 'static>,
    pub source: Arc<String>,
}

impl CssModule {
    pub fn is_css_module(&self) -> bool {
        self.module
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
        _package_json: Option<Arc<PackageJson>>,
    ) -> Result<Self, ModuleGraphError> {
        let file_name = fs::file_name(&module.path);
        let css_module = file_name.ends_with(".module.css");
        let source = Arc::new(fs::read_file(&module.path)?);

        let sheet = StyleSheet::parse(
            &source,
            ParserOptions {
                filename: file_name,
                css_modules: css_module.then(Config::default),
                ..ParserOptions::default()
            },
        )
        .map_err(|error| CssModuleError::ParseFailed(error.into_owned()))?;

        Ok(CssModule {
            exports: FxIndexMap::default(),
            module: css_module,
            sheet: unsafe {
                mem::transmute::<StyleSheet<'_, '_>, StyleSheet<'static, 'static>>(sheet)
            },
            source,
        })
    }

    fn parse(&mut self, module: &mut Module) -> Result<(), ModuleGraphError> {
        if !self.is_css_module() {
            return Ok(());
        }

        let mut exports_hashes = BTreeMap::default();
        let css = self
            .sheet
            .to_css(PrinterOptions::default())
            .map_err(CssModuleError::ParseModuleFailed)?;

        let mut map_module_import = |imports: Vec<CssModuleReference>| {
            for import in imports {
                if let CssModuleReference::Dependency { name, specifier } = import {
                    let symbol = ImportedSymbol {
                        kind: ImportedKind::Value,
                        source_name: None,
                        symbol_id: None,
                        name: AtomStr::from(name.as_str()),
                    };

                    if let Some(existing_import) = module
                        .imports
                        .iter_mut()
                        .find(|i| i.source_request.as_str() == specifier)
                    {
                        existing_import.symbols.push(symbol);
                        continue;
                    }

                    module.imports.push(Import {
                        kind: ImportKind::SyncStatic,
                        module_id: 0,
                        source_request: AtomStr::from(specifier.as_str()),
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
                        name: AtomStr::from(source_name.as_str()),
                    }],
                    ..Export::default()
                });

                map_module_import(export_info.composes);
                exports_hashes.insert(source_name, export_info.name);
            }
        }

        // Snapshots...
        #[cfg(debug_assertions)]
        {
            module
                .imports
                .sort_by(|a, d| a.source_request.cmp(&d.source_request));
            module
                .exports
                .sort_by(|a, d| a.symbols[0].name.cmp(&d.symbols[0].name));
        }

        self.exports = FxIndexMap::from_iter(exports_hashes);

        Ok(())
    }
}

impl fmt::Debug for CssModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CssModule")
            .field("exports", &self.exports)
            .field("module", &self.module)
            .field("source", &self.source)
            .finish()
    }
}
