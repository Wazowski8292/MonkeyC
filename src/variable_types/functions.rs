use crate::semantic_analyzer::{TableTypes, TokenType};
use super::variable::{Modifire, Types};

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub parameters: Option<Vec<TableTypes>>,
    pub name: Option<String>,
    pub table: Vec<TableTypes>,
    pub return_type: Option<TokenType>,
    pub modifire: Vec<Modifire>,
}

impl Types for Function {
    fn new(_: TokenType) -> Self {
        Self {
            parameters: None,
            name: None,
            table: vec![],
            return_type: None,
            modifire: vec![],
        }
    }

    fn add_arguments(&mut self, argument: String) {
        if self.name.is_none() {
            self.name = Some(argument.clone());
        } else {
            let token = TokenType::from_str(&argument);
            if token == TokenType::Bool
                || token == TokenType::Char
                || token == TokenType::String
                || token == TokenType::Int
                || token == TokenType::Float
            {
                self.return_type = Some(token);
            }
        }
    }
}
