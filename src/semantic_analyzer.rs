use crate::parser::Block;
use std::vec::Vec;

#[derive(Debug, PartialEq, Clone, Copy)]
enum TokenType {
    IF,
    ELSE,

    INT,
    FLOAT,
    BOOL,
    STRING,

    PLUS,
    MINUS,
    EQUALS,
    
    NUMBER,

    FN,
    
    UNKNOW,
}

impl TokenType {
    fn from_str(s: &str) -> Self {
        match s {
            "if" => TokenType::IF,
            "else" => TokenType::ELSE,

            "int" => TokenType::INT,
            "float" => TokenType::FLOAT,
            "str" => TokenType::STRING,
            "bool" => TokenType::BOOL,

            "+" => TokenType::PLUS,
            "-" => TokenType::MINUS,
            "=" => TokenType::EQUALS,

            "fn" => TokenType::FN,

            _ => TokenType::UNKNOW,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Variable {
    token_type: TokenType,
    value: Option<String>,
    name: Option<String>,
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
         matches!(TokenType::from_str(&arg), TokenType::UNKNOW)
    }

    fn finished_definition(&self) -> bool {
        self.name.is_some()
    }

    fn add_arguments(&mut self, argument: String) {
        if !Variable::is_valid_argument(argument.clone()) {
            return;
        }

        if self.name == None {
            self.name = Some(argument);
        } else {
            self.value = Some(argument);
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Function {
    parameters: Option<Vec<TableTypes>>,
    name: Option<String>,
    table: Vec<TableTypes>,
}

impl Types for Function {
    fn new(token: TokenType) -> Self {
        Self {
            parameters: None,
            name: None,
            table: vec![],
        }
    }

    fn is_valid_argument(arg: String) -> bool {
         matches!(TokenType::from_str(&arg), TokenType::UNKNOW)
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
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Reasingment {
    target: usize,
    value: Option<String>,
}

impl Types for Reasingment {
    fn new(token: TokenType) -> Self {
        Self {
            target: 0,
            value: None,
        }
    }
    fn is_valid_argument(arg: String) -> bool {
         matches!(TokenType::from_str(&arg), TokenType::UNKNOW)
    }
    
    fn finished_definition(&self) -> bool {
        true
    }

    fn add_arguments(&mut self, argument: String) {
        if !Reasingment::is_valid_argument(argument.clone()) {
            return;
        }

        self.value = Some(argument.clone()); 
    }
}

#[derive(Debug, PartialEq, Clone)]
enum TableTypes {
    VARIABLE(Variable),
    FUNCTION(Function),
    REASIGNMENT(Reasingment),
    ARGUMENT,
    CONDITIONAL,
}

trait Types {
    fn new(token: TokenType) -> Self;
    fn is_valid_argument(arg: String) -> bool;
    fn finished_definition(&self) -> bool;
    fn add_arguments(&mut self, argument: String);
}

impl TableTypes {
    fn from_token(token: TokenType) -> Self{
        match token {
            TokenType::INT | TokenType::FLOAT | TokenType::STRING | TokenType::BOOL => TableTypes::VARIABLE(Variable::new(token)),
            TokenType::FN => TableTypes::FUNCTION(Function::new(token)),
            TokenType::IF | TokenType::ELSE => TableTypes::CONDITIONAL,
            TokenType::UNKNOW => TableTypes::REASIGNMENT(Reasingment::new(TokenType::UNKNOW)),
            _ => TableTypes::ARGUMENT,
        }
    }

    fn finished_definition(&self) -> bool {
        match self {
            TableTypes::VARIABLE(var) => var.finished_definition(),
            TableTypes::FUNCTION(fun) => fun.finished_definition(),
            TableTypes::REASIGNMENT(asing) => asing.finished_definition(),
            _ => {true},
        }
    }

    fn add_arguments(&mut self, argument: String) {
        match self {
            TableTypes::VARIABLE(var) => { var.add_arguments(argument)}
            TableTypes::FUNCTION(fun) => { fun.add_arguments(argument)}
            TableTypes::REASIGNMENT(reasing) => {reasing.add_arguments(argument)}
            _ => {}
        }
    }
}

struct SemanticAnalyzer {
    table: Vec<TableTypes>,
    error_messages: Vec<String>,
    set_value: bool,
    defining_fn: bool,
    defining_parameters: bool,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            table: vec![],
            error_messages: vec![],
            set_value: false,
            defining_fn: false,
            defining_parameters: false,
        }
    }

    pub fn analyze(&mut self, stack: Vec<Block>) {
        for block in stack.iter() {
            match block {
                Block::Word(word) => {
                    self.tokenize_word(word.to_string());
                }
                Block::Line(words) => {
                    self.tokenize_line(words);
                }
                Block::Multiple(lines) => {
                    for line in lines {
                        self.tokenize_line(line);
                    }
                }
                Block::Collection(blocks) => {
                    let last_is_fn = matches!(self.active_table().last(), Some(TableTypes::FUNCTION(_)));
                    
                    if last_is_fn && !self.defining_fn {
                        self.defining_fn = true;
                        self.defining_parameters = true;
                        self.analyze(blocks.to_vec());
                    } else if self.defining_fn {
                        self.defining_parameters = false;
                        self.analyze(blocks.to_vec());
                        self.defining_fn = false;
                    }
                }
            }
        }
    } 

    fn resolve(&mut self, name: String) -> Option<usize> {
        self.active_table().iter().position(|entry| match entry {
            TableTypes::VARIABLE(v) => v.name == Some(name.clone()),
            TableTypes::FUNCTION(f) => f.name == Some(name.clone()),
            _ => false,
        })
    }

    fn add_entry(&mut self, token: TokenType) {
        let table_type: TableTypes = TableTypes::from_token(token);
        self.active_table().push(table_type)
    }

    fn active_table(&mut self) -> &mut Vec<TableTypes> {
        if self.defining_fn {
            if let Some(TableTypes::FUNCTION(f)) = self.table.last_mut() {
                if self.defining_parameters {
                    if f.parameters.is_none() {
                        f.parameters = Some(vec![]);
                    }
                    return unsafe { &mut *(f.parameters.as_mut().unwrap() as *mut _) };
                }
                return unsafe { &mut *((&mut f.table) as *mut _) };
            }
        }
        &mut self.table
    }

    fn tokenize_word(&mut self, word: String) {
        let token = TokenType::from_str(&word);

        if token == TokenType::EQUALS {
            self.set_value = true;
            return;
        }

        let index = self.resolve(word.clone());
        let is_known = token != TokenType::UNKNOW || index.is_some();
        let last_finished = self.active_table().last().map_or(true, |e| e.finished_definition());

        if self.active_table().is_empty() {
            if token == TokenType::UNKNOW {
                self.error_messages.push(format!("Tried to start the programm with an unknow word {}", word));
            }
            self.add_entry(token);
        } else if !last_finished || self.set_value {
            self.active_table().last_mut().unwrap().add_arguments(word);
            self.set_value = false;
        } else {
            if token != TokenType::UNKNOW {
                self.add_entry(token);
            } else if is_known {
                self.set_value = true;

                self.active_table().push(TableTypes::REASIGNMENT(Reasingment {target: index.expect("Error finding the index of the value to be reasign"), value: None}));
            } else {
                self.error_messages.push(format!("Undefined symbol: '{}'", word));
            }
        }
    }

    fn tokenize_line(&mut self, line: &Vec<String>) {
        for word in line {
            self.tokenize_word(word.to_string());
        }
    }
}

pub fn analyze_semantically(stack: Vec<Block>) {
    let mut semantic_analyzer: SemanticAnalyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze(stack);

    println!("Semantic analyzer table: {:#?}", semantic_analyzer.table);
    println!("Semantic analyzer erros msg: {:#?}", semantic_analyzer.error_messages);
}