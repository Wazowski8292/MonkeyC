use crate::parser::{Block, Word};
use crate::variable_types::{Variable, Function, Reasingment, FunctionCall, Conditional, Loop, Types};
use std::vec::Vec;
use crate::enbeded_funcs::FUNCTIONS;

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
    Multiplication,
    Division,
    Equals,

    LogicalEquals,
    LogicalAnd,
    LogicalOr,
    Not,

    RightBitShift,
    LeftBitShift,
    And,
    Or,
    
    IntegerLiteral,
    FloatLiteral,
    CharLiteral,
    StringLiteral,
    BoolLiteral,

    FnLiteral,

    WhileLoop,
    
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
            "char" => TokenType::Char,

            "+" => TokenType::Plus,
            "-" => TokenType::Minus,
            "*" => TokenType::Multiplication,
            "/" => TokenType::Division,
            "=" => TokenType::Equals,

            "==" => TokenType::LogicalEquals,
            "&&" => TokenType::LogicalAnd,
            "||" => TokenType::LogicalOr,
            "!" => TokenType::Not,

            ">>" => TokenType::RightBitShift,
            "<<" => TokenType::LeftBitShift,
            "&" => TokenType::And,
            "|" => TokenType::Or,

            "fn" => TokenType::FnLiteral,

            "while" => TokenType::WhileLoop,

            _ if s.parse::<i64>().is_ok() => TokenType::IntegerLiteral,
            _ if s.parse::<f64>().is_ok() => TokenType::FloatLiteral,
            _ if s.len() >= 2 && s.starts_with('"') && s.ends_with('"') => TokenType::StringLiteral,
            _ if s.len() >= 3 && s.starts_with('\'') && s.ends_with('\'') => TokenType::CharLiteral,
            "true" | "false" => TokenType::BoolLiteral,

            _ => TokenType::Unknow,
        }
    }

    pub fn is_value(token: TokenType) -> bool {
        token == TokenType::Unknow || token == TokenType::IntegerLiteral || token == TokenType::FloatLiteral ||
        token == TokenType::BoolLiteral || token ==TokenType::StringLiteral || token ==TokenType::CharLiteral
    }

    pub fn is_operator(token: TokenType) -> bool {
        TokenType::is_aritmetic_operator(token.clone()) || TokenType::is_binary_operator(token.clone()) ||
        TokenType::is_logical_operator(token.clone())
    }

    pub fn is_logical_operator(token: TokenType) -> bool {
        token == TokenType::LogicalEquals || token == TokenType::LogicalAnd || token == TokenType::LogicalOr || token == TokenType::Not
    }
    
    pub fn is_aritmetic_operator(token: TokenType) -> bool {
        token == TokenType::Plus || token == TokenType::Minus || token == TokenType::Multiplication || token == TokenType::Division || token == TokenType::Equals 
    }

    pub fn is_binary_operator(token: TokenType) -> bool {
        token == TokenType::RightBitShift || token == TokenType::LeftBitShift || token == TokenType::And || token == TokenType::Or 
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
    Conditional(Conditional),
    Loop(Loop),
    Unknown,
}

impl TableTypes {
    pub fn from_token(token: TokenType) -> Self{
        if TokenType::is_operator(token.clone()) {
            return TableTypes::Variable(Variable::new(token));
        }

        match token {
            TokenType::Int | TokenType::Float | TokenType::String | TokenType::Bool | TokenType::Char | TokenType::IntegerLiteral | TokenType::FloatLiteral | TokenType::BoolLiteral | TokenType::StringLiteral | TokenType::CharLiteral  => TableTypes::Variable(Variable::new(token)),
            TokenType::FnLiteral => TableTypes::Function(Function::new(token)),
            TokenType::If | TokenType::Else => TableTypes::Conditional(Conditional::new(token)),
            TokenType::WhileLoop => TableTypes::Loop(Loop::new(token)),
            TokenType::Unknow => TableTypes::Reasingment(Reasingment::new(TokenType::Unknow)),
            _ => TableTypes::Unknown,
        }
    }

    fn finished_definition(&self) -> bool {
        match self {
            TableTypes::Variable(var) => var.finished_definition(),
            TableTypes::Function(fun) => fun.finished_definition(),
            TableTypes::Reasingment(asing) => asing.finished_definition(),
            TableTypes::FunctionCall(fc) => fc.finished_definition(),
            TableTypes::Conditional(con) => con.finished_definition(),
            TableTypes::Loop(while_loop) => while_loop.finished_definition(),
            _ => {true},
        }
    }

