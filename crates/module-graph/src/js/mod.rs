mod stats;
mod visit_imports_exports;

use self::visit_imports_exports::*;
use crate::module::{Module, Source, SourceParser};
use crate::module_graph_error::ModuleGraphError;
use mediatype::names::{JAVASCRIPT, TEXT};
use mediatype::MediaTypeBuf;
use oxc::allocator::Allocator;
use oxc::ast::ast::Program;
use oxc::ast::Visit;
use oxc::parser::Parser;
use oxc::span::SourceType;
use rustc_hash::FxHashSet;
use starbase_utils::fs;
use std::sync::Arc;

pub use self::stats::JavaScriptStats;

pub struct JavaScriptModule {
    pub ast: Option<Program<'static>>,
    pub mime_type: MediaTypeBuf,
    pub package_type: JavaScriptPackageType,
    pub source: Arc<String>,
    pub source_type: SourceType,
    pub stats: JavaScriptStats,
}

impl JavaScriptModule {
    pub fn is_barrel_file(&self, threshold: usize) -> bool {
        self.stats.other_statements == 0 && self.stats.export_statements >= threshold
    }
}

impl SourceParser for JavaScriptModule {
    fn parse_into_module(module: &mut Module) -> Result<Source, ModuleGraphError> {
        let source_text = fs::read_file(&module.path)?;
        let source_type = SourceType::from_path(&module.path).unwrap();
        let mut stats = JavaScriptStats::default();

        // Parse the file into an AST, and extract imports/exports
        let allocator = Allocator::default();
        let parser = Parser::new(&allocator, &source_text, source_type);
        let result = parser.parse();

        // TODO handle errors

        // Extract imports and exports
        {
            let mut visitor = ExtractImportsExports {
                module,
                stats: &mut stats,
                extracted_dynamic_imports: FxHashSet::default(),
                extracted_requires: FxHashSet::default(),
            };

            visitor.visit_program(&result.program);
        }

        drop(result);

        Ok(Source::JavaScript(Box::new(JavaScriptModule {
            ast: None,
            mime_type: MediaTypeBuf::new(TEXT, JAVASCRIPT),
            package_type: JavaScriptPackageType::Unknown, // TODO
            source: Arc::new(source_text),
            source_type,
            stats,
        })))
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum JavaScriptPackageType {
    #[default]
    Unknown,
    // ".cjs"
    Cjs,
    // "type: commonjs" in package.json
    CjsPackageJson,
    // ".mjs"
    Mjs,
    // "type: module" in package.json
    EsmPackageJson,
}

impl JavaScriptPackageType {
    pub fn is_esm(&self) -> bool {
        matches!(self, Self::Mjs | Self::EsmPackageJson)
    }

    pub fn is_cjs(&self) -> bool {
        matches!(self, Self::Cjs | Self::CjsPackageJson)
    }
}
