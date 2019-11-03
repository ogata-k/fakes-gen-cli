#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FileType {
    CSV,
    TSV,
    JSON,
}

impl Default for FileType {
    fn default() -> Self {
        FileType::CSV
    }
}
