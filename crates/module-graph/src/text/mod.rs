use crate::module::{Module, Source, SourceParser};
use crate::module_graph_error::ModuleGraphError;
use oxc_resolver::PackageJson as ResolvedPackageJson;
use starbase_utils::fs;
use std::sync::Arc;

#[derive(Debug)]
pub enum TextModuleKind {
    Graphql,
    Unknown,
}

#[derive(Debug)]
pub struct TextModule {
    pub kind: TextModuleKind,
    pub source: Arc<String>,
}

impl SourceParser for TextModule {
    fn parse_into_module(
        module: &mut Module,
        _package_json: Option<Arc<ResolvedPackageJson>>,
    ) -> Result<Source, ModuleGraphError> {
        let kind = match module.path.extension().and_then(|ext| ext.to_str()) {
            Some("gql" | "graphql") => TextModuleKind::Graphql,
            _ => TextModuleKind::Unknown,
        };

        Ok(Source::Text(Box::new(TextModule {
            kind,
            source: Arc::new(fs::read_file(&module.path)?),
        })))
    }
}
