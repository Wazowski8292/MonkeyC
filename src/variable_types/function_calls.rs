use crate::semantic_analyzer::{Scope, TableTypes, TokenType};
use super::variable::{parse_array_syntax, Types, Value, Variable};

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub target: usize,
    pub parameters: Option<Vec<TableTypes>>,
    pub name: String,
    pub scope: Scope,
}

impl Types for FunctionCall {
    fn new(_: TokenType) -> Self {
        Self {
            target: 0,
            parameters: None,
            name: String::new(),
            scope: Scope::Root,
        }
    }

    fn add_arguments(&mut self, argument: String) {
        let table_type = if let Some((arr_name, idx_str)) = parse_array_syntax(&argument) {
            let mut v = Variable::new(TokenType::Unknow);
            v.value = Some(vec![Value::Index(arr_name, idx_str)]);
            TableTypes::Variable(v)
        } else {
            let mut table_type = TableTypes::from_token(TokenType::from_str(&argument));
            match table_type {
                TableTypes::Variable(ref mut v) => v.value.get_or_insert_with(Vec::new).push(Value::Var(argument)),
                TableTypes::Reasingment(ref mut r) => r.name = argument,
                _ => {}
            }
            table_type
        };

        self.parameters.get_or_insert_with(Vec::new).push(table_type);
    }
}
