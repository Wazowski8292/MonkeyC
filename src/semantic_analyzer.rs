use crate::parser::{Block, Word};
use crate::variable_types::{Variable, Function, Reasingment, FunctionCall, Conditional, Loop, Return, Types, Value};
use std::vec::Vec;
use crate::enbeded_funcs::FUNCTIONS;

struct Entry {
    word: Word,
    token: TokenType, 
    index: Option<(usize, Scope, bool)>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    If,
    Else,

    Int,
    Float,
    Double,
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
    DoubleLiteral,
    CharLiteral,
    StringLiteral,
    BoolLiteral,

    FnLiteral,
    Return,
    ReturnType,

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
            "double" => TokenType::Double,
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
            "return" => TokenType::Return,
            "->" => TokenType::ReturnType,

            "while" => TokenType::WhileLoop,

            _ if s.parse::<i64>().is_ok() => TokenType::IntegerLiteral,
            _ if (s.ends_with('f') || s.ends_with('F'))
                && s[..s.len()-1].parse::<f32>().is_ok() => TokenType::FloatLiteral,
            _ if s.parse::<f64>().is_ok() => TokenType::DoubleLiteral,
            _ if s.len() >= 2 && s.starts_with('"') && s.ends_with('"') => TokenType::StringLiteral,
            _ if s.len() >= 3 && s.starts_with('\'') && s.ends_with('\'') => TokenType::CharLiteral,
            "true" | "false" => TokenType::BoolLiteral,

            _ => TokenType::Unknow,
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            TokenType::If => "if".to_string(),
            TokenType::Else => "else".to_string(),

            TokenType::Int => "int".to_string(),
            TokenType::Float => "float".to_string(),
            TokenType::Double => "double".to_string(),
            TokenType::String => "str".to_string(),
            TokenType::Bool => "bool".to_string(),
            TokenType::Char => "char".to_string(),

            TokenType::Plus => "+".to_string(),
            TokenType::Minus => "-".to_string(),
            TokenType::Multiplication => "*".to_string(),
            TokenType::Division => "/".to_string(),
            TokenType::Equals => "=".to_string(),

            TokenType::LogicalEquals => "==".to_string(),
            TokenType::LogicalAnd => "&&".to_string(),
            TokenType::LogicalOr => "||".to_string(),
            TokenType::Not => "!".to_string(),

            TokenType::RightBitShift => ">>".to_string(),
            TokenType::LeftBitShift => "<<".to_string(),
            TokenType::And => "&".to_string(),
            TokenType::Or => "|".to_string(),

            TokenType::FnLiteral => "fn".to_string(),
            TokenType::Return => "return".to_string(),
            TokenType::ReturnType => "->".to_string(),

            TokenType::WhileLoop => "while".to_string(),

            TokenType::IntegerLiteral => "<integer literal>".to_string(),
            TokenType::FloatLiteral => "<float literal>".to_string(),
            TokenType::DoubleLiteral => "<double literal>".to_string(),
            TokenType::StringLiteral => "<string literal>".to_string(),
            TokenType::CharLiteral => "<char literal>".to_string(),
            TokenType::BoolLiteral => "<bool literal>".to_string(),

