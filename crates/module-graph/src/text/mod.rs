use crate::module::*;
use crate::module_graph_error::ModuleGraphError;
use oxc_resolver::PackageJson as ResolvedPackageJson;
use starbase_utils::fs;
use std::sync::Arc;

#[derive(Debug)]
pub enum TextModuleKind {
    Graphql,
    Html,
    Less,
    Sass,
    Scss,
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

impl ModuleSource for TextModule {
    fn kind(&self) -> SourceKind {
        SourceKind::Text
    }

    fn source(&self) -> &[u8] {
        self.source.as_bytes()
    }

    fn load(
        module: &mut Module,
        _package_json: Option<Arc<ResolvedPackageJson>>,
    ) -> Result<Self, ModuleGraphError> {
        Ok(TextModule {
            kind: match module.path.extension().and_then(|ext| ext.to_str()) {
                Some("gql" | "graphql") => TextModuleKind::Graphql,
                Some("html") => TextModuleKind::Html,
                Some("less") => TextModuleKind::Less,
                Some("map") => TextModuleKind::Sourcemap,
                Some("sass") => TextModuleKind::Sass,
                Some("scss") => TextModuleKind::Scss,
                Some("styl") => TextModuleKind::Stylus,
                Some("svg") => TextModuleKind::Svg,
                _ => TextModuleKind::Unknown,
            },
            source: Arc::new(fs::read_file(&module.path)?),
        })
    }
}
