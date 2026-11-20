use crate::semantic_analyzer::TokenType;
use super::variable::{Types, Variable};

#[derive(Debug, PartialEq, Clone)]
pub struct Return {
    pub value: Option<Variable>,
}

impl Types for Return {
    fn new(token: TokenType) -> Self {
        let mut var = Variable::new(token);
        var.name = Some("_".to_string());

        Self {
            value: Some(var),
        }
    }

    fn add_arguments(&mut self, argument: String) {
        if let Some(ref mut val) = self.value {
            val.add_arguments(argument);
        }
    }
}
