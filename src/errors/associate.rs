#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeAssociate {
    pub lines: String,
    pub file: String,
    pub line: usize,
    pub start_column: usize,
    pub end_column: usize,
}
