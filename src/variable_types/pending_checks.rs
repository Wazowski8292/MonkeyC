#[derive(Debug, PartialEq, Clone)]
pub struct PendingCheck {
    pub module: String,
    pub symbol: String,
    pub line: usize,
    pub char_num: usize,
}

impl PendingCheck {
    pub fn new(module: String, symbol: String, line: usize, char_num: usize) -> Self {
        Self {
            module,
            symbol,
            line,
            char_num,
        }
    }
}