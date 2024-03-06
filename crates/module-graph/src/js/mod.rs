mod stats;
mod visit_imports_exports;

use self::visit_imports_exports::*;
use crate::module::*;
use crate::module_graph_error::ModuleGraphError;
// use once_cell::sync::OnceCell;
use oxc::allocator::Allocator;
use oxc::ast::ast::Program;
use oxc::ast::Visit;
use oxc::parser::Parser;
use oxc::span::SourceType;
use oxc_resolver::PackageJson as ResolvedPackageJson;
use rustc_hash::FxHashSet;
use starbase_utils::fs;
use std::fmt;
use std::sync::Arc;

pub use self::stats::JavaScriptStats;

pub struct JavaScriptModule {
    pub package_type: JavaScriptPackageType,
    pub source: Arc<String>,
    pub source_type: SourceType,
    pub stats: JavaScriptStats,
    // pub program: OnceCell<Program<'static>>,
    // allocator: Pin<Box<Allocator>>,
}

impl JavaScriptModule {
    pub fn is_barrel_file(&self, threshold: usize) -> bool {
        self.stats.other_statements == 0 && self.stats.export_statements >= threshold
    }

    // TODO: Get this working!!!
    // pub fn get_program(&self) -> Result<&Program<'static>, ModuleGraphError> {
    //     let source_type = self.source_type;
    //     let source = unsafe { mem::transmute::<_, &'static str>(&*self.source) };
    //     let allocator = unsafe { mem::transmute::<_, &'static Allocator>(&self.allocator) };

    //     self.program.get_or_try_init(move || {
    //         let parser = Parser::new(allocator, source, source_type);
    //         let result = parser.parse();

    //         // TODO handle errors

    //         // Ok(unsafe { mem::transmute::<Program<'_>, Program<'static>>(result.program) })
    //         Ok(result.program)
    //     })
    // }
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

        Ok(Self {
            package_type: JavaScriptPackageType::Unknown, // TODO
            // program: OnceCell::new(),
            source: Arc::new(source),
            source_type,
            stats: JavaScriptStats::default(),
            // allocator: Box::pin(Allocator::default()),
        })
    }

    fn parse(&mut self, module: &mut Module) -> Result<(), ModuleGraphError> {
        // let program = { self.get_program()? };
        let allocator = Allocator::default();
        let result = Parser::new(&allocator, &self.source, self.source_type).parse();
        let program = result.program;

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
