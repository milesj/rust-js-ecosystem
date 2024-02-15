use crate::{Module, ModuleGraphError, ModuleType};
use mediatype::names::vnd::{MICROSOFT_ICON, MOZILLA_APNG};
use mediatype::names::x_::{MIDI, MSVIDEO};
use mediatype::names::{
    AAC, AUDIO, AVIF, BMP, GIF, IMAGE, JPEG, MP4, MPEG, OGG, OPUS, PNG, TIFF, VIDEO, WAV, WEBM,
    WEBP,
};
use mediatype::MediaTypeBuf;
use starbase_utils::fs;
use std::path::Path;

pub fn create_media_module(path: &Path) -> Result<Module, ModuleGraphError> {
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
    let module_type = match path.extension().map(|ext| ext.to_str()).flatten() {
        // Audio
        Some("aac") => ModuleType::Audio {
            mime_type: MediaTypeBuf::new(AUDIO, AAC),
        },
        Some("mid" | "midi") => ModuleType::Audio {
            mime_type: MediaTypeBuf::new(AUDIO, MIDI),
        },
        Some("mp3") => ModuleType::Audio {
            mime_type: MediaTypeBuf::new(AUDIO, MPEG),
        },
        Some("ogg" | "oga" | "mogg") => ModuleType::Audio {
            mime_type: MediaTypeBuf::new(AUDIO, OGG),
        },
        Some("opus") => ModuleType::Audio {
            mime_type: MediaTypeBuf::new(AUDIO, OPUS),
        },
        Some("weba") => ModuleType::Audio {
            mime_type: MediaTypeBuf::new(AUDIO, WEBM),
        },
        Some("wav") => ModuleType::Audio {
            mime_type: MediaTypeBuf::new(AUDIO, WAV),
        },
        // Images
        Some("apng") => ModuleType::Image {
            mime_type: MediaTypeBuf::new(IMAGE, MOZILLA_APNG),
        },
        Some("avif") => ModuleType::Image {
            mime_type: MediaTypeBuf::new(IMAGE, AVIF),
        },
        Some("bmp") => ModuleType::Image {
            mime_type: MediaTypeBuf::new(IMAGE, BMP),
        },
        Some("gif") => ModuleType::Image {
            mime_type: MediaTypeBuf::new(IMAGE, GIF),
        },
        Some("ico") => ModuleType::Image {
            mime_type: MediaTypeBuf::new(IMAGE, MICROSOFT_ICON),
        },
        Some("jpg" | "jpeg" | "jpe" | "jif" | "jfif" | "pjpeg" | "pjp") => ModuleType::Image {
            mime_type: MediaTypeBuf::new(IMAGE, JPEG),
        },
        Some("png") => ModuleType::Image {
            mime_type: MediaTypeBuf::new(IMAGE, PNG),
        },
        Some("tif" | "tiff") => ModuleType::Image {
            mime_type: MediaTypeBuf::new(IMAGE, TIFF),
        },
        Some("webp") => ModuleType::Image {
            mime_type: MediaTypeBuf::new(IMAGE, WEBP),
        },
        // Video
        Some("avi") => ModuleType::Video {
            mime_type: MediaTypeBuf::new(VIDEO, MSVIDEO),
        },
        Some("mp4") => ModuleType::Video {
            mime_type: MediaTypeBuf::new(VIDEO, MP4),
        },
        Some("mpeg") => ModuleType::Video {
            mime_type: MediaTypeBuf::new(VIDEO, MPEG),
        },
        Some("ogv") => ModuleType::Video {
            mime_type: MediaTypeBuf::new(VIDEO, OGG),
        },
        Some("webm") => ModuleType::Video {
            mime_type: MediaTypeBuf::new(VIDEO, WEBM),
        },
        _ => {
            unreachable!();
        }
    };

    Ok(Module::new(path, fs::read_file_bytes(path)?, module_type))
}
