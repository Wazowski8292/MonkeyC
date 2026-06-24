use crate::parser::Block;
use std::vec::Vec;

#[derive(Debug, PartialEq, Clone)]
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
            "+" => TokenType::PLUS,
            "-" => TokenType::MINUS,
            "=" => TokenType::EQUALS,

            _ => TokenType::UNKNOW,
        }
    }
}

struct Variable {
    token_type: TokenType,
    value: String,
}

struct Function {
    token_type: TokenType,
    parameters: Vec<TokenType>,
}

enum TableTypes {
    VARIABLE(Variable),
    FUNCTION(Function),
}

pub fn analyze_semantically(stack: Vec<Block>) {
    let table: Vec<TableTypes> = vec![];

    for block in stack.iter() {
        match block {
            Block::Word(w) => {
                let token = TokenType::from_str(w);
                println!("{:?}", token);
            }
            Block::Line(words) => {
                for w in words {
                    let token = TokenType::from_str(w);
                    println!("{:?}", token);
                }
            }
            Block::Multiple(lines) => {
                for line in lines {
                    for w in line {
                        let token = TokenType::from_str(w);
                        println!("{:?}", token);
                    }
                }
            }
            Block::Collection(blocks) => {
                analyze_semantically(blocks.to_vec());
            }
        }
    }
}         