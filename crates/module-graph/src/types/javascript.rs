use crate::js::ExtractImportsExports;
use crate::module::*;
use crate::module_graph_error::ModuleGraphError;
use crate::module_type::ModuleType;
use mediatype::names::{JAVASCRIPT, TEXT};
use mediatype::MediaTypeBuf;
use oxc::allocator::Allocator;
use oxc::ast::ast::{ImportDeclarationSpecifier, ModuleDeclaration, Statement};
use oxc::ast::Visit;
use oxc::parser::Parser;
use oxc::span::SourceType;
use oxc_resolver::{AliasValue, ResolveOptions, Resolver};
use rustc_hash::FxHashSet;
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
            package_type: PackageType::Unknown, // TODO
            source_type,
        },
    );

    // Parse the file into an AST, and extract imports/exports
    let allocator = Allocator::default();
    let parser = Parser::new(&allocator, &source_text, source_type);
    let result = parser.parse();

    // TODO handle errors

    // Extract imports and exports
    {
        let mut visitor = ExtractImportsExports {
            module: &mut module,
            extracted_dynamic_imports: FxHashSet::default(),
            extracted_requires: FxHashSet::default(),
        };

        visitor.visit_program(&result.program);
    }

    drop(result);

    module.source = Arc::new(source_text.into_bytes());

    Ok(module)
}

pub fn resolve_path_of_import(importee: &Path, specifier: &str, options: ResolveOptions) {
    Resolver::new(options).resolve(importee, specifier);
}

#[derive(Debug, Default, Clone, Copy)]
pub enum PackageType {
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

impl PackageType {
    pub fn is_esm(&self) -> bool {
        matches!(self, Self::Mjs | Self::EsmPackageJson)
    }

    pub fn is_commonjs(&self) -> bool {
        matches!(self, Self::Cjs | Self::CjsPackageJson)
    }
}