            TokenType::Unknow => "<unknown>".to_string(),
        }
    }

    pub fn is_value(token: TokenType) -> bool {
        token == TokenType::Unknow || token == TokenType::IntegerLiteral || token == TokenType::FloatLiteral ||
        token == TokenType::DoubleLiteral || token == TokenType::BoolLiteral || token ==TokenType::StringLiteral || token ==TokenType::CharLiteral
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

    pub fn literal_type(&self) -> Option<TokenType> {
        match self {
            TokenType::IntegerLiteral => Some(TokenType::Int),
            TokenType::FloatLiteral => Some(TokenType::Float),
            TokenType::DoubleLiteral => Some(TokenType::Double),
            TokenType::StringLiteral => Some(TokenType::String),
            TokenType::CharLiteral => Some(TokenType::Char),
            TokenType::BoolLiteral => Some(TokenType::Bool),
            TokenType::Int | TokenType::Float | TokenType::Double
            | TokenType::String | TokenType::Bool | TokenType::Char => Some(*self),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Scope {
    Root,
    Function,
    Parameter,
    EnbedFunc,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TableTypes {
    Variable(Variable),
    Function(Function),
    Reasingment(Reasingment),
    FunctionCall(FunctionCall),
    Conditional(Conditional),
    Loop(Loop),
    Return(Return),
    Unknown,
}

impl TableTypes {
    pub fn from_token(token: TokenType) -> Self{
        if TokenType::is_operator(token.clone()) {
            return TableTypes::Variable(Variable::new(token));
        }

        match token {
            TokenType::FnLiteral => TableTypes::Function(Function::new(token)),
            TokenType::Return => TableTypes::Return(Return::new(token)),
            TokenType::If | TokenType::Else => TableTypes::Conditional(Conditional::new(token)),
            TokenType::WhileLoop => TableTypes::Loop(Loop::new(token)),
            TokenType::Unknow => TableTypes::Reasingment(Reasingment::new(TokenType::Unknow)),
            TokenType::Int | TokenType::Float | TokenType::Double
            | TokenType::Bool | TokenType::Char | TokenType::String => TableTypes::Variable(Variable::new(token)),
            _ if TokenType::is_value(token) && token != TokenType::Unknow => TableTypes::Variable(Variable::new(token)),
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
            TableTypes::Return(returns) => returns.finished_definition(),
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
            TableTypes::Return(returns) => returns.add_arguments(argument),
            _ => {}
        }
    }
}

struct SemanticAnalyzer {
    table: Vec<TableTypes>,
    error_messages: Vec<String>,
    set_value: bool,
    set_return_value: bool,
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
            set_return_value: false,
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
                    let last_is_fn = matches!(self.active_table().last(), Some(TableTypes::Function(_)));
                    self.max_nesting += 1;

                    
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

    fn is_enbeded_func(name: String) -> (bool, usize){
        let mut i = 0;
        for func in FUNCTIONS.iter() {
            if func.name == name {
                return (true, i);
            }

            i += 1;
        }

        (false, 0)
    }

    fn find_in_level(name: &str, table: &Vec<TableTypes>, nest_level: usize) -> Option<(usize, Scope, bool)> {
        let (is_enbeded_func, index) = Self::is_enbeded_func(name.to_string());
        table.iter().enumerate().find_map(|(idx, entry)| match entry {
            TableTypes::Variable(v) if v.name.as_deref() == Some(name) => {
                Some((idx, Self::scope_for_level(nest_level), false))
            }
            TableTypes::Function(f) if f.name.as_deref() == Some(name) => {
                Some((idx, Self::scope_for_level(nest_level), true))
            }
            _ if is_enbeded_func => Some((index, Scope::EnbedFunc, true)),
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
        if let Some(entry) = last_table.last() {
            if Self::pending_call_in(entry).is_some() {
                return last_table;
            }
        }

        let has_child = matches!(
            last_table.last(),
            Some(TableTypes::Function(_)) | Some(TableTypes::Conditional(_)) | Some(TableTypes::Loop(_))
        );

        if !has_child || (current_nest_level == max_nesting ) && !defining_parameters {
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
        } else if token == TokenType::ReturnType {
            self.set_return_value = true;
            return;
        }

        let index = self.resolve(word.word.clone());
        let mut last_finished = self.active_table().last().map_or(true, |e| e.finished_definition());
        
        let mut is_fc = false;
        let mut in_nested_call = false;
        let mut fc_target = 0;
        let mut fc_scope = Scope::Root;
        let mut fc_params_len = 0;

        let defining_parameters = self.defining_parameters;

        match self.active_table().last() {
            Some(TableTypes::FunctionCall(fc)) if defining_parameters => {
                is_fc = true;
                fc_target = fc.target;
                fc_scope = fc.scope;
                fc_params_len = fc.parameters.as_ref().map_or(0, |p| p.len());
            }
            Some(entry @ (TableTypes::Variable(_) | TableTypes::Return(_))) if defining_parameters => {
                if let Some(pending) = Self::pending_call_in(entry) {
                    in_nested_call = true;
                    fc_target = pending.target;
                    fc_params_len = pending.parameters.as_ref().map_or(0, |p| p.len());
                }
            }   
            _ => {}
        }

        let mut expected_fc_params = 0;
        if is_fc || in_nested_call {
            if fc_scope == Scope::EnbedFunc {
                expected_fc_params = FUNCTIONS[fc_target].parameters.len();
            } else if let Some(TableTypes::Function(f)) = self.table.get(fc_target) {
                expected_fc_params = f.parameters.as_ref().map_or(0, |p| p.len());
            }

            last_finished = false;
        }

        let in_reasignment = matches!(self.active_table().last(), Some(TableTypes::Reasingment(_)));
        let in_function_call = matches!(self.active_table().last(), Some(TableTypes::FunctionCall(_))) && self.defining_parameters;
        let in_conditional = matches!(self.active_table().last(), Some(TableTypes::Conditional(_)));
        
        let in_call = (in_reasignment || in_function_call || in_conditional || in_nested_call) && !last_finished;
        
        let entry = Entry {
            word: word,
            token: token,
            index: index,
        };

        if !last_finished || self.set_value || self.set_return_value || in_call {
            self.handle_argument(entry, in_reasignment, in_function_call, in_nested_call, expected_fc_params, fc_params_len, fc_target);
        } else {
            self.handle_new_entry(entry);
        }
    }

    fn handle_argument(&mut self, entry_info: Entry, in_reasignment: bool, in_function_call: bool, in_nested_call: bool, expected_fc_params: usize, fc_params_len: usize, fc_target: usize) {
        let token = entry_info.token.clone();
        let word = entry_info.word.clone();
        let index = entry_info.index.clone();
        
        self.set_return_value = false;
        self.set_value &= TokenType::is_operator(token.clone());
        let in_call_or_reasign = in_reasignment || in_function_call || in_nested_call;

        if in_function_call || in_nested_call {
            if fc_params_len >= expected_fc_params {
                self.error_messages.push(format!("Too many arguments for function call. Expected: {}, Found: {}; Line: {}; Char pos: {}", expected_fc_params, fc_params_len + 1, word.line.unwrap_or(0), word.char_num.unwrap_or(0)));
            } else {
                self.check_parameters(entry_info, fc_params_len, fc_target);                
            }
        }

        if in_call_or_reasign && token == TokenType::Unknow && index.is_none() {
            self.error_messages.push(format!("Undefined symbol: {}; Line: {}; Char pos: {}", word.word, word.line.unwrap_or(0), word.char_num.unwrap_or(0)));
        }

        let mut argument = word.word.clone();

        let mut mismatch: Option<(TokenType, Option<TokenType>)> = None;
        match self.active_table().last_mut() {
            Some(entry) => {
                if let TableTypes::Variable(var) = entry {
                    if in_function_call {
                        argument += ";fc";
                    } else if !in_nested_call {
                        let value_type = TokenType::from_str(&argument.clone()).literal_type();

                        if var.name.is_some() {
                            if let Some(vt) = value_type {
                                if vt != var.token_type {
                                    mismatch = Some((var.token_type, Some(vt)));
                                }
                            }
                        }
                    }
                } else if let TableTypes::Reasingment(reasign) = entry {
                    if reasign.token_type != TokenType::Unknow && !in_nested_call {
                        let value_type = TokenType::from_str(&argument.clone()).literal_type();
                        if let Some(vt) = value_type {
                            if vt != reasign.token_type && !TokenType::is_operator(reasign.token_type) {
                                mismatch = Some((reasign.token_type, Some(vt)));
                            }
                        }
                    }
                }
            },
            None => {
                self.error_messages.push("There wasn't a last entry".to_string());
                return;
            },
        }

        if let Some((expected, found)) = mismatch {
            self.error_messages.push(format!(
                "Type mismatch. Expected: {}, Found: {}, Line: {}, Char pos: {}",
                expected.to_str(),
                found.unwrap_or(TokenType::Unknow).to_str(),
                word.line.unwrap_or(0), 
                word.char_num.unwrap_or(0)));
        }

        let table_snapshot = self.table.clone();
        let mut extra_errors: Vec<String> = Vec::new();

        let new_entry = match self.active_table().last_mut() {
            Some(entry) => entry,
            None => {
                self.error_messages.push("There wasn't a last entry".to_string());
                return;
            },
        };

        if in_nested_call {
            if let Some(fc) = Self::pending_call_in_mut(new_entry) {
                fc.add_arguments(argument);
                Self::add_caller_info_on_call(fc, index, word.word);
                return;
            }
        }

        new_entry.add_arguments(argument);
        
        Self::add_caller_info(new_entry, index, word.word.clone(), &table_snapshot, &mut extra_errors, &word);

        self.error_messages.append(&mut extra_errors);
    }

    fn check_parameters(&mut self, entry_info: Entry, fc_params_len: usize, fc_target: usize) {
        let token = entry_info.token.clone();
        let word = entry_info.word.clone();
        let index = entry_info.index.clone();

        let expected_type = if let Some(TableTypes::Function(f)) = self.table.get(fc_target) {
            f.parameters.as_ref()
                .and_then(|params| params.get(fc_params_len))
                .and_then(|p| match p {
                    TableTypes::Variable(v) => Some(v.token_type),
                    _ => None,
                })
        } else {
            None
        };

        if let Some(expected_type) = expected_type {
            let actual_type = token.literal_type().or_else(|| self.declared_type_of(&index));

            if let Some(actual_type) = actual_type {
                if actual_type != expected_type {
                    self.error_messages.push(format!(
                        "Type mismatch for argument {} of function call: expected {:?}, got {:?}; Line: {}; Char pos: {}",
                        fc_params_len + 1, expected_type, actual_type,
                        word.line.unwrap_or(0), word.char_num.unwrap_or(0)
                    ));
                }
            }
        }
    }

    fn declared_type_of(&self, index: &Option<(usize, Scope, bool)>) -> Option<TokenType> {
        let (idx, scope, _) = index.as_ref()?;

        let entry = match scope {
            Scope::Root => self.table.get(*idx),
            Scope::Function | Scope::Parameter => self.table.iter().find_map(|t| {
                if let TableTypes::Function(f) = t {
                    f.table.get(*idx)
                } else {
                    None
                }
            }),
            Scope::EnbedFunc => {
                todo!()
            }
        };

        match entry {
            Some(TableTypes::Variable(v)) => Some(v.token_type),
            _ => None,
        }
    }

    fn add_caller_info(new_entry: &mut TableTypes, index: Option<(usize, Scope, bool)>, word: String, table: &Vec<TableTypes>, error_messages: &mut Vec<String>, call_word: &Word) {
        let Some((idx, scope, is_func)) = index else { return };

        match new_entry {
            TableTypes::FunctionCall(fc) => Self::add_caller_info_on_call(fc, Some((idx, scope, is_func)), word),
            TableTypes::Variable(var) if is_func => {
                let var_type = var.token_type;
                Self::promote_pending_var_to_call(var, idx, word, scope);
                Self::check_func_return_type(error_messages, table, idx, var_type, call_word);
            }
            TableTypes::Reasingment(reasign) if is_func => {
                let reasign_type = reasign.token_type;
                reasign.parameters.get_or_insert_with(Vec::new).push(
                    TableTypes::FunctionCall(FunctionCall { target: idx, parameters: None, name: word , scope: scope})
                );
                if reasign_type != TokenType::Unknow {
                    Self::check_func_return_type(error_messages, table, idx, reasign_type, call_word);
                }
            }
            TableTypes::Return(ret) if is_func => {
                if let Some(var) = ret.value.as_mut() {
                    Self::promote_pending_var_to_call(var, idx, word, scope);
                }
            }
            _ => {}
        }
    }

    fn check_func_return_type(error_messages: &mut Vec<String>, table: &Vec<TableTypes>, func_idx: usize, expected_type: TokenType, word: &Word) {
        let return_type = table.get(func_idx).and_then(|e| {
            if let TableTypes::Function(f) = e { f.return_type } else { None }
        });

        if let Some(ret_type) = return_type {
            if ret_type != expected_type {
                error_messages.push(format!(
                    "Type mismatch: variable is '{}' but function '{}' returns '{}'; Line: {}; Char pos: {}",
                    expected_type.to_str(),
                    word.word,
                    ret_type.to_str(),
                    word.line.unwrap_or(0),
                    word.char_num.unwrap_or(0)
                ));
            }
        }
    }

    fn promote_pending_var_to_call(variable: &mut Variable, idx: usize, word: String, scope: Scope) {
        let Some(values) = variable.value.as_mut() else { return };
        let Some(last) = values.last_mut() else { return };

        if let Value::Var(_) = last {
            *last = Value::FuncCall(FunctionCall {
                target: idx,
                parameters: None,
                name: word,
                scope: scope,
            });
        }
    }

    fn add_caller_info_on_call(fc: &mut FunctionCall, index: Option<(usize, Scope, bool)>, word: String) {
        let Some((idx, scope, is_func)) = index else { return };
        let Some(last) = fc.parameters.as_mut().and_then(|p| p.last_mut()) else { return };
        let TableTypes::Reasingment(v) = last else { return };

        if is_func {
            *last = TableTypes::FunctionCall(FunctionCall {
                target: idx,
                parameters: None,
                name: word,
                scope: scope,
            });
        } else {
            v.target = idx;
            v.target_scope = scope;
        }
    }

    fn handle_new_entry(&mut self, entry_info: Entry) {
        let token = entry_info.token.clone();
        let word = entry_info.word.clone();
        let index = entry_info.index.clone();

        if !TokenType::is_value(token) {
            self.add_entry(token);
        } else if index.is_some() {
            self.set_value = true;

            let (idx, scope, is_func) = index.expect("Error finding the index of the value to be reasign");
            
            if is_func {
                let func_call = FunctionCall {
                    target: idx,
                    parameters: None,
                    name: word.word,
                    scope: scope,
                };
                self.active_table().push(TableTypes::FunctionCall(func_call));
            } else {
                let target_type = match scope {
                    Scope::Root => self.table.get(idx),
                    Scope::Function | Scope::Parameter => self.table.iter().rev().find_map(|t| {
                        if let TableTypes::Function(f) = t { f.table.get(idx) } else { None }
                    }),
                    Scope::EnbedFunc => todo!()
                };
                let token_type = match target_type {
                    Some(TableTypes::Variable(v)) => v.token_type,
                    _ => TokenType::Unknow,
                };
                let reasign = Reasingment {
                    target: idx,
                    target_scope: scope,
                    parameters: None,
                    name: word.word,
                    token_type,
                };
                self.active_table().push(TableTypes::Reasingment(reasign));
            }
        } else {
            self.error_messages.push(format!("Undefined symbol: {}; Line: {}:{}", word.word, word.line.unwrap_or(0), word.char_num.unwrap_or(0)));
        }
    }

    fn pending_call_in(entry: &TableTypes) -> Option<&FunctionCall> {
        match entry {
            TableTypes::Variable(v) => v.pending_call(),
            TableTypes::Return(r) => r.value.as_ref().and_then(|v| v.pending_call()),
            _ => None,
        }
    }

    fn pending_call_in_mut(entry: &mut TableTypes) -> Option<&mut FunctionCall> {
        match entry {
            TableTypes::Variable(v) => v.pending_call_mut(),
            TableTypes::Return(r) => r.value.as_mut().and_then(|v| v.pending_call_mut()),
            _ => None,
        }
    }

    fn tokenize_line(&mut self, line: &Vec<Word>) {
        for word in line {
            self.tokenize_word(word.clone());
        }
    }
}

pub fn analyze_semantically(stack: Vec<Block>) -> Result<Vec<TableTypes>, usize>{
    let mut semantic_analyzer: SemanticAnalyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze(stack);
    

    println!("Semantic analyzer table: {:#?}", semantic_analyzer.table);
    let len = semantic_analyzer.error_messages.len();
    if len > 0 {
        println!("Semantic analyzer erros msg: {:#?}", semantic_analyzer.error_messages);
        return Err(len);
    }

    Ok(semantic_analyzer.table)
}