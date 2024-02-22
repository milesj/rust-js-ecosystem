use crate::module::{Module, Source, SourceParser};
use crate::module_graph_error::ModuleGraphError;
use oxc_resolver::PackageJson as ResolvedPackageJson;
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
    pub source: Arc<Vec<u8>>,
}

impl SourceParser for MediaModule {
    fn parse_into_module(
        module: &mut Module,
        _package_json: Option<Arc<ResolvedPackageJson>>,
    ) -> Result<Source, ModuleGraphError> {
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
                unreachable!();
            }
        };

        let source = Box::new(MediaModule {
            kind,
            source: Arc::new(fs::read_file_bytes(&module.path)?),
        });

        Ok(match kind {
            MediaModuleKind::Audio => Source::Audio(source),
            MediaModuleKind::Image => Source::Image(source),
            MediaModuleKind::Video => Source::Video(source),
        })
    }
}
