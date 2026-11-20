use crate::semantic_analyzer::{TableTypes, TokenType};
use super::variable::{Types, Variable};

#[derive(Debug, PartialEq, Clone)]
pub struct Conditional {
    pub token_type: TokenType,
    pub condition: Vec<TableTypes>,
    pub table: Vec<TableTypes>,
}

impl Types for Conditional {
    fn new(token: TokenType) -> Self {
        let mut var = Variable::new(TokenType::BoolLiteral);
        var.name = Some("_".to_string());

        Self {
            token_type: token,
            condition: vec![TableTypes::Variable(var)],
            table: vec![],
        }
    }

    fn add_arguments(&mut self, argument: String) {
        if let Some(TableTypes::Variable(v)) = self.condition.last_mut() {
            v.add_arguments(argument);
        }
    }
}
