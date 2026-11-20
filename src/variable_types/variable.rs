use super::function_calls::FunctionCall;
use crate::semantic_analyzer::TokenType;

pub trait Types {
    fn new(token: TokenType) -> Self;
    fn add_arguments(&mut self, argument: String);
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Var(String),
    FuncCall(FunctionCall),
    Deref(String),
    Ref(String),
    Index(String, String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum PointerType {
    Pointer,
    Reference,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Modifire {
    Constant,
    Public,
    None,
}

impl Modifire {
    pub fn from_token(token: TokenType) -> Modifire {
        match token {
            TokenType::Const => Modifire::Constant,
            TokenType::Public => Modifire::Public,
            _ => Modifire::None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub token_type: TokenType,
    pub value: Option<Vec<Value>>,
    pub name: Option<String>,
    pub ptr: Option<PointerType>,
    pub is_array: bool,
    pub array_size: Option<usize>,
    pub modifire: Vec<Modifire>,
}

impl Variable {
    pub fn pending_call(&self) -> Option<&FunctionCall> {
        match self.value.as_ref()?.last()? {
            Value::FuncCall(fc) => Some(fc),
            _ => None,
        }
    }

    pub fn pending_call_mut(&mut self) -> Option<&mut FunctionCall> {
        match self.value.as_mut()?.last_mut()? {
            Value::FuncCall(fc) => Some(fc),
            _ => None,
        }
    }
}

pub fn parse_array_syntax(s: &str) -> Option<(String, String)> {
    if let (Some(open), Some(close)) = (s.find('['), s.rfind(']')) {
        if open < close {
            let name = s[..open].trim().to_string();
            let index = s[open + 1..close].trim().to_string();
            if !name.is_empty() && !index.is_empty() {
                return Some((name, index));
            }
        }
    }
    None
}

impl Types for Variable {
    fn new(token: TokenType) -> Self {
        Self {
            token_type: token,
            value: None,
            name: None,
            ptr: None,
            is_array: false,
            array_size: None,
            modifire: vec![],
        }
    }

    fn add_arguments(&mut self, argument: String) {
        if self.name.is_none() {
            if let Some((arr_name, size_str)) = parse_array_syntax(&argument) {
                self.name = Some(arr_name);
                self.is_array = true;
                self.array_size = size_str.parse::<usize>().ok();
            } else {
                self.name = Some(argument);
            }
        } else {
            if let Some((arr_name, idx_str)) = parse_array_syntax(&argument) {
                self.value.get_or_insert_with(Vec::new).push(Value::Index(arr_name, idx_str));
            } else {
                self.value.get_or_insert_with(Vec::new).push(Value::Var(argument));
            }
        }
    }
}
