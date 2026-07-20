use crate::semantic_analyzer::{TokenType, TableTypes, Scope};

pub trait Types {
    fn new(token: TokenType) -> Self;
    fn finished_definition(&self) -> bool;
    fn add_arguments(&mut self, argument: String);
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Var(String),
    FuncCall(FunctionCall),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub token_type: TokenType,
    pub value: Option<Vec<Value>>,
    pub name: Option<String>,
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

impl Types for Variable {
    fn new(token: TokenType) -> Self {
        Self {
            token_type: token,
            value: None,
            name: None,
        }
    }

    fn finished_definition(&self) -> bool {
        self.name.is_some()
    }

    fn add_arguments(&mut self, argument: String) {
        if self.name == None {
            self.name = Some(argument);
        } else {
            self.value.get_or_insert_with(Vec::new).push(Value::Var(argument));
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub parameters: Option<Vec<TableTypes>>,
    pub name: Option<String>,
    pub table: Vec<TableTypes>,
    pub return_type: Option<TokenType>,
}

impl Types for Function {
    fn new(_: TokenType) -> Self {
        Self {
            parameters: None,
            name: None,
            table: vec![],
            return_type: None,
        }
    }

    fn finished_definition(&self) -> bool {
        self.name.is_some()
    }

    fn add_arguments(&mut self, argument: String) {
        if self.name == None {
            self.name = Some(argument.clone());
        } else {
            let token = TokenType::from_str(&argument);
            if token == TokenType::Bool || token == TokenType::Char || token == TokenType::String || token == TokenType::Int || token == TokenType::Float {
                self.return_type = Some(token);
            }
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct Reasingment {
    pub target: usize,
    pub target_scope: Scope,
    pub parameters: Option<Vec<TableTypes>>,
    pub name: String,
}

impl Types for Reasingment {
    fn new(_: TokenType) -> Self {
        Self {
            target: 0,
            target_scope: Scope::Root,
            parameters: None,
            name: String::new(),
        }
    }

    fn finished_definition(&self) -> bool {
        self.parameters.clone().map_or(true, |p| !p.is_empty())
    }

    fn add_arguments(&mut self, argument: String) {
        let mut table_type = TableTypes::from_token(TokenType::from_str(&argument));

        match table_type {
            TableTypes::Variable(ref mut v) => v.value.get_or_insert_with(Vec::new).push(Value::Var(argument)),
            TableTypes::Reasingment(ref mut r) => r.name = argument,
            _ => {}
        }
        
        self.parameters.get_or_insert_with(Vec::new).push(table_type); 
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub target: usize,
    pub parameters: Option<Vec<TableTypes>>,
    pub name: String,
}

impl Types for FunctionCall {
    fn new(_: TokenType) -> Self {
        Self {
            target: 0,
            parameters: None,
            name: String::new(),
        }
    }

    fn finished_definition(&self) -> bool {
        true
    }

    fn add_arguments(&mut self, argument: String) {
        let mut table_type = TableTypes::from_token(TokenType::from_str(&argument));

        match table_type {
            TableTypes::Variable(ref mut v) => v.value.get_or_insert_with(Vec::new).push(Value::Var(argument)),
            TableTypes::Reasingment(ref mut r) => r.name = argument,
            _ => {}
        }

        self.parameters.get_or_insert_with(Vec::new).push(table_type); 
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Conditional {
    pub condition: Vec<TableTypes>,
    pub table: Vec<TableTypes>,
}

impl Types for Conditional {
    fn new(_: TokenType) -> Self {
        let mut var = Variable::new(TokenType::BoolLiteral);
        var.name = Some("_".to_string());

        Self {
            condition: vec![TableTypes::Variable(var)],
            table: vec![],
        }
    }

    fn finished_definition(&self) -> bool {
        false
    }

    fn add_arguments(&mut self, argument: String) {
        if let Some(TableTypes::Variable(v)) = self.condition.last_mut() {
            v.add_arguments(argument);
        }
    }
} 

#[derive(Debug, PartialEq, Clone)]
pub struct Loop {
    pub condition: Vec<TableTypes>,
    pub table: Vec<TableTypes>,
}

impl Types for Loop {
    fn new(_: TokenType) -> Self {
        let mut var = Variable::new(TokenType::BoolLiteral);
        var.name = Some("_".to_string());

        Self {
            condition: vec![TableTypes::Variable(var)],
            table: vec![],
        }
    }

    fn finished_definition(&self) -> bool {
        false
    }

    fn add_arguments(&mut self, argument: String) {
        if let Some(TableTypes::Variable(v)) = self.condition.last_mut() {
            v.add_arguments(argument);
        }
    }
} 

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

    fn finished_definition(&self) -> bool {
        false
    }

    fn add_arguments(&mut self, argument: String) {
        if let Some(_) = self.value.clone() {
            self.value.as_mut().unwrap().add_arguments(argument);
        }
    }
} 