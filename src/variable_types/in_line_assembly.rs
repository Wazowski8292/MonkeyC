use crate::semantic_analyzer::TokenType;
use super::variable::Types;

#[derive(Debug, PartialEq, Clone)]
pub struct InlineAssembly {
    pub table: Vec<String>,
    pub alloc_size: usize,
}

impl Types for InlineAssembly {
    fn new(_: TokenType) -> Self {
        Self {
            table: vec![],
            alloc_size: 1,
        }
    }

    fn add_arguments(&mut self, argument: String) {
        self.alloc_size = argument.parse::<usize>().unwrap();
    }
}
