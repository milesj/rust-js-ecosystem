#[derive(Debug, Default)]
pub struct JavaScriptStats {
    pub dynamic_import_count: usize,
    pub export_statements: usize,
    pub exports_default: bool,
    pub import_statements: usize,
    pub other_statements: usize, // In the program root
    pub require_count: usize,
}
