use mediatype::MediaTypeBuf;
use oxc::span::{Atom, Span};

use crate::module_type::ModuleType;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub enum ImportedValueKind {
    Default, // import name
    Star,    // import * as name
    Type,    // import { type value }, import type { value }
    Value,   // import { value }, import { value as name }
}

pub struct ImportedValue {
    pub kind: ImportedValueKind,
    pub source_name: Option<Atom>,
    pub name: Atom,
}

pub enum ImportKind {
    AsyncStatic,  // import
    AsyncDynamic, // import()
    SyncStatic,   // require()
}

pub struct Import {
    pub kind: ImportKind,
    pub source: Atom,
    pub span: Span,
    pub values: Vec<ImportedValue>,
}

pub struct ExportRecord {}

pub type ModuleId = u32;

#[derive(Default)]
pub struct Module {
    /// Fragment string appended to the file path.
    pub fragment: Option<String>,

    /// Unique ID of the module.
    pub id: ModuleId,

    /// List of modules being imported from, and the values being imported.
    pub imports: Vec<Import>,

    /// Absolute path to the module file.
    pub path: PathBuf,

    /// Query string appended to the file path.
    pub query: Option<String>,

    /// Source in binary, to support all module types.
    pub source: Arc<Vec<u8>>,

    /// Type of module, with associated mime and source information.
    pub type_of: ModuleType,
}

impl Module {
    pub fn new(path: &Path, source: Vec<u8>, type_of: ModuleType) -> Self {
        Self {
            id: 0,
            path: path.to_owned(),
            source: Arc::new(source),
            type_of,
            ..Module::default()
        }
    }

    pub fn get_mime_type(&self) -> &MediaTypeBuf {
        match &self.type_of {
            ModuleType::Audio { mime_type } => mime_type,
            ModuleType::Image { mime_type } => mime_type,
            ModuleType::JavaScript { mime_type, .. } => mime_type,
            ModuleType::Video { mime_type } => mime_type,
            _ => unreachable!(),
        }
    }

    /// Is the module an external file (in node modules)?
    pub fn is_external(&self) -> bool {
        self.path
            .components()
            .any(|comp| comp.as_os_str() == "node_modules")
    }
}
