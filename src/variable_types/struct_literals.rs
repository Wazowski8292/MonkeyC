use crate::semantic_analyzer::{TableTypes, TokenType};
use super::variable::{Modifire, Types};

#[derive(Debug, PartialEq, Clone)]
pub struct StructLiteral {
    pub name: String,
    pub arguments: Vec<TableTypes>,
    pub functions: Vec<TableTypes>,
    pub modifire: Vec<Modifire>,
}

impl Types for StructLiteral {
    fn new(_: TokenType) -> Self {
        Self {
            name: "".to_string(),
            arguments: vec![],
            functions: vec![],
            modifire: vec![],
        }
    }

    fn add_arguments(&mut self, argument: String) {
        self.name = argument;
    }
}
