use crate::semantic_analyzer::{TokenType, TableTypes, Scope};

pub trait Types {
    fn new(token: TokenType) -> Self;
    fn is_valid_argument(arg: String) -> bool;
    fn finished_definition(&self) -> bool;
    fn add_arguments(&mut self, argument: String);
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub token_type: TokenType,
    pub value: Option<String>,
    pub name: Option<String>,
}

impl Types for Variable {
    fn new(token: TokenType) -> Self {
        Self {
            token_type: token,
            value: None,
            name: None,
        }
    }

    fn is_valid_argument(arg: String) -> bool {
         TokenType::is_value(TokenType::from_str(&arg))
    }

    fn finished_definition(&self) -> bool {
        self.name.is_some()
    }

    fn add_arguments(&mut self, argument: String) {
        if self.name == None {
            self.name = Some(argument);
        } else {
            *self.value.get_or_insert_with(String::new) += &(" ".to_string() + &argument);
            //self.value = Some(argument);
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub parameters: Option<Vec<TableTypes>>,
    pub name: Option<String>,
    pub table: Vec<TableTypes>,
}

impl Types for Function {
    fn new(_: TokenType) -> Self {
        Self {
            parameters: None,
            name: None,
            table: vec![],
        }
    }

    fn is_valid_argument(arg: String) -> bool {
         matches!(TokenType::from_str(&arg), TokenType::Unknow)
    }

    fn finished_definition(&self) -> bool {
        self.name.is_some()
    }

    fn add_arguments(&mut self, argument: String) {
        if !Function::is_valid_argument(argument.clone()) {
            return;
        }

        if self.name == None {
            self.name = Some(argument.clone());
        } else {
            let mut table_type = TableTypes::from_token(TokenType::from_str(&argument));

            if let TableTypes::Variable(ref mut v) = table_type {
                v.value = Some(argument);
            }
            self.parameters.get_or_insert_with(Vec::new).push(table_type); 
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct Reasingment {
    pub target: usize,
    pub target_scope: Scope,
    pub parameters: Option<Vec<TableTypes>>,
}

impl Types for Reasingment {
    fn new(_: TokenType) -> Self {
        Self {
            target: 0,
            target_scope: Scope::Root,
            parameters: None,
        }
    }
    fn is_valid_argument(arg: String) -> bool {
        TokenType::is_value(TokenType::from_str(&arg))
    }
    
    fn finished_definition(&self) -> bool {
        self.parameters.clone().map_or(0, |p| p.len()) >= 0
    }

    fn add_arguments(&mut self, argument: String) {
        if !Reasingment::is_valid_argument(argument.clone()) {
            return;
        }

        let mut table_type = TableTypes::from_token(TokenType::from_str(&argument));

        if let TableTypes::Variable(ref mut v) = table_type {
            v.value = Some(argument);
        }
        self.parameters.get_or_insert_with(Vec::new).push(table_type); 
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub target: usize,
    pub target_scope: Scope,
    pub parameters: Option<Vec<TableTypes>>,
}

impl Types for FunctionCall {
    fn new(_: TokenType) -> Self {
        Self {
            target: 0,
            target_scope: Scope::Root,
            parameters: None,
        }
    }
    fn is_valid_argument(arg: String) -> bool {
        TokenType::is_value(TokenType::from_str(&arg))
    }
    
    fn finished_definition(&self) -> bool {
        true
    }

    fn add_arguments(&mut self, argument: String) {
        if !FunctionCall::is_valid_argument(argument.clone()) {
            return;
        }

        let mut table_type = TableTypes::from_token(TokenType::from_str(&argument));

        if let TableTypes::Variable(ref mut v) = table_type {
            v.value = Some(argument);
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
        Self {
            condition: vec![],
            table: vec![],
        }
    }

    fn is_valid_argument(_: String) -> bool {
        true
    }

    fn finished_definition(&self) -> bool {
        self.condition.len() > 0
    }

    fn add_arguments(&mut self, argument: String) {
        if !FunctionCall::is_valid_argument(argument.clone()) {
            return;
        }

        let mut table_type = TableTypes::from_token(TokenType::from_str(&argument));

        if let TableTypes::Variable(ref mut v) = table_type {
            v.value = Some(argument);
        }
        self.condition.push(table_type); 
    }
} 

#[derive(Debug, PartialEq, Clone)]
pub struct Loop {
    pub condition: Vec<TableTypes>,
    pub table: Vec<TableTypes>,
}

impl Types for Loop {
    fn new(_: TokenType) -> Self {
        Self {
            condition: vec![],
            table: vec![],
        }
    }

    fn is_valid_argument(_: String) -> bool {
        true
    }

    fn finished_definition(&self) -> bool {
        self.condition.len() > 0
    }

    fn add_arguments(&mut self, argument: String) {
        if !FunctionCall::is_valid_argument(argument.clone()) {
            return;
        }

        let mut table_type = TableTypes::from_token(TokenType::from_str(&argument));

        if let TableTypes::Variable(ref mut v) = table_type {
            v.value = Some(argument);
        }
        self.condition.push(table_type); 
    }
} 