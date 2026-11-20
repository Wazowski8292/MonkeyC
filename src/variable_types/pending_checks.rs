#[derive(Debug, PartialEq, Clone)]
pub struct PendingCheck {
    pub module: String,
    pub symbol: String,
    pub line: usize,
    pub char_num: usize,
}
