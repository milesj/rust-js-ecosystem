use mediatype::MediaTypeBuf;
use oxc::span::SourceType;

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
        source_type: SourceType,
    },
    Video {
        mime_type: MediaTypeBuf,
    },
}
