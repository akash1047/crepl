#[derive(Debug, Default, Clone)]
pub struct Position {
    pub filename: String,
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}
