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
    fn is_valid_argument(arg: TokenType) -> bool {
         matches!(arg, TokenType::UNKNOW)
    }

    fn finished_definition(&self) -> bool {
        self.name.is_some()
    }
}

impl Variable {
    fn add_argument(&mut self, argument: String) {
        println!("argument to add {}", argument);

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
    parameters: Vec<TokenType>,
}

#[derive(Debug)]
enum TableTypes {
    VARIABLE(Variable),
    FUNCTION(Function),
    ARGUMENT,
    CONDITIONAL,
}

trait Types {
    fn is_valid_argument(arg: TokenType) -> bool;
    fn finished_definition(&self) -> bool;
}

impl TableTypes {
    fn from_token(token: TokenType) -> Self{
        match token {
            TokenType::INT | TokenType::FLOAT | TokenType::STRING | TokenType::BOOL => TableTypes::VARIABLE(Variable { token_type: token, value: None, name: None}),
            TokenType::IF | TokenType::ELSE => TableTypes::CONDITIONAL,
            _ => TableTypes::ARGUMENT,
        }
    }

    fn finished_definition(&self) -> bool {
        match self {
            TableTypes::VARIABLE(var) => var.finished_definition(),
            _ => {true},
        }
    }
}

struct SemanticAnalyzer {
    table: Vec<TableTypes>,
    error_messages: Vec<String>,
    set_value: bool
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            table: vec![],
            error_messages: vec![],
            set_value: false,
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
                    self.analyze(blocks.to_vec());
                }
            }
        }
    } 

    fn add_entry(&mut self, token: TokenType) {
        let table_type: TableTypes = TableTypes::from_token(token);
        self.table.push(table_type)
    }

    fn tokenize_word(&mut self, word: String) {
        let token = TokenType::from_str(&word);

        if token == TokenType::EQUALS {
            self.set_value = true;
            return;
        }

        if let Some(last_entry) = self.table.last_mut() {
            if !last_entry.finished_definition() || self.set_value {
                match last_entry {
                    TableTypes::VARIABLE(var) => { var.add_argument(word)}
                    _ => {}
                }
                self.set_value = false;

            } else {
                self.add_entry(token);
            }
        } else {
            if token == TokenType::UNKNOW {
                self.error_messages.push(format!("Tried to start the programm with an unknow word {}", word));
            } 

            self.add_entry(token);
            return;
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