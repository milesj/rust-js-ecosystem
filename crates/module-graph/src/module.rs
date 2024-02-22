use crate::css::CssModule;
use crate::js::JavaScriptModule;
use crate::json::JsonModule;
use crate::media::MediaModule;
use crate::module_graph_error::ModuleGraphError;
use crate::text::TextModule;
use crate::yaml::YamlModule;
use oxc::ast::ast::BindingIdentifier;
use oxc::span::{Atom, Span};
use oxc::syntax::symbol::SymbolId;
use oxc_resolver::PackageJson as ResolvedPackageJson;
use starbase_utils::json::JsonValue;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug)]
pub enum ImportedKind {
    Default,       // import name
    DefaultType,   // import type name
    Namespace,     // import * as name
    NamespaceType, // import type * as name
    Value,         // import { value }, import { value as name }
    ValueType,     // import { type T }, import type { T }
}

#[derive(Debug)]
pub struct ImportedSymbol {
    pub kind: ImportedKind,
    pub source_name: Option<Atom>,
    pub symbol_id: Option<SymbolId>,
    pub name: Atom,
}

impl ImportedSymbol {
    pub fn from_binding(kind: ImportedKind, binding: &BindingIdentifier) -> Self {
        Self {
            kind,
            source_name: None,
            symbol_id: binding.symbol_id.clone().into_inner(),
            name: binding.name.clone(),
        }
    }
}

#[derive(Debug)]
pub enum ImportKind {
    AsyncStatic,  // import
    AsyncDynamic, // import()
    SyncStatic,   // require()
}

#[derive(Debug)]
pub struct Import {
    pub kind: ImportKind,
    pub module_id: ModuleId,
    pub source_request: Atom,
    pub span: Span,
    pub symbols: Vec<ImportedSymbol>,
    pub type_only: bool,
}

#[derive(Debug)]
pub enum ExportedKind {
    Default,       // export default name
    DefaultType,   // export default T
    Namespace,     // export *, export * as name
    NamespaceType, // export type *, export type * as name
    Value,         // export name, export { name }
    ValueType,     // export type T, export { type name }
}

#[derive(Debug)]
pub struct ExportedSymbol {
    pub kind: ExportedKind,
    pub symbol_id: Option<SymbolId>,
    pub name: Atom,
}

#[derive(Debug, Default)]
pub enum ExportKind {
    #[default]
    Modern, // export
    Legacy, // module.exports, exports.name
    Native, // non-JS files
}

#[derive(Debug, Default)]
pub struct Export {
    pub kind: ExportKind,
    pub module_id: Option<ModuleId>,
    pub source: Option<Atom>,
    pub span: Option<Span>,
    pub symbols: Vec<ExportedSymbol>,
    pub type_only: bool,
}

pub type ModuleId = u32;

#[derive(Debug, Default)]
pub enum Source {
    #[default]
    Unknown,
    Audio(Box<MediaModule>),
    Css(Box<CssModule>),
    Image(Box<MediaModule>),
    JavaScript(Box<JavaScriptModule>),
    Json(Box<JsonModule>),
    Text(Box<TextModule>),
    Video(Box<MediaModule>),
    Yaml(Box<YamlModule>),
}

pub trait SourceParser {
    fn parse_into_module(
        module: &mut Module,
        package_json: Option<Arc<ResolvedPackageJson>>,
    ) -> Result<Source, ModuleGraphError>;
}

#[derive(Debug, Default)]
pub struct Module {
    /// List of symbols being exported, and optionally the module they came from.
    pub exports: Vec<Export>,

    /// Fragment string appended to the file path.
    pub fragment: Option<String>,

    /// Unique ID of the module.
    pub id: ModuleId,

    /// List of modules being imported from, and the symbols being imported.
    pub imports: Vec<Import>,

    /// Name of the package the module belongs to.
    pub package_name: Option<String>,

    /// Absolute path to the module file.
    pub path: PathBuf,

    /// Query string appended to the file path.
    pub query: Option<String>,

    /// Type of module, with associated mime and source information.
    pub source: Source,
}

impl Module {
    pub fn new(path: &Path) -> Self {
        Self {
            id: 0,
            path: path.to_owned(),
            ..Module::default()
        }
    }

    /// Is the module an external file (in node modules)?
    pub fn is_external(&self) -> bool {
        self.path
            .components()
            .any(|comp| comp.as_os_str() == "node_modules")
    }

    pub(crate) fn load_and_parse_source(
        &mut self,
        package_json: Option<Arc<ResolvedPackageJson>>,
    ) -> Result<(), ModuleGraphError> {
        if let Some(package) = &package_json {
            if let Some(fields) = package.raw_json().as_object() {
                if let Some(JsonValue::String(name)) = fields.get("name") {
                    self.package_name = Some(name.to_owned());
                }
            }
        }

        self.source = match self.path.extension().and_then(|ext| ext.to_str()) {
            Some("css") => CssModule::parse_into_module(self, package_json)?,
            Some("js" | "jsx" | "ts" | "tsx" | "mts" | "cts" | "mjs" | "cjs") => {
                JavaScriptModule::parse_into_module(self, package_json)?
            }
            Some("json" | "jsonc" | "json5") => JsonModule::parse_into_module(self, package_json)?,
            Some("yaml" | "yml") => YamlModule::parse_into_module(self, package_json)?,
            Some(
                "gql" | "graphql" | "html" | "less" | "map" | "sass" | "scss" | "styl" | "svg",
            ) => TextModule::parse_into_module(self, package_json)?,
            _ => MediaModule::parse_into_module(self, package_json)?,
        };

        Ok(())
    }
}
