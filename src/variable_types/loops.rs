use crate::semantic_analyzer::{TableTypes, TokenType};
use super::reasignment::Reasingment;
use super::variable::{Types, Variable};

#[derive(Debug, PartialEq, Clone)]
pub struct Loop {
    pub token_type: TokenType,
    pub condition: Vec<TableTypes>,
    pub table: Vec<TableTypes>,
}

impl Types for Loop {
    fn new(token_type: TokenType) -> Self {
        let mut table_type = TableTypes::Unknown;
        if token_type != TokenType::ForLoop {
            let mut var = Variable::new(TokenType::BoolLiteral);
            var.name = Some("_".to_string());
            table_type = TableTypes::Variable(var);
        }

        Self {
            token_type: token_type,
            condition: vec![table_type],
            table: vec![],
        }
    }

    fn add_arguments(&mut self, mut argument: String) {
        let is_first_for_arg = self.token_type == TokenType::ForLoop && argument.ends_with(',');

        match self.condition.last_mut() {
            Some(TableTypes::Variable(variable)) => {
                if is_first_for_arg {
                    argument.pop();
                }

                variable.add_arguments(argument);

                if is_first_for_arg {
                    if self.condition.len() == 1 {
                        let mut var = Variable::new(TokenType::BoolLiteral);
                        var.name = Some("_".to_string());
                        self.condition.push(TableTypes::Variable(var));
                    } else if self.condition.len() == 2 {
                        if let TableTypes::Variable(variable) = self.condition[0].clone() {
                            let reasign = Reasingment::new(variable.token_type);
                            self.condition.push(TableTypes::Reasingment(reasign));
                        }
                    }
                }
            }
            Some(TableTypes::Reasingment(reasign)) => {
                if reasign.name.is_empty() {
                    reasign.name = argument;
                } else {
                    reasign.add_arguments(argument);
                }
            }
            Some(TableTypes::Unknown) => {
                self.condition.pop();
                let variable = Variable::new(TokenType::from_str(&argument));
                self.condition.push(TableTypes::Variable(variable));
            }
            _ => {}
        }
    }
}