    fn add_arguments(&mut self, argument: String) {
        match self {
            TableTypes::Variable(var) => var.add_arguments(argument),
            TableTypes::Function(fun) => fun.add_arguments(argument),
            TableTypes::Reasingment(reasing) => reasing.add_arguments(argument),
            TableTypes::FunctionCall(fc) => fc.add_arguments(argument),
            TableTypes::Conditional(con) => con.add_arguments(argument),
            TableTypes::Loop(while_loop) => while_loop.add_arguments(argument),
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
    max_nesting: usize,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            table: vec![],
            error_messages: vec![],
            set_value: false,
            defining_fn: false,
            defining_parameters: false,
            max_nesting: 1,
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
                    self.max_nesting += 1;
                    let last_is_fn = matches!(self.active_table().last(), Some(TableTypes::Function(_)));
                    
                    if last_is_fn && !self.defining_fn {
                        self.defining_fn = true;
                        self.analyze(blocks.to_vec());
                        self.defining_fn = false;
                    } else {
                        self.analyze(blocks.to_vec());
                    }

                    self.max_nesting -= 1;
                }
                Block::Parameter(blocks) => {
                    let prev_defining_fn = self.defining_fn;
                    //let prev_defining_parameters = self.defining_parameters;
                    self.defining_fn = true;
                    self.defining_parameters = true;
                    self.analyze(blocks.to_vec());
                    self.defining_parameters = false;
                    self.defining_fn = prev_defining_fn;
                }
            }
        }
    } 

    fn resolve(&mut self, name: String) -> Option<(usize, Scope, bool)> {
        if let Some(result) = Self::resolve_in_chain( &name, &mut self.table, 0, self.max_nesting, self.defining_parameters) {
            return Some(result);
        } else {
            return self.resolve_in_parameters(&name);
        }
    }

    fn is_enbeded_func(name: String) -> bool{
        for func in FUNCTIONS.iter() {
            if func.name == name {
                println!("Found print");
                return true;
            }
        }

        false
    }

    fn find_in_level(name: &str, table: &Vec<TableTypes>, nest_level: usize) -> Option<(usize, Scope, bool)> {
        table.iter().enumerate().find_map(|(idx, entry)| match entry {
            TableTypes::Variable(v) if v.name.as_deref() == Some(name) => {
                Some((idx, Self::scope_for_level(nest_level), false))
            }
            TableTypes::Function(f) if f.name.as_deref() == Some(name) => {
                Some((idx, Self::scope_for_level(nest_level), true))
            }
            _ if Self::is_enbeded_func(name.to_string()) => Some((0, Scope::Function, true)),
            _ => None,
        })
    }

    fn scope_for_level(nest_level: usize) -> Scope {
        if nest_level == 0 { 
            Scope::Root 
        } else {
            Scope::Function 
        }
    }

    fn should_descend(table: &Vec<TableTypes>, current_nest_level: usize, max_nesting: usize, defining_parameters: bool) -> bool {
        let has_child = matches!(
            table.last(),
            Some(TableTypes::Function(_)) | Some(TableTypes::Conditional(_)) | Some(TableTypes::Loop(_))
        );
        if !has_child {
            return false;
        }

        let nesting_exhausted = (current_nest_level + 1 > max_nesting) && !defining_parameters;
        let blocked_by_params = defining_parameters
            && matches!(table.last(), Some(TableTypes::Conditional(_)) | Some(TableTypes::Loop(_)));

        !nesting_exhausted && !blocked_by_params
    }

    fn descend_and_resolve(name: &str, table: &mut Vec<TableTypes>, current_nest_level: usize, max_nesting: usize, defining_parameters: bool) -> Option<(usize, Scope, bool)> {
        match table.last_mut().unwrap() {
            TableTypes::Function(func) => {
                if defining_parameters && func.table.is_empty() {
                    None
                } else {
                    Self::resolve_in_chain(name, &mut func.table, current_nest_level + 1, max_nesting, defining_parameters)
                }
            }
            TableTypes::Conditional(con) => {
                Self::resolve_in_chain(name, &mut con.table, current_nest_level + 1, max_nesting, defining_parameters)
            }
            TableTypes::Loop(while_loop) => {
                Self::resolve_in_chain(name, &mut while_loop.table, current_nest_level + 1, max_nesting, defining_parameters)
            }
            _ => unreachable!(),
        }
    }

    fn resolve_in_chain( name: &str, table: &mut Vec<TableTypes>, current_nest_level: usize, max_nesting: usize, defining_parameters: bool) -> Option<(usize, Scope, bool)> {
        if Self::should_descend(table, current_nest_level, max_nesting, defining_parameters) {
            if let Some(found) = Self::descend_and_resolve(name, table, current_nest_level, max_nesting, defining_parameters) {
                return Some(found);
            }
        }

        Self::find_in_level(name, table, current_nest_level)
    }

    fn resolve_in_parameters(&self, name: &str) -> Option<(usize, Scope, bool)> {
        if !self.defining_fn {
            return None;
        }

        let Some(TableTypes::Function(f)) = self.table.last() else {
            return None;
        };
        let params = f.parameters.as_ref()?;

        params.iter().enumerate().find_map(|(idx, entry)| match entry {
            TableTypes::Variable(v) if v.name.as_deref() == Some(name) => Some((idx, Scope::Parameter, false)),
            _ => None,
        })
    }

    fn add_entry(&mut self, token: TokenType) {
        let table_type: TableTypes = TableTypes::from_token(token);
        self.active_table().push(table_type)
    }

    fn active_table(&mut self) -> &mut Vec<TableTypes> {
        if self.defining_fn {
            Self::desend_table(self.defining_parameters, &mut self.table, 1, self.max_nesting)
        } else {
            &mut self.table
        }
    }

    fn desend_table(defining_parameters: bool, last_table: &mut Vec<TableTypes>, current_nest_level: usize, max_nesting: usize) -> &mut Vec<TableTypes> {
        
        let has_child = matches!(
            last_table.last(),
            Some(TableTypes::Function(_)) | Some(TableTypes::Conditional(_)) | Some(TableTypes::Loop(_))
        );

        if !has_child || (current_nest_level + 1 > max_nesting ) && !defining_parameters {
            return last_table;
        }

        if defining_parameters && max_nesting == current_nest_level{
            if let Some(TableTypes::Conditional(_)) = last_table.last() {
                return last_table;
            } else if let Some(TableTypes::Loop(_)) = last_table.last() {
                return last_table;
            }
        }

        match last_table.last_mut().unwrap() {
            TableTypes::Function(func) => {
                if defining_parameters && func.table.is_empty() {
                    func.parameters.get_or_insert_with(Vec::new)
                } else {
                    Self::desend_table(defining_parameters, &mut func.table, current_nest_level + 1, max_nesting)
                }
            },
            TableTypes::Conditional(con) => {
                Self::desend_table(defining_parameters, &mut con.table, current_nest_level + 1, max_nesting)
            },
            TableTypes::Loop(while_loop) => {
                Self::desend_table(defining_parameters, &mut while_loop.table, current_nest_level + 1, max_nesting)
            },
            _ => unreachable!(),
        }
    }

    fn tokenize_word(&mut self, word: Word) {
        let token = TokenType::from_str(&word.word);

        self.set_value |= TokenType::is_operator(token.clone());

        if token == TokenType::Equals {
            return;
        }

        let index = self.resolve(word.word.clone());
        let mut last_finished = self.active_table().last().map_or(true, |e| e.finished_definition());
        
        let mut is_fc = false;
        let mut fc_target = 0;
        let mut fc_target_scope = Scope::Root;
        let mut fc_params_len = 0;

        let defining_parameters = self.defining_parameters;

        match self.active_table().last() {
            Some(TableTypes::FunctionCall(fc)) if defining_parameters => {
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
            last_finished = false;
        }

        let in_reasignment = matches!(self.active_table().last(), Some(TableTypes::Reasingment(_)));
        let in_function_call = matches!(self.active_table().last(), Some(TableTypes::FunctionCall(_))) && self.defining_parameters;
        let in_conditional = matches!(self.active_table().last(), Some(TableTypes::Conditional(_)));
        
        let in_call = (in_reasignment || in_function_call || in_conditional) && !last_finished;
        
        if !last_finished || self.set_value || in_call {
            self.handle_argument(word, token, index, in_reasignment, in_function_call, expected_fc_params, fc_params_len);
        } else {
            self.handle_new_entry(word, token, index);
        }
    }

    fn handle_argument(&mut self, word: Word, token: TokenType, index: Option<(usize, Scope, bool)>, in_reasignment: bool, in_function_call: bool, expected_fc_params: usize, fc_params_len: usize) {
        self.set_value &= TokenType::is_operator(token.clone());
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
        
        Self::add_caller_info(new_entry, index, word.word);
    }

    fn add_caller_info(new_entry: &mut TableTypes, index: Option<(usize, Scope, bool)>, word: String) {
        let Some((idx, scope, is_func)) = index else { return };
        let TableTypes::FunctionCall(fc) = new_entry else { return };
        let Some(last) = fc.parameters.as_mut().and_then(|p| p.last_mut()) else { return };
        let TableTypes::Reasingment(v) = last else { return };

        if is_func {
            *last = TableTypes::FunctionCall(FunctionCall {
                target: idx,
                target_scope: scope,
                parameters: None,
                name: word,
            });
        } else {
            v.target = idx;
            v.target_scope = scope;
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
                    name: word.word,
                };
                self.active_table().push(TableTypes::FunctionCall(func_call));
            } else {
                let reasign = Reasingment {
                    target: idx,
                    target_scope: scope,
                    parameters: None,
                    name: word.word,
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

pub fn analyze_semantically(stack: Vec<Block>) -> Vec<TableTypes>{
    let mut semantic_analyzer: SemanticAnalyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze(stack);
    

    println!("Semantic analyzer table: {:#?}", semantic_analyzer.table);
    if semantic_analyzer.error_messages.len() > 0 {
        println!("Semantic analyzer erros msg: {:#?}", semantic_analyzer.error_messages);
    }

    semantic_analyzer.table
}