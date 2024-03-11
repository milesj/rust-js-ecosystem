use crate::module::*;
use crate::module_graph_error::ModuleGraphError;
use nodejs_package_json::PackageJson;
use starbase_utils::fs;
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub enum MediaModuleKind {
    Audio,
    Image,
    Video,
}

#[derive(Debug)]
pub struct MediaModule {
    pub kind: MediaModuleKind,
    pub source: Arc<Vec<u8>>, // Binary file
}

impl ModuleSource for MediaModule {
    fn kind(&self) -> SourceKind {
        match self.kind {
            MediaModuleKind::Audio => SourceKind::Audio,
            MediaModuleKind::Image => SourceKind::Image,
            MediaModuleKind::Video => SourceKind::Video,
        }
    }

    fn source(&self) -> &[u8] {
        &self.source
    }

    fn load(
        module: &mut Module,
        _package_json: Option<Arc<PackageJson>>,
    ) -> Result<Self, ModuleGraphError> {
        // https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
        let kind = match module.path.extension().and_then(|ext| ext.to_str()) {
            // Audio
            Some(
                "aac" | "mid" | "midi" | "mp3" | "ogg" | "oga" | "mogg" | "opus" | "weba" | "wav",
            ) => MediaModuleKind::Audio,

            // Images
            Some(
                "apng" | "avif" | "bmp" | "gif" | "ico" | "jpg" | "jpeg" | "jpe" | "jif" | "jfif"
                | "pjpeg" | "pjp" | "png" | "tif" | "tiff" | "webp",
            ) => MediaModuleKind::Image,

            // Video
            Some("avi" | "mp4" | "mpeg" | "ogv" | "webm") => MediaModuleKind::Video,

            _ => {
                return Err(ModuleGraphError::UnsupportedFileType(module.path.clone()));
            }
        };

        Ok(MediaModule {
            kind,
            source: Arc::new(fs::read_file_bytes(&module.path)?),
        })
    }
}
