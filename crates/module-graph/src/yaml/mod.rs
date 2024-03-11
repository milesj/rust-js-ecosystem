use crate::atom::AtomStr;
use crate::module::*;
use crate::module_graph_error::ModuleGraphError;
use nodejs_package_json::PackageJson;
use starbase_utils::{fs, yaml};
use std::sync::Arc;

pub use starbase_utils::yaml::YamlValue;

#[derive(Debug)]
pub struct YamlModule {
    pub data: Arc<YamlValue>,
    pub source: Arc<String>,
}

impl ModuleSource for YamlModule {
    fn kind(&self) -> SourceKind {
        SourceKind::Yaml
    }

    fn source(&self) -> &[u8] {
        self.source.as_bytes()
    }

    fn load(
        module: &mut Module,
        _package_json: Option<Arc<PackageJson>>,
    ) -> Result<Self, ModuleGraphError> {
        let source = fs::read_file(&module.path)?;
        let data: YamlValue = yaml::from_str(&source).unwrap(); // TODO

        Ok(Self {
            data: Arc::new(data),
            source: Arc::new(source),
        })
    }

    fn parse(&mut self, module: &mut Module) -> Result<(), ModuleGraphError> {
        let mut export = Export {
            kind: ExportKind::Native,
            ..Export::default()
        };

        // The entire document itself is a default export
        export.symbols.push(ExportedSymbol {
            kind: ExportedKind::Default,
            symbol_id: None,
            name: AtomStr::from("default"),
        });

        // When an object document, each direct property is an export
        if let YamlValue::Mapping(object) = &*self.data {
            for key in object.keys() {
                if let YamlValue::String(key) = key {
                    export.symbols.push(ExportedSymbol {
                        kind: ExportedKind::Value,
                        symbol_id: None,
                        name: AtomStr::from(key.as_str()),
                    });
                }
            }
        }

        module.exports.push(export);

        Ok(())
    }
}
