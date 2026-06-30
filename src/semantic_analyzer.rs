use crate::parser::{Block, Word};
use crate::variable_types::{Variable, Function, Reasingment, FunctionCall, Types};
use std::vec::Vec;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    If,
    Else,

    Int,
    Float,
    Bool,
    Char,
    String,

    Plus,
    Minus,
    Equals,
    
    IntegerLiteral,
    FloatLiteral,
    CharLiteral,
    StringLiteral,
    BoolLiteral,

    FnLiteral,
    
    Unknow,
}

impl TokenType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "if" => TokenType::If,
            "else" => TokenType::Else,

            "int" => TokenType::Int,
            "float" => TokenType::Float,
            "str" => TokenType::String,
            "bool" => TokenType::Bool,

            "+" => TokenType::Plus,
            "-" => TokenType::Minus,
            "=" => TokenType::Equals,

            "fn" => TokenType::FnLiteral,

            _ if s.parse::<i64>().is_ok() => TokenType::IntegerLiteral,
            _ if s.parse::<f64>().is_ok() => TokenType::FloatLiteral,

            _ => TokenType::Unknow,
        }
    }

    pub fn is_value(token: TokenType) -> bool {
        token == TokenType::Unknow || token == TokenType::IntegerLiteral || token == TokenType::FloatLiteral ||
        token ==TokenType::BoolLiteral || token ==TokenType::StringLiteral || token ==TokenType::CharLiteral
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Scope {
    Root,
    Function,
    Parameter,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TableTypes {
    Variable(Variable),
    Function(Function),
    Reasingment(Reasingment),
    FunctionCall(FunctionCall),
    Argument,
    Conditional,
}

impl TableTypes {
    pub fn from_token(token: TokenType) -> Self{
        match token {
            TokenType::Int | TokenType::Float | TokenType::String | TokenType::Bool | TokenType::IntegerLiteral | TokenType::FloatLiteral => TableTypes::Variable(Variable::new(token)),
            TokenType::FnLiteral => TableTypes::Function(Function::new(token)),
            TokenType::If | TokenType::Else => TableTypes::Conditional,
            TokenType::Unknow => TableTypes::Reasingment(Reasingment::new(TokenType::Unknow)),
            _ => TableTypes::Argument,
        }
    }

    fn finished_definition(&self) -> bool {
        match self {
            TableTypes::Variable(var) => var.finished_definition(),
            TableTypes::Function(fun) => fun.finished_definition(),
            TableTypes::Reasingment(asing) => asing.finished_definition(),
            TableTypes::FunctionCall(fc) => fc.finished_definition(),
            _ => {true},
        }
    }

    fn add_arguments(&mut self, argument: String) {
        match self {
            TableTypes::Variable(var) => { var.add_arguments(argument)}
            TableTypes::Function(fun) => { fun.add_arguments(argument)}
            TableTypes::Reasingment(reasing) => {reasing.add_arguments(argument)}
            TableTypes::FunctionCall(fc) => {fc.add_arguments(argument)}
            _ => {}
        }
    }
}

enum ActiveTable {
    Root,
    FunctionTable,
    FunctionParameters,
    ReassignmentParameters,
    FunctionCallParameters,
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
                    self.tokenize_word(word.clone());
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
                    let last_is_fn = matches!(self.active_table().last(), Some(TableTypes::Function(_)));
                    
                    if last_is_fn && !self.defining_fn {
                        self.defining_fn = true;
                        self.analyze(blocks.to_vec());
                        self.defining_fn = false;
                    } 
                }
                Block::Parameter(blocks) => {
                    let prev_defining_fn = self.defining_fn;
                    let prev_defining_parameters = self.defining_parameters;
                    self.defining_fn = true;
                    self.defining_parameters = true;
                    self.analyze(blocks.to_vec());
                    self.defining_parameters = prev_defining_parameters;
                    self.defining_fn = prev_defining_fn;
                }
            }
        }
    } 

    fn resolve(&mut self, name: String) -> Option<(usize, Scope, bool)> {
        let local_result = self.active_table().iter().enumerate().find_map(|(idx, entry)| match entry {
            TableTypes::Variable(v) if v.name == Some(name.clone()) => Some((idx, false)),
            TableTypes::Function(f) if f.name == Some(name.clone()) => Some((idx, true)),
            _ => None,
        });

        if let Some((idx, is_func)) = local_result {
            if !self.defining_fn {
                return Some((idx, Scope::Root, is_func));
            }
            return Some((idx, Scope::Function, is_func));
        }

        if self.defining_fn {
            if let Some(TableTypes::Function(f)) = self.table.last() {
                if let Some(params) = &f.parameters {
                    let param_result = params.iter().enumerate().find_map(|(idx, entry)| match entry {
                        TableTypes::Variable(v) if v.name == Some(name.clone()) => Some((idx, false)),
                        _ => None,
                    });
                    if let Some((idx, is_func)) = param_result {
                        return Some((idx, Scope::Parameter, is_func));
                    }
                }
            }
        }
        
        let root_result = self.table.iter().enumerate().find_map(|(idx, entry)| match entry {
            TableTypes::Variable(v) if v.name == Some(name.clone()) => Some((idx, false)),
            TableTypes::Function(f) if f.name == Some(name.clone()) => Some((idx, true)),
            _ => None,
        });

        root_result.map(|(idx, is_func)| (idx, Scope::Root, is_func))
    }

    fn add_entry(&mut self, token: TokenType) {
        let table_type: TableTypes = TableTypes::from_token(token);
        self.active_table().push(table_type)
    }

    fn active_table(&mut self) -> &mut Vec<TableTypes> {
        let which = if self.defining_fn {
            match self.table.last() {
                Some(TableTypes::Function(func)) => {
                    if self.defining_parameters {
                        if let Some(TableTypes::FunctionCall(_)) = func.table.last() {
                            ActiveTable::FunctionTable
                        } else {
                            ActiveTable::FunctionParameters
                        }
                    } else {
                        ActiveTable::FunctionTable
                    }
                }
                Some(TableTypes::Reasingment(_)) => ActiveTable::ReassignmentParameters,
                _ => ActiveTable::Root,
            }
        } else {
            ActiveTable::Root
        };

        match which {
            ActiveTable::Root => &mut self.table,
            ActiveTable::FunctionTable => {
                if let Some(TableTypes::Function(f)) = self.table.last_mut() {
                    &mut f.table
                } else { unreachable!() }
            }
            ActiveTable::FunctionParameters => {
                if let Some(TableTypes::Function(f)) = self.table.last_mut() {
                    f.parameters.get_or_insert_with(Vec::new)
                } else { unreachable!() }
            }
            // TODO also should eliminate but not now
            ActiveTable::ReassignmentParameters => {
                if let Some(TableTypes::Reasingment(r)) = self.table.last_mut() {
                    r.parameters.get_or_insert_with(Vec::new)
                } else { unreachable!()}
            } 
            _ => {todo!()}
        }
    }

    fn tokenize_word(&mut self, word: Word) {
        let token = TokenType::from_str(&word.word);

        if token == TokenType::Equals {
            self.set_value = true;
            return;
        }

        let index = self.resolve(word.word.clone());
        let last_finished = self.active_table().last().map_or(true, |e| e.finished_definition());
        
        let mut last_completed = false;
        let mut is_fc = false;
        let mut fc_target = 0;
        let mut fc_target_scope = Scope::Root;
        let mut fc_params_len = 0;

        match self.active_table().last() {
            Some(TableTypes::Reasingment(r)) => {
                last_completed = r.parameters.as_ref().map_or(0, |p| p.len()) >= 1;
            }
            Some(TableTypes::FunctionCall(fc)) => {
                is_fc = true;
                fc_target = fc.target;
                fc_target_scope = fc.target_scope;
                fc_params_len = fc.parameters.as_ref().map_or(0, |p| p.len());
            }
            _ => {}
        }

        let mut expected_fc_params = 0;
        if is_fc {
            let expected_params = match fc_target_scope {
                Scope::Root =>{ 
                    if let Some(TableTypes::Function(f)) = self.table.get(fc_target) {
                        f.parameters.as_ref().map_or(0, |p| p.len())
                    } else { 0 }
                },
                Scope::Function => {
                    if let Some(TableTypes::Function(f)) = self.table.last() {
                        if let Some(TableTypes::Function(inner)) = f.table.get(fc_target) {
                            inner.parameters.as_ref().map_or(0, |p| p.len())
                        } else { 0 }
                    } else { 0 }
                },
                _ => 0,
            };
            expected_fc_params = expected_params;
            last_completed = false;
        }

        let in_reasignment = matches!(self.active_table().last(), Some(TableTypes::Reasingment(_)));
        let in_function_call = matches!(self.active_table().last(), Some(TableTypes::FunctionCall(_))) && self.defining_parameters;
        
        let in_call_or_reasign = (in_reasignment || in_function_call) && !last_completed;
        
        if !last_finished || self.set_value || in_call_or_reasign {
            self.handle_argument(word, token, index, in_reasignment, in_function_call, expected_fc_params, fc_params_len);
        } else {
            self.handle_new_entry(word, token, index);
        }
    }

    fn handle_argument(&mut self, word: Word, token: TokenType, index: Option<(usize, Scope, bool)>, in_reasignment: bool, in_function_call: bool, expected_fc_params: usize, fc_params_len: usize) {
        self.set_value = false;
        let in_call_or_reasign = in_reasignment || in_function_call;

        if in_function_call && fc_params_len >= expected_fc_params {
            self.error_messages.push(format!("Too many arguments for function call. Expected {}, got {}; Line: {}; Char pos: {}", expected_fc_params, fc_params_len + 1, word.line.unwrap_or(0), word.char_num.unwrap_or(0)));
        }

        if in_call_or_reasign && token == TokenType::Unknow && index.is_none() {
            self.error_messages.push(format!("Undefined symbol: {}; Line: {}; Char pos: {}", word.word, word.line.unwrap_or(0), word.char_num.unwrap_or(0)));
            return;
        }

        let new_entry = match self.active_table().last_mut() {
            Some(entry) => entry,
            None => {
                self.error_messages.push(("There wasn't a last entry").to_string()); 
                return; 
            },
        };
        new_entry.add_arguments(word.word.clone());
        
        if let Some((idx, scope, is_func)) = index {
            let last_param = match new_entry {
                TableTypes::Reasingment(r) => r.parameters.as_mut().and_then(|p| p.last_mut()),
                TableTypes::FunctionCall(fc) => fc.parameters.as_mut().and_then(|p| p.last_mut()),
                _ => None,
            };

            if let Some(last) = last_param {
                if let TableTypes::Reasingment(v) = last {
                    if is_func {
                        *last = TableTypes::FunctionCall(FunctionCall {
                            target: idx,
                            target_scope: scope,
                            parameters: None,
                        });
                    } else {
                        v.target = idx;
                        v.target_scope = scope;
                    }
                }
            }
        }
    }

    fn handle_new_entry(&mut self, word: Word, token: TokenType, index: Option<(usize, Scope, bool)>) {
        if !TokenType::is_value(token) {
            self.add_entry(token);
        } else if index.is_some() {
            self.set_value = true;

            let (idx, scope, is_func) = index.expect("Error finding the index of the value to be reasign");
            
            if is_func {
                let func_call = FunctionCall {
                    target: idx,
                    target_scope: scope,
                    parameters: None,
                };
                self.active_table().push(TableTypes::FunctionCall(func_call));
            } else {
                let reasign = Reasingment {
                    target: idx,
                    target_scope: scope,
                    parameters: None,
                };
                self.active_table().push(TableTypes::Reasingment(reasign));
            }
        } else {
            self.error_messages.push(format!("Undefined symbol: {}; Line: {}:{}", word.word, word.line.unwrap_or(0), word.char_num.unwrap_or(0)));
        }
    }

    fn tokenize_line(&mut self, line: &Vec<Word>) {
        for word in line {
            self.tokenize_word(word.clone());
        }
    }
}

pub fn analyze_semantically(stack: Vec<Block>) {
    let mut semantic_analyzer: SemanticAnalyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze(stack);

    println!("Semantic analyzer table: {:#?}", semantic_analyzer.table);
    println!("Semantic analyzer erros msg: {:#?}", semantic_analyzer.error_messages);
}