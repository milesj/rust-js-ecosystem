use crate::types::javascript::PackageType;
use mediatype::MediaTypeBuf;
use oxc::span::SourceType;

#[derive(Default)]
pub enum ModuleType {
    // Asset
    Audio {
        mime_type: MediaTypeBuf,
    },
    // Css
    Image {
        mime_type: MediaTypeBuf,
    },
    JavaScript {
        mime_type: MediaTypeBuf,
        package_type: PackageType,
        source_type: SourceType,
    },
    Video {
        mime_type: MediaTypeBuf,
    },
    #[default]
    Unknown,
}
