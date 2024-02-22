use crate::module::{Module, Source, SourceParser};
use crate::module_graph_error::ModuleGraphError;
use oxc_resolver::PackageJson as ResolvedPackageJson;
use starbase_utils::fs;
use std::sync::Arc;

// Vue, svelte

#[derive(Debug)]
pub enum TextModuleKind {
    Graphql,
    Html,
    Less,
    Sass,
    Sourcemap,
    Stylus,
    Svg,
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
        Ok(Source::Text(Box::new(TextModule {
            kind: match module.path.extension().and_then(|ext| ext.to_str()) {
                Some("gql" | "graphql") => TextModuleKind::Graphql,
                Some("html") => TextModuleKind::Html,
                Some("less") => TextModuleKind::Less,
                Some("map") => TextModuleKind::Sourcemap,
                Some("sass" | "scss") => TextModuleKind::Sass,
                Some("styl") => TextModuleKind::Stylus,
                Some("svg") => TextModuleKind::Svg,
                _ => TextModuleKind::Unknown,
            },
            source: Arc::new(fs::read_file(&module.path)?),
        })))
    }
}
