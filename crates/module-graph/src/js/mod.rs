mod stats;
mod visit_imports_exports;

use self::visit_imports_exports::*;
use crate::module::*;
use crate::module_graph_error::ModuleGraphError;
use oxc::allocator::Allocator;
// use oxc::ast::ast::Program;
use oxc::ast::Visit;
use oxc::parser::Parser;
use oxc::span::SourceType;
use oxc_resolver::PackageJson as ResolvedPackageJson;
use rustc_hash::FxHashSet;
use starbase_utils::fs;
use std::fmt;
// use std::mem;
use std::sync::Arc;

pub use self::stats::JavaScriptStats;

pub struct JavaScriptModule {
    pub package_type: JavaScriptPackageType,
    pub source_type: SourceType,
    pub stats: JavaScriptStats,

    // Order is important here, as they need to be dropped in sequence!
    // pub program: Program<'static>,
    // #[allow(dead_code)]
    // allocator: Box<Allocator>,
    pub source: Arc<String>,
}

impl JavaScriptModule {
    pub fn is_barrel_file(&self, threshold: usize) -> bool {
        self.stats.other_statements == 0 && self.stats.export_statements >= threshold
    }
}

impl ModuleSource for JavaScriptModule {
    fn kind(&self) -> SourceKind {
        SourceKind::Unknown
    }

    fn source(&self) -> &[u8] {
        self.source.as_bytes()
    }

    fn load(
        module: &mut Module,
        _package_json: Option<Arc<ResolvedPackageJson>>,
    ) -> Result<Self, ModuleGraphError> {
        let source = fs::read_file(&module.path)?;
        let source_type = SourceType::from_path(&module.path).unwrap();
        // let allocator = Allocator::default();

        // // TODO errors
        // let program = unsafe {
        //     let src = mem::transmute::<_, &'static str>(&*source);
        //     let alloc = mem::transmute::<_, &'static Allocator>(&allocator);
        //     Parser::new(alloc, src, source_type).parse().program
        // };

        Ok(Self {
            package_type: JavaScriptPackageType::Unknown, // TODO
            source: Arc::new(source),
            source_type,
            stats: JavaScriptStats::default(),
            // allocator: Box::new(allocator),
            // program,
        })
    }

    fn parse(&mut self, module: &mut Module) -> Result<(), ModuleGraphError> {
        // TODO temporary, move to load
        let source = Arc::clone(&self.source);
        let allocator = Allocator::default();
        let program = Parser::new(&allocator, &source, self.source_type)
            .parse()
            .program;

        // Extract imports and exports
        {
            let mut stats = JavaScriptStats::default();
            let mut visitor = ExtractImportsExports {
                module,
                stats: &mut stats,
                extracted_dynamic_imports: FxHashSet::default(),
                extracted_requires: FxHashSet::default(),
                ast: std::marker::PhantomData,
            };

            visitor.visit_program(&program);
            self.stats = stats;
        }

        Ok(())
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

impl fmt::Debug for JavaScriptModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JavaScriptModule")
            .field("package_type", &self.package_type)
            .field("source", &self.source)
            .field("source_type", &self.source_type)
            .field("stats", &self.stats)
            .finish()
    }
}
