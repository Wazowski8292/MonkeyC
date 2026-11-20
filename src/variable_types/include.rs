use crate::semantic_analyzer::TokenType;
use super::variable::Types;

#[derive(Debug, PartialEq, Clone)]
pub struct Include {
    pub file: String,
    pub modules: Vec<String>,
    //pub line: usize,
    //pub char_pos: usize,
}

impl Types for Include {
    fn new(_: TokenType) -> Self {
        Self {
            file: String::new(),
            modules: vec![],
            //line: 0,
            //char_pos: 0,
        }
    }

    fn add_arguments(&mut self, argument: String) {
        if self.file.is_empty() {
            self.file = argument;
        } else {
            self.modules.push(argument);
        }
    }
}
