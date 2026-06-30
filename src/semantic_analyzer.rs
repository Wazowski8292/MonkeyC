use crate::parser::{Block, Word};
use std::vec::Vec;

#[derive(Debug, PartialEq, Clone, Copy)]
enum TokenType {
    IF,
    ELSE,

    INT,
    FLOAT,
    BOOL,
    CHAR,
    STRING,

    PLUS,
    MINUS,
    EQUALS,
    
    INTEGER_LITERAL,
    FLOAT_LITERAL,
    CHAR_LITERAL,
    STRING_LITERAL,
    BOOL_LITERAL,

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

            _ if s.parse::<i64>().is_ok() => TokenType::INTEGER_LITERAL,
            _ if s.parse::<f64>().is_ok() => TokenType::FLOAT_LITERAL,

            _ => TokenType::UNKNOW,
        }
    }

    fn is_value(token: TokenType) -> bool {
        token == TokenType::UNKNOW || token == TokenType::INTEGER_LITERAL || token == TokenType::FLOAT_LITERAL ||
        token ==TokenType::BOOL_LITERAL || token ==TokenType::STRING_LITERAL || token ==TokenType::CHAR_LITERAL
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
         TokenType::is_value(TokenType::from_str(&arg))
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
    fn new(_: TokenType) -> Self {
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

#[derive(Debug, PartialEq, Clone, Copy)]
enum Scope {
    Root,
    Function,
    Parameter,
}

#[derive(Debug, PartialEq, Clone)]
struct Reasingment {
    target: usize,
    target_scope: Scope,
    parameters: Option<Vec<TableTypes>>,
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
        true
    }

    fn add_arguments(&mut self, argument: String) {
        if !Reasingment::is_valid_argument(argument.clone()) {
            return;
        }

        let mut table_type = TableTypes::from_token(TokenType::from_str(&argument));

        if let TableTypes::VARIABLE(ref mut v) = table_type {
            v.value = Some(argument);
        }
        self.parameters.get_or_insert_with(Vec::new).push(table_type); 
    }
}

#[derive(Debug, PartialEq, Clone)]
struct FunctionCall {
    target: usize,
    target_scope: Scope,
    parameters: Option<Vec<TableTypes>>,
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

        if let TableTypes::VARIABLE(ref mut v) = table_type {
            v.value = Some(argument);
        }
        self.parameters.get_or_insert_with(Vec::new).push(table_type); 
    }
}

