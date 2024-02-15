use crate::module_type::ModuleType;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct Module {
    /// Absolute path to the module file.
    pub path: PathBuf,

    /// Source in binary, to support all module types.
    pub source: Arc<Vec<u8>>,

    pub type_of: ModuleType,
}

impl Module {
    pub fn new(path: &Path, source: Vec<u8>, type_of: ModuleType) -> Self {
        Self {
            path: path.to_owned(),
            source: Arc::new(source),
            type_of,
        }
    }

    /// Is the module an external file (in node modules)?
    pub fn is_external(&self) -> bool {
        self.path
            .components()
            .any(|comp| comp.as_os_str() == "node_modules")
    }
}
