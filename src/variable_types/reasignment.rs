use crate::semantic_analyzer::{Scope, TableTypes, TokenType};
use super::function_calls::FunctionCall;
use super::variable::{parse_array_syntax, Modifire, PointerType, Types, Value, Variable};

#[derive(Debug, PartialEq, Clone)]
pub struct Reasingment {
    pub target: usize,
    pub target_scope: Scope,
    pub parameters: Option<Vec<TableTypes>>,
    pub name: String,
    pub token_type: TokenType,
    pub ptr: Option<PointerType>,
    pub array_index: Option<String>,
    pub modifire: Vec<Modifire>,
}

impl Reasingment {
    pub fn pending_call(&self) -> Option<&FunctionCall> {
        match self.parameters.as_ref()?.last()? {
            TableTypes::FunctionCall(fc) => Some(fc),
            _ => None,
        }
    }

    pub fn pending_call_mut(&mut self) -> Option<&mut FunctionCall> {
        match self.parameters.as_mut()?.last_mut()? {
            TableTypes::FunctionCall(fc) => Some(fc),
            _ => None,
        }
    }
}

impl Types for Reasingment {
    fn new(_: TokenType) -> Self {
        Self {
            target: 0,
            target_scope: Scope::Root,
            parameters: None,
            name: String::new(),
            token_type: TokenType::Unknow,
            ptr: None,
            array_index: None,
            modifire: vec![],
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
