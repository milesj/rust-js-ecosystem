use crate::module::{Module, Source, SourceParser};
use crate::module_graph_error::ModuleGraphError;
use mediatype::names::vnd::{MICROSOFT_ICON, MOZILLA_APNG};
use mediatype::names::x_::{MIDI, MSVIDEO};
use mediatype::names::{
    AAC, AUDIO, AVIF, BMP, GIF, IMAGE, JPEG, MP4, MPEG, OGG, OPUS, PNG, TIFF, VIDEO, WAV, WEBM,
    WEBP,
};
use mediatype::MediaTypeBuf;
use starbase_utils::fs;
use std::sync::Arc;

#[derive(Debug)]
pub struct MediaModule {
    pub mime_type: MediaTypeBuf,
    pub source: Arc<Vec<u8>>,
}

impl MediaModule {
    pub fn is_audio(&self) -> bool {
        self.mime_type.ty() == AUDIO
    }

    pub fn is_image(&self) -> bool {
        self.mime_type.ty() == IMAGE
    }

    pub fn is_video(&self) -> bool {
        self.mime_type.ty() == VIDEO
    }
}

impl SourceParser for MediaModule {
    fn parse_into_module(module: &mut Module) -> Result<Source, ModuleGraphError> {
        // https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
        let mime_type = match module.path.extension().and_then(|ext| ext.to_str()) {
            // Audio
            Some("aac") => MediaTypeBuf::new(AUDIO, AAC),
            Some("mid" | "midi") => MediaTypeBuf::new(AUDIO, MIDI),
            Some("mp3") => MediaTypeBuf::new(AUDIO, MPEG),
            Some("ogg" | "oga" | "mogg") => MediaTypeBuf::new(AUDIO, OGG),
            Some("opus") => MediaTypeBuf::new(AUDIO, OPUS),
            Some("weba") => MediaTypeBuf::new(AUDIO, WEBM),
            Some("wav") => MediaTypeBuf::new(AUDIO, WAV),

            // Images
            Some("apng") => MediaTypeBuf::new(IMAGE, MOZILLA_APNG),
            Some("avif") => MediaTypeBuf::new(IMAGE, AVIF),
            Some("bmp") => MediaTypeBuf::new(IMAGE, BMP),
            Some("gif") => MediaTypeBuf::new(IMAGE, GIF),
            Some("ico") => MediaTypeBuf::new(IMAGE, MICROSOFT_ICON),
            Some("jpg" | "jpeg" | "jpe" | "jif" | "jfif" | "pjpeg" | "pjp") => {
                MediaTypeBuf::new(IMAGE, JPEG)
            }
            Some("png") => MediaTypeBuf::new(IMAGE, PNG),
            Some("tif" | "tiff") => MediaTypeBuf::new(IMAGE, TIFF),
            Some("webp") => MediaTypeBuf::new(IMAGE, WEBP),

            // Video
            Some("avi") => MediaTypeBuf::new(VIDEO, MSVIDEO),
            Some("mp4") => MediaTypeBuf::new(VIDEO, MP4),
            Some("mpeg") => MediaTypeBuf::new(VIDEO, MPEG),
            Some("ogv") => MediaTypeBuf::new(VIDEO, OGG),
            Some("webm") => MediaTypeBuf::new(VIDEO, WEBM),

            _ => {
                unreachable!();
            }
        };

        let source = Box::new(MediaModule {
            mime_type,
            source: Arc::new(fs::read_file_bytes(&module.path)?),
        });

        Ok(if source.is_audio() {
            Source::Audio(source)
        } else if source.is_image() {
            Source::Image(source)
        } else if source.is_video() {
            Source::Video(source)
        } else {
            unreachable!()
        })
    }
}
