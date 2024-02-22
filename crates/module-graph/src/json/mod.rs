use crate::module::{
    Export, ExportKind, ExportedKind, ExportedSymbol, Module, Source, SourceParser,
};
use crate::module_graph_error::ModuleGraphError;
use oxc::span::Atom;
use oxc::syntax::symbol::SymbolId;
use oxc_resolver::PackageJson as ResolvedPackageJson;
use starbase_utils::json::{self, JsonValue};
use std::sync::Arc;

#[derive(Debug)]
pub struct JsonModule {
    pub source: Arc<JsonValue>,
}

impl SourceParser for JsonModule {
    fn parse_into_module(
        module: &mut Module,
        _package_json: Option<Arc<ResolvedPackageJson>>,
    ) -> Result<Source, ModuleGraphError> {
        let data: JsonValue = json::read_file(&module.path)?;

        let mut symbol_count: isize = -1;
        let mut create_symbol = || {
            symbol_count += 1;
            Some(SymbolId::new(symbol_count as usize))
        };

        let mut export = Export {
            kind: ExportKind::Native,
            ..Export::default()
        };

        // The entire document itself is a default export
        export.symbols.push(ExportedSymbol {
            kind: ExportedKind::Default,
            symbol_id: create_symbol(),
            name: Atom::from("default"),
        });

        // When an object document, each direct property is an export
        if let JsonValue::Object(object) = &data {
            for key in object.keys() {
                export.symbols.push(ExportedSymbol {
                    kind: ExportedKind::Value,
                    symbol_id: create_symbol(),
                    name: Atom::from(key.as_str()),
                });
            }
        }

        module.exports.push(export);

        Ok(Source::Json(Box::new(JsonModule {
            source: Arc::new(data),
        })))
    }
}