#[derive(Debug, PartialEq, Clone)]
enum TableTypes {
    VARIABLE(Variable),
    FUNCTION(Function),
    REASIGNMENT(Reasingment),
    FUNCTION_CALL(FunctionCall),
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
            TokenType::INT | TokenType::FLOAT | TokenType::STRING | TokenType::BOOL | TokenType::INTEGER_LITERAL | TokenType::FLOAT_LITERAL => TableTypes::VARIABLE(Variable::new(token)),
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
            TableTypes::FUNCTION_CALL(fc) => fc.finished_definition(),
            _ => {true},
        }
    }

    fn add_arguments(&mut self, argument: String) {
        match self {
            TableTypes::VARIABLE(var) => { var.add_arguments(argument)}
            TableTypes::FUNCTION(fun) => { fun.add_arguments(argument)}
            TableTypes::REASIGNMENT(reasing) => {reasing.add_arguments(argument)}
            TableTypes::FUNCTION_CALL(fc) => {fc.add_arguments(argument)}
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
                    let last_is_fn = matches!(self.active_table().last(), Some(TableTypes::FUNCTION(_)));
                    
                    if last_is_fn && !self.defining_fn {
                        self.defining_fn = true;
                        self.analyze(blocks.to_vec());
                        self.defining_fn = false;
                    } 
                }
                Block::Parameter(blocks) => {
                    self.defining_fn = true;
                    self.defining_parameters = true;
                    self.analyze(blocks.to_vec());
                    self.defining_parameters = false;
                    self.defining_fn = false;
                }
            }
        }
    } 

    fn resolve(&mut self, name: String) -> Option<(usize, Scope, bool)> {
        let local_result = self.active_table().iter().enumerate().find_map(|(idx, entry)| match entry {
            TableTypes::VARIABLE(v) if v.name == Some(name.clone()) => Some((idx, false)),
            TableTypes::FUNCTION(f) if f.name == Some(name.clone()) => Some((idx, true)),
            _ => None,
        });

        if let Some((idx, is_func)) = local_result {
            if !self.defining_fn {
                return Some((idx, Scope::Root, is_func));
            }
            return Some((idx, Scope::Function, is_func));
        }

        if self.defining_fn {
            if let Some(TableTypes::FUNCTION(f)) = self.table.last() {
                if let Some(params) = &f.parameters {
                    let param_result = params.iter().enumerate().find_map(|(idx, entry)| match entry {
                        TableTypes::VARIABLE(v) if v.name == Some(name.clone()) => Some((idx, false)),
                        _ => None,
                    });
                    if let Some((idx, is_func)) = param_result {
                        return Some((idx, Scope::Parameter, is_func));
                    }
                }
            }
        }
        
        let root_result = self.table.iter().enumerate().find_map(|(idx, entry)| match entry {
            TableTypes::VARIABLE(v) if v.name == Some(name.clone()) => Some((idx, false)),
            TableTypes::FUNCTION(f) if f.name == Some(name.clone()) => Some((idx, true)),
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
                Some(TableTypes::FUNCTION(func)) => {
                    if self.defining_parameters {
                        if let Some(TableTypes::FUNCTION_CALL(_)) = func.table.last() {
                            ActiveTable::FunctionTable
                        } else {
                            ActiveTable::FunctionParameters
                        }
                    } else {
                        ActiveTable::FunctionTable
                    }
                }
                Some(TableTypes::REASIGNMENT(_)) => ActiveTable::ReassignmentParameters,
                _ => ActiveTable::Root,
            }
        } else {
            ActiveTable::Root
        };

        match which {
            ActiveTable::Root => &mut self.table,
            ActiveTable::FunctionTable => {
                if let Some(TableTypes::FUNCTION(f)) = self.table.last_mut() {
                    &mut f.table
                } else { unreachable!() }
            }
            ActiveTable::FunctionParameters => {
                if let Some(TableTypes::FUNCTION(f)) = self.table.last_mut() {
                    f.parameters.get_or_insert_with(Vec::new)
                } else { unreachable!() }
            }
            // TODO also should eliminate but not now
            ActiveTable::ReassignmentParameters => {
                if let Some(TableTypes::REASIGNMENT(r)) = self.table.last_mut() {
                    r.parameters.get_or_insert_with(Vec::new)
                } else { unreachable!()}
            } 
            _ => {todo!()}
        }
    }

    fn tokenize_word(&mut self, word: Word) {
        let token = TokenType::from_str(&word.word);

        if token == TokenType::EQUALS {
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
            Some(TableTypes::REASIGNMENT(r)) => {
                last_completed = r.parameters.as_ref().map_or(0, |p| p.len()) >= 1;
            }
            Some(TableTypes::FUNCTION_CALL(fc)) => {
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
                    if let Some(TableTypes::FUNCTION(f)) = self.table.get(fc_target) {
                        f.parameters.as_ref().map_or(0, |p| p.len())
                    } else { 0 }
                },
                Scope::Function => {
                    if let Some(TableTypes::FUNCTION(f)) = self.table.last() {
                        if let Some(TableTypes::FUNCTION(inner)) = f.table.get(fc_target) {
                            inner.parameters.as_ref().map_or(0, |p| p.len())
                        } else { 0 }
                    } else { 0 }
                },
                _ => 0,
            };
            expected_fc_params = expected_params;
            last_completed = false;
        }

        let in_reasignment = matches!(self.active_table().last(), Some(TableTypes::REASIGNMENT(_)));
        let in_function_call = matches!(self.active_table().last(), Some(TableTypes::FUNCTION_CALL(_)));
        
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

        if in_call_or_reasign && token == TokenType::UNKNOW && index.is_none() {
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
                TableTypes::REASIGNMENT(r) => r.parameters.as_mut().and_then(|p| p.last_mut()),
                TableTypes::FUNCTION_CALL(fc) => fc.parameters.as_mut().and_then(|p| p.last_mut()),
                _ => None,
            };

            if let Some(last) = last_param {
                if let TableTypes::REASIGNMENT(v) = last {
                    if is_func {
                        *last = TableTypes::FUNCTION_CALL(FunctionCall {
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
        let is_known = token != TokenType::UNKNOW || index.is_some();

        if !TokenType::is_value(token) {
            self.add_entry(token);
        } else if is_known {
            self.set_value = true;

            let (idx, scope, is_func) = index.expect("Error finding the index of the value to be reasign");
            
            if is_func {
                let func_call = FunctionCall {
                    target: idx,
                    target_scope: scope,
                    parameters: None,
                };
                self.active_table().push(TableTypes::FUNCTION_CALL(func_call));
            } else {
                let reasign = Reasingment {
                    target: idx,
                    target_scope: scope,
                    parameters: None,
                };
                self.active_table().push(TableTypes::REASIGNMENT(reasign));
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