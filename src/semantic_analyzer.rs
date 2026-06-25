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

#[derive(Debug)]
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

    fn is_valid_argument(arg: TokenType) -> bool {
         matches!(arg, TokenType::UNKNOW)
    }

    fn finished_definition(&self) -> bool {
        self.name.is_some()
    }

    fn add_arguments(&mut self, argument: String) {
        if self.name == None {
            self.name = Some(argument);
        } else {
            self.value = Some(argument);
        }
    }
}

#[derive(Debug)]
struct Function {
    token_type: TokenType,
    parameters: Vec<String>,
    name: Option<String>,
    table: Vec<TableTypes>,
}

impl Types for Function {
    fn new(token: TokenType) -> Self {
        Self {
            token_type: token,
            parameters: vec![],
            name: None,
            table: vec![],
        }
    }

    fn is_valid_argument(arg: TokenType) -> bool {
         matches!(arg, TokenType::UNKNOW)
    }

    fn finished_definition(&self) -> bool {
        self.name.is_some()
    }

    fn add_arguments(&mut self, argument: String) {
        if self.name == None {
            self.name = Some(argument);
        } else {
            self.parameters.push(argument);
        }
    }
}

#[derive(Debug)]
enum TableTypes {
    VARIABLE(Variable),
    FUNCTION(Function),
    ARGUMENT,
    CONDITIONAL,
}

trait Types {
    fn new(token: TokenType) -> Self;
    fn is_valid_argument(arg: TokenType) -> bool;
    fn finished_definition(&self) -> bool;
    fn add_arguments(&mut self, argument: String);
}

impl TableTypes {
    fn from_token(token: TokenType) -> Self{
        match token {
            TokenType::INT | TokenType::FLOAT | TokenType::STRING | TokenType::BOOL => TableTypes::VARIABLE(Variable::new(token)),
            TokenType::FN => TableTypes::FUNCTION(Function::new(token)),
            TokenType::IF | TokenType::ELSE => TableTypes::CONDITIONAL,
            _ => TableTypes::ARGUMENT,
        }
    }

    fn finished_definition(&self) -> bool {
        match self {
            TableTypes::VARIABLE(var) => var.finished_definition(),
            TableTypes::FUNCTION(fun) => fun.finished_definition(),
            _ => {true},
        }
    }

    fn add_arguments(&mut self, argument: String) {
        match self {
            TableTypes::VARIABLE(var) => { var.add_arguments(argument)}
            TableTypes::FUNCTION(fun) => { fun.add_arguments(argument)}
            _ => {}
        }
    }
}

struct SemanticAnalyzer {
    table: Vec<TableTypes>,
    error_messages: Vec<String>,
    set_value: bool,
    last_resolved_index: Option<usize>,
    defining_fn: bool,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            table: vec![],
            error_messages: vec![],
            set_value: false,
            last_resolved_index: None,
            defining_fn: false,
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
                    self.defining_fn = matches!(self.active_table().last(), Some(TableTypes::FUNCTION(_)));

                    self.analyze(blocks.to_vec());

                    self.defining_fn = false;
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
            if let Some(idx) = self.last_resolved_index {
                self.active_table()[idx].add_arguments(word);
                self.last_resolved_index = None;
            } else {
                self.active_table().last_mut().unwrap().add_arguments(word);
            }
            self.set_value = false;
        } else {
            if token != TokenType::UNKNOW {
                self.add_entry(token);
            } else if is_known {
                self.set_value = true;
                self.last_resolved_index = index;
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