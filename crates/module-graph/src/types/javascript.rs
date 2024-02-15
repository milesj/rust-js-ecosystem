use crate::{Module, ModuleGraphError, ModuleType};
use mediatype::names::{JAVASCRIPT, TEXT};
use mediatype::MediaTypeBuf;
use oxc::allocator::Allocator;
use oxc::parser::Parser;
use oxc::span::SourceType;
use starbase_utils::fs;
use std::path::Path;
use std::sync::Arc;

pub fn create_javascript_module(path: &Path) -> Result<Module, ModuleGraphError> {
    let source_text = fs::read_file(path)?;
    let source_type = SourceType::from_path(path).unwrap();
    let mut module = Module::new(
        path,
        Vec::new(),
        ModuleType::JavaScript {
            mime_type: MediaTypeBuf::new(TEXT, JAVASCRIPT),
            source_type,
        },
    );

    {
        let allocator = Allocator::default();
        let parser = Parser::new(&allocator, &source_text, source_type);
        let result = parser.parse();
    }

    module.source = Arc::new(source_text.into_bytes());

    Ok(module)
}
