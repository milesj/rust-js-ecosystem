use crate::module::*;
use crate::module_graph_error::ModuleGraphError;
use nodejs_package_json::PackageJson;
use std::sync::Arc;

#[derive(Debug)]
pub struct DummyModule;

impl ModuleSource for DummyModule {
    fn kind(&self) -> SourceKind {
        SourceKind::Unknown
    }

    fn source(&self) -> &[u8] {
        &[]
    }

    fn load(
        _module: &mut Module,
        _package_json: Option<Arc<PackageJson>>,
    ) -> Result<Self, ModuleGraphError> {
        Ok(DummyModule)
    }
}
