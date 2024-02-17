use crate::module::{
    Export, ExportKind, ExportedKind, ExportedSymbol, Module, Source, SourceParser,
};
use crate::module_graph_error::ModuleGraphError;
use mediatype::names::x_::YAML;
use mediatype::names::APPLICATION;
use mediatype::MediaTypeBuf;
use oxc::span::{Atom, Span};
use oxc::syntax::symbol::SymbolId;
use starbase_utils::yaml::{self, YamlValue};
use std::cell::Cell;
use std::sync::Arc;

pub struct YamlModule {
    pub mime_type: MediaTypeBuf,
    pub source: Arc<YamlValue>,
}

impl SourceParser for YamlModule {
    fn parse_into_module(module: &mut Module) -> Result<Source, ModuleGraphError> {
        let data: YamlValue = yaml::read_file(&module.path)?;

        let mut symbol_count: isize = -1;
        let mut create_symbol = || {
            symbol_count += 1;
            Some(SymbolId::new(symbol_count as usize))
        };

        let mut export = Export {
            kind: ExportKind::Native,
            module_id: 0,
            source: None,
            span: Span::default(),
            symbols: vec![],
        };

        // The entire document itself is a default export
        export.symbols.push(ExportedSymbol {
            kind: ExportedKind::Default,
            symbol_id: Cell::new(create_symbol()),
            name: Atom::from("default"),
        });

        // When an object document, each direct property is an export
        if let YamlValue::Mapping(object) = &data {
            for key in object.keys() {
                if let YamlValue::String(key) = key {
                    export.symbols.push(ExportedSymbol {
                        kind: ExportedKind::Value,
                        symbol_id: Cell::new(create_symbol()),
                        name: Atom::from(key.as_str()),
                    });
                }
            }
        }

        module.exports.push(export);

        Ok(Source::Yaml(Box::new(YamlModule {
            mime_type: MediaTypeBuf::new(APPLICATION, YAML),
            source: Arc::new(data),
        })))
    }
}
