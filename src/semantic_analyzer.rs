use crate::parser::{Block, Word};
use crate::variable_types::{Variable, Function, Reasingment, FunctionCall, Conditional, Loop, Return, StructLiteral, Types, Value, PointerType};
use std::vec::Vec;
use crate::enbeded_funcs::FUNCTIONS;

struct Entry {
    word: Word,
    token: TokenType, 
    index: Option<(usize, Scope, ResolveType)>,
}

struct Error {
    msg: String,
    line: usize,
    char: usize,
}

#[derive(Debug, PartialEq, Clone)]
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
    PlusEquals,
    MinusEquals,
    PlusPlus,
    MinusMinus,

    LogicalEquals,
    NotEquals,
    GreaterThan,
    LessThan,
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

    StructLiteral,
    StructDef(String),

    WhileLoop,
    
    Dots,
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
            "+=" => TokenType::PlusEquals,
            "-=" => TokenType::MinusEquals,
            "++" => TokenType::PlusPlus,
            "--" => TokenType::MinusMinus,

            "==" => TokenType::LogicalEquals,
            "!=" => TokenType::NotEquals,
            ">" => TokenType::GreaterThan,
            "<" => TokenType::LessThan,
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

            "struct" => TokenType::StructLiteral,

            "while" => TokenType::WhileLoop,

            "::" => TokenType::Dots,
            
            "true" | "false" => TokenType::BoolLiteral,

            _ if s.parse::<i64>().is_ok() => TokenType::IntegerLiteral,
            _ if (s.ends_with('f') || s.ends_with('F'))
                && s[..s.len()-1].parse::<f32>().is_ok() => TokenType::FloatLiteral,
            _ if s.parse::<f64>().is_ok() => TokenType::DoubleLiteral,
            _ if s.len() >= 2 && s.starts_with('"') && s.ends_with('"') => TokenType::StringLiteral,
            _ if s.len() >= 3 && s.starts_with('\'') && s.ends_with('\'') => TokenType::CharLiteral,

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
            TokenType::PlusEquals => "+=".to_string(),
            TokenType::MinusEquals => "-=".to_string(),
            TokenType::PlusPlus => "++".to_string(),
            TokenType::MinusMinus => "--".to_string(),

            TokenType::LogicalEquals => "==".to_string(),
            TokenType::NotEquals => "!=".to_string(),
            TokenType::GreaterThan => ">".to_string(),
            TokenType::LessThan => "<".to_string(),
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

            TokenType::StructLiteral => "struct".to_string(),
            TokenType::StructDef(name) => format!("struct type of {}", name.clone()),

            TokenType::WhileLoop => "while".to_string(),

            TokenType::IntegerLiteral => "<integer literal>".to_string(),
            TokenType::FloatLiteral => "<float literal>".to_string(),
            TokenType::DoubleLiteral => "<double literal>".to_string(),
            TokenType::StringLiteral => "<string literal>".to_string(),
            TokenType::CharLiteral => "<char literal>".to_string(),
            TokenType::BoolLiteral => "<bool literal>".to_string(),

            TokenType::Dots => "->".to_string(),
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
        token == TokenType::LogicalEquals || token == TokenType::NotEquals || token == TokenType::GreaterThan || token == TokenType::LessThan || token == TokenType::LogicalAnd || token == TokenType::LogicalOr || token == TokenType::Not
    }
    
    pub fn is_aritmetic_operator(token: TokenType) -> bool {
        token == TokenType::Plus || token == TokenType::Minus || token == TokenType::Multiplication || token == TokenType::Division || token == TokenType::Equals || token == TokenType::PlusEquals || token == TokenType::MinusEquals || token == TokenType::PlusPlus || token == TokenType::MinusMinus
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
            | TokenType::String | TokenType::Bool | TokenType::Char => Some(self.clone()),
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
    StructLiteral(StructLiteral),
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
            | TokenType::Bool | TokenType::Char | TokenType::String | TokenType::StructDef(_) => TableTypes::Variable(Variable::new(token)),
            TokenType::StructLiteral => TableTypes::StructLiteral(StructLiteral::new(token)),
            _ if TokenType::is_value(token.clone()) && token.clone() != TokenType::Unknow => TableTypes::Variable(Variable::new(token)),
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
            TableTypes::StructLiteral(struct_literal) => struct_literal.finished_definition(),
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
            TableTypes::StructLiteral(struct_literal) => struct_literal.add_arguments(argument),
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum ResolveType {
    Variable,
    Function,
    Struct,
}

struct SemanticAnalyzer {
    table: Vec<TableTypes>,
    error_messages: Vec<Error>,
    set_value: bool,
    set_return_value: bool,
    defining_fn: bool,
    defining_parameters: bool,
    max_nesting: usize,
    ptr_type: Option<PointerType>,
    acces_var: bool,
    struct_instance_name: Option<String>,
    current_struct_init: Option<String>,
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
            ptr_type: None,
            acces_var: false,
            struct_instance_name: None,
            current_struct_init: None,
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
                    self.current_struct_init = None;
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

    fn resolve(&mut self, name: String) -> Option<(usize, Scope, ResolveType)> {
        let lookup_name = crate::variable_types::parse_array_syntax(&name)
            .map(|(arr_name, _)| arr_name)
            .unwrap_or(name);
        if self.acces_var {
            if let Some(inst_name) = self.struct_instance_name.clone() {
                return Self::resolve_in_chain(&inst_name, &mut self.table, 0, self.max_nesting, self.defining_parameters)
                    .or_else(|| self.resolve_in_parameters(&inst_name));
            }
            return None;
        } else if let Some(result) = Self::resolve_in_chain(&lookup_name, &mut self.table, 0, self.max_nesting, self.defining_parameters) {
            return Some(result);
        } else {
            return self.resolve_in_parameters(&lookup_name);
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

    fn find_in_level(name: &str, table: &Vec<TableTypes>, nest_level: usize) -> Option<(usize, Scope, ResolveType)> {
        let (is_enbeded_func, index) = Self::is_enbeded_func(name.to_string());
        table.iter().enumerate().find_map(|(idx, entry)| match entry {
            TableTypes::Variable(v) if v.name.as_deref() == Some(name) => Some((idx, Self::scope_for_level(nest_level), ResolveType::Variable)),
            TableTypes::Function(f) if f.name.as_deref() == Some(name) => Some((idx, Self::scope_for_level(nest_level), ResolveType::Function)),
            TableTypes::StructLiteral(s) if s.name == name => Some((idx, Self::scope_for_level(nest_level), ResolveType::Struct)),
            _ if is_enbeded_func => Some((index, Scope::EnbedFunc, ResolveType::Function)),
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
            Some(TableTypes::Function(_)) | Some(TableTypes::Conditional(_)) | Some(TableTypes::Loop(_)) | Some(TableTypes::StructLiteral(_))
        );
        if !has_child {
            return false;
        }

        let nesting_exhausted = (current_nest_level + 1 > max_nesting) && !defining_parameters;
        let blocked_by_params = defining_parameters
            && matches!(table.last(), Some(TableTypes::Conditional(_)) | Some(TableTypes::Loop(_)));

        !nesting_exhausted && !blocked_by_params
    }

    fn descend_and_resolve(name: &str, table: &mut Vec<TableTypes>, current_nest_level: usize, 
        max_nesting: usize, defining_parameters: bool) -> Option<(usize, Scope, ResolveType)> {
        
        match table.last_mut().unwrap() {
            TableTypes::Function(func) => {
                if defining_parameters && func.table.is_empty() {
                    None
                } else {
                    Self::resolve_in_chain(name, &mut func.table, current_nest_level + 1, max_nesting, defining_parameters)
                }
            }
            TableTypes::Conditional(con) => Self::resolve_in_chain(name, &mut con.table, current_nest_level + 1, max_nesting, defining_parameters),
            TableTypes::Loop(while_loop) => Self::resolve_in_chain(name, &mut while_loop.table, current_nest_level + 1, max_nesting, defining_parameters),
            TableTypes::StructLiteral(struct_def) => Self::resolve_in_chain(name, &mut struct_def.functions, current_nest_level + 1, max_nesting, defining_parameters),
            _ => unreachable!(),
        }
    }

    fn resolve_in_chain( name: &str, table: &mut Vec<TableTypes>, current_nest_level: usize, 
        max_nesting: usize, defining_parameters: bool) -> Option<(usize, Scope, ResolveType)> {

        if Self::should_descend(table, current_nest_level, max_nesting, defining_parameters) {
            if let Some(found) = Self::descend_and_resolve(name, table, current_nest_level, max_nesting, defining_parameters) {
                return Some(found);
            }
        }

        Self::find_in_level(name, table, current_nest_level)
    }

    fn resolve_in_parameters(&self, name: &str) -> Option<(usize, Scope, ResolveType)> {
        if !self.defining_fn {
            return None;
        }

        let Some(TableTypes::Function(f)) = self.table.last() else {
            return None;
        };
        let params = f.parameters.as_ref()?;

        params.iter().enumerate().find_map(|(idx, entry)| match entry {
            TableTypes::Variable(v) if v.name.as_deref() == Some(name) => Some((idx, Scope::Parameter, ResolveType::Variable)),
            _ => None,
        })
    }

    fn add_entry(&mut self, token: TokenType) {
        let mut table_type: TableTypes = TableTypes::from_token(token.clone());
        match table_type {
            TableTypes::Variable(ref mut var) => var.ptr = self.ptr_type.clone(),
            TableTypes::Reasingment(ref mut re) => re.ptr = self.ptr_type.clone(),
            _ => {}
        }
        
        self.defining_fn |= token == TokenType::FnLiteral;

        self.active_table().push(table_type)
    }

    fn active_table(&mut self) -> &mut Vec<TableTypes> {
        Self::desend_table(self.defining_parameters, self.defining_fn, &mut self.table, 1, self.max_nesting)

    }

    fn desend_table( defining_parameters: bool, defining_fn: bool, last_table: &mut Vec<TableTypes>, 
        current_nest_level: usize, max_nesting: usize) -> &mut Vec<TableTypes> {
        
        if let Some(entry) = last_table.last() {
            if Self::pending_call_in(entry).is_some() {
                return last_table;
            }
        }

        let has_child = matches!(
            last_table.last(),
            Some(TableTypes::Function(_)) | Some(TableTypes::Conditional(_)) | Some(TableTypes::Loop(_)) | Some(TableTypes::StructLiteral(_))
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
                    Self::desend_table(defining_parameters, defining_fn, &mut func.table, current_nest_level + 1, max_nesting)
                }
            },
            TableTypes::Conditional(con) => {
                Self::desend_table(defining_parameters, defining_fn, &mut con.table, current_nest_level + 1, max_nesting)
            },
            TableTypes::Loop(while_loop) => {
                Self::desend_table(defining_parameters, defining_fn, &mut while_loop.table, current_nest_level + 1, max_nesting)
            },
            TableTypes::StructLiteral(struct_literal) => {
                if defining_fn {
                    Self::desend_table(defining_parameters, defining_fn, &mut struct_literal.functions, current_nest_level + 1, max_nesting)
                } else {
                    Self::desend_table(defining_parameters, defining_fn, &mut struct_literal.arguments, current_nest_level + 1, max_nesting)
                }
            }
            _ => unreachable!(),
        }
    }

    fn tokenize_word(&mut self, mut word: Word) {
        match word.word.clone().chars().next() {
            Some('&') => { self.ptr_type = Some(PointerType::Reference); word.word = word.word.replace("&", ""); }
            Some('*') => { self.ptr_type = Some(PointerType::Pointer); word.word = word.word.replace("*", ""); }
            _ => self.ptr_type = None,
        }

        let token = TokenType::from_str(&word.word);

        self.set_value |= TokenType::is_operator(token.clone());

        match token {
            TokenType::Equals => return,
            TokenType::ReturnType => {self.set_return_value = true; return;}
            TokenType::Dots => {
                let inst_name = self.find_preceding_instance_name();
                self.struct_instance_name = inst_name;
                self.acces_var = true;
                return;
            }
            _ => {}
        }

        if self.acces_var {
            self.acces_var = false;
            let field_name = word.word.clone();
            let inst_name = self.struct_instance_name.take().unwrap_or_default();
            let mangled = format!("{}__{}", inst_name, field_name);

            let inst_index = Self::resolve_in_chain(
                &inst_name, &mut self.table, 0, self.max_nesting, self.defining_parameters,
            ).or_else(|| self.resolve_in_parameters(&inst_name));

            let (target, scope, tok) = if let Some((idx, sc, _)) = inst_index {
                let active_tbl = &self.table;
                let tok = match sc {
                    Scope::Root => active_tbl.get(idx),
                    Scope::Function | Scope::Parameter => active_tbl.iter().rev().find_map(|t| {
                        if let TableTypes::Function(f) = t { f.table.get(idx) } else { None }
                    }),
                    Scope::EnbedFunc => None,
                };
                let tok_type = match tok {
                    Some(TableTypes::Variable(v)) => v.token_type.clone(),
                    _ => TokenType::Unknow,
                };
                (idx, sc, tok_type)
            } else {
                (0, Scope::Root, TokenType::Unknow)
            };

            if let Some(TableTypes::Reasingment(r)) = self.active_table().last_mut() {
                if r.parameters.as_ref().map_or(true, |p| p.is_empty()) {
                    r.name = mangled.clone();
                    r.target = target;
                    r.target_scope = scope;
                    r.token_type = tok.clone();
                    return;
                }
            }

            if matches!(self.active_table().last(), Some(TableTypes::FunctionCall(_))) && self.defining_parameters {
                if let Some(TableTypes::FunctionCall(fc)) = self.active_table().last_mut() {
                    if let Some(p) = fc.parameters.as_mut().and_then(|p| p.last_mut()) {
                        if let TableTypes::Reasingment(inner) = p {
                            inner.name = mangled.clone();
                            inner.target = target;
                            inner.target_scope = scope;
                            inner.token_type = tok.clone();
                            return;
                        }
                    }
                }
            }

            if let Some(TableTypes::Reasingment(r)) = self.active_table().last_mut() {
                if let Some(p) = r.parameters.as_mut().and_then(|p| p.last_mut()) {
                    if let TableTypes::Reasingment(inner) = p {
                        inner.name = mangled.clone();
                        inner.target = target;
                        inner.target_scope = scope;
                        inner.token_type = tok.clone();
                        return;
                    }
                }
            }

            if let Some(TableTypes::Variable(v)) = self.active_table().last_mut() {
                if let Some(vals) = v.value.as_mut() {
                    if let Some(last_val) = vals.last_mut() {
                        *last_val = Value::Var(mangled.clone());
                        return;
                    }
                }
            }

            let reasign = Reasingment {
                target,
                target_scope: scope,
                parameters: None,
                name: mangled,
                token_type: tok,
                ptr: self.ptr_type.clone(),
                array_index: None,
            };
            self.active_table().push(TableTypes::Reasingment(reasign));
            self.set_value = true;
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

    fn find_preceding_instance_name(&self) -> Option<String> {
        let active = self.active_table_ref();
        match active.last() {
            Some(TableTypes::FunctionCall(fc)) => {
                if let Some(p) = fc.parameters.as_ref().and_then(|p| p.last()) {
                    match p {
                        TableTypes::Reasingment(r) => Some(r.name.clone()),
                        TableTypes::Variable(v) => v.name.clone(),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            Some(TableTypes::Reasingment(r)) => {
                if let Some(p) = r.parameters.as_ref().and_then(|p| p.last()) {
                    match p {
                        TableTypes::Reasingment(inner) => Some(inner.name.clone()),
                        TableTypes::Variable(v) => v.name.clone(),
                        _ => None,
                    }
                } else {
                    Some(r.name.clone())
                }
            }
            Some(TableTypes::Variable(v)) => {
                if let Some(vals) = v.value.as_ref().and_then(|v| v.last()) {
                    if let Value::Var(name) = vals {
                        Some(name.clone())
                    } else {
                        v.name.clone()
                    }
                } else {
                    v.name.clone()
                }
            }
            _ => None,
        }
    }

    fn active_table_ref(&self) -> &Vec<TableTypes> {
        Self::descend_table_ref(self.defining_parameters, self.defining_fn, &self.table, 1, self.max_nesting)
    }

    fn descend_table_ref<'a>(defining_parameters: bool, defining_fn: bool,
        last_table: &'a Vec<TableTypes>, current_nest_level: usize, max_nesting: usize) -> &'a Vec<TableTypes> {

        let has_child = matches!(
            last_table.last(),
            Some(TableTypes::Function(_)) | Some(TableTypes::Conditional(_)) | Some(TableTypes::Loop(_)) | Some(TableTypes::StructLiteral(_))
        );

        if !has_child || (current_nest_level == max_nesting) && !defining_parameters {
            return last_table;
        }

        if defining_parameters && max_nesting == current_nest_level {
            if let Some(TableTypes::Conditional(_)) = last_table.last() {
                return last_table;
            } else if let Some(TableTypes::Loop(_)) = last_table.last() {
                return last_table;
            }
        }

        match last_table.last().unwrap() {
            TableTypes::Function(func) => {
                if defining_parameters && func.table.is_empty() {
                    func.parameters.as_ref().map(|p| p as &Vec<TableTypes>).unwrap_or(last_table)
                } else {
                    Self::descend_table_ref(defining_parameters, defining_fn, &func.table, current_nest_level + 1, max_nesting)
                }
            }
            TableTypes::Conditional(con) => {
                Self::descend_table_ref(defining_parameters, defining_fn, &con.table, current_nest_level + 1, max_nesting)
            }
            TableTypes::Loop(while_loop) => {
                Self::descend_table_ref(defining_parameters, defining_fn, &while_loop.table, current_nest_level + 1, max_nesting)
            }
            TableTypes::StructLiteral(struct_literal) => {
                if defining_fn {
                    Self::descend_table_ref(defining_parameters, defining_fn, &struct_literal.functions, current_nest_level + 1, max_nesting)
                } else {
                    Self::descend_table_ref(defining_parameters, defining_fn, &struct_literal.arguments, current_nest_level + 1, max_nesting)
                }
            }
            _ => unreachable!(),
        }
    }

    fn handle_argument(&mut self, entry_info: Entry, in_reasignment: bool, in_function_call: bool, in_nested_call: bool,
        expected_fc_params: usize, fc_params_len: usize, fc_target: usize) {

        let token = entry_info.token.clone();
        let word = entry_info.word.clone();
        let index = entry_info.index.clone();
        
        self.set_return_value = false;
        self.acces_var = false;

        if let Some((_, _, ResolveType::Struct)) = index {
            let inst_name = match self.active_table().last() {
                Some(TableTypes::Variable(v)) => v.name.clone(),
                Some(TableTypes::Reasingment(r)) => Some(r.name.clone()),
                _ => None,
            };
            if let Some(name) = inst_name {
                self.current_struct_init = Some(name);
                self.set_value = false;
                if matches!(self.active_table().last(), Some(TableTypes::Reasingment(_))) {
                    self.active_table().pop();
                }
                return;
            }
        }

        if token == TokenType::PlusPlus || token == TokenType::MinusMinus {
            self.set_value = false;
        } else {
            self.set_value &= TokenType::is_operator(token.clone());
        }
        let in_call_or_reasign = in_reasignment || in_function_call || in_nested_call;

        if in_function_call || in_nested_call {
            if fc_params_len >= expected_fc_params {
                let error = Error {
                    msg: format!("Too many arguments for function call. Expected: {}, Found: {}", 
                            expected_fc_params, fc_params_len + 1),
                    line: word.line.unwrap_or(0), 
                    char: word.char_num.unwrap_or(0),
                };

                self.error_messages.push(error); 
            } else {
                self.check_parameters(entry_info, fc_params_len, fc_target);                
            }
        }

        if in_call_or_reasign && token == TokenType::Unknow && index.is_none() {
            let error = Error {
                msg: format!("Undefined symbol: {}", word.word),
                line: word.line.unwrap_or(0), 
                char: word.char_num.unwrap_or(0),
            };

            self.error_messages.push(error);
        }

        let rhs_ptr_type = self.ptr_type.clone();
        let mut mismatch: Option<(TokenType, Option<TokenType>, Option<PointerType>)> = None;
        match self.active_table().last_mut() {
            Some(TableTypes::Variable(var)) if !in_nested_call => {
                
                let value_type = TokenType::from_str(&word.word.clone()).literal_type();

                if var.name.is_some() && let Some(vt) = value_type {
                    let corrrect_ptr_type = match var.ptr {
                        Some(PointerType::Pointer) => rhs_ptr_type == Some(PointerType::Reference), 
                        _ => true,
                    };

                    if vt != var.token_type && !corrrect_ptr_type {
                        mismatch = Some((var.token_type.clone(), Some(vt), rhs_ptr_type.clone()));
                    }
                }   
            }
            Some(TableTypes::Reasingment(reasign)) if !in_nested_call => {
                if reasign.token_type == TokenType::Unknow {
                    let value_type = TokenType::from_str(&word.word.clone()).literal_type();
                    if let Some(vt) = value_type {
                        let corrrect_ptr_type = match reasign.ptr {
                            Some(PointerType::Reference) => rhs_ptr_type != Some(PointerType::Pointer),
                            None => rhs_ptr_type == None,
                            _ => true,
                        };


                        if vt != reasign.token_type && !TokenType::is_operator(reasign.token_type.clone()) && !corrrect_ptr_type {
                            mismatch = Some((reasign.token_type.clone(), Some(vt), rhs_ptr_type));
                        }
                    }
                }
            }
            None => {
                let error = Error {
                    msg: "There wasn't a last entry".to_string(),
                    line: word.line.unwrap_or(0), 
                    char: word.char_num.unwrap_or(0),

                };
                
                self.error_messages.push(error);
                return;
            }
            _ => {}
        }

        if let Some((expected, found, ptr_type)) = mismatch {
            let ptr_type_str = match ptr_type {
                Some(PointerType::Pointer) => "*",
                Some(PointerType::Reference) => "&",
                _ => "",
            };
            let error = Error {
                msg: format!("Type mismatch. Expected: {}{}, Found: {}", ptr_type_str, expected.to_str(), found.unwrap_or(TokenType::Unknow).to_str()),
                line: word.line.unwrap_or(0), 
                char: word.char_num.unwrap_or(0)

            };

            self.error_messages.push(error);
        }

        let new_entry = match self.active_table().last_mut() {
            Some(entry) => entry,
            None => {
                let error = Error {
                    msg : "There wasn't a last entry".to_string(),
                    line: word.line.unwrap_or(0), 
                    char: word.char_num.unwrap_or(0),

                };

                self.error_messages.push(error);
                return;
            },
        };

        if in_nested_call {
            if let Some(fc) = Self::pending_call_in_mut(new_entry) {
                fc.add_arguments(word.word.clone());
                Self::add_caller_info_on_call(fc, index, word.word);
                return;
            }
        }

        new_entry.add_arguments(word.word.clone());
        
        if let Some((idx, expected_type)) = Self::add_caller_info(new_entry, index, &word) {
            self.check_func_return_type(idx, expected_type, &word);
        }
        
        self.add_pointer_info();
    }

    fn add_pointer_info(&mut self) {
        let ptr_type = self.ptr_type.clone();
        match self.active_table().last_mut() {
            Some(TableTypes::Variable(var)) => {
                if let Some(val) = var.value.as_mut().and_then(|v| v.last_mut()) {
                    if let Value::Var(name) = val {
                        *val = match ptr_type {
                            Some(PointerType::Reference) => Value::Ref(name.clone()),
                            Some(PointerType::Pointer) => Value::Deref(name.clone()),
                            None => Value::Var(name.clone()),
                        };
                    }
                }
            }
            Some(TableTypes::Reasingment(last)) => {
                if let Some(TableTypes::Reasingment(reasign)) = last.parameters.as_mut().and_then(|v| v.last_mut()) {
                    reasign.ptr = ptr_type;
                }
            }
            Some(TableTypes::FunctionCall(fc)) => {
                if let Some(TableTypes::Reasingment(reasign)) = fc.parameters.as_mut().and_then(|v| v.last_mut()) {
                    reasign.ptr = ptr_type;
                }
            }
            _ => {}
        }
    }

    fn check_parameters(&mut self, entry_info: Entry, fc_params_len: usize, fc_target: usize) {
        let token = entry_info.token.clone();
        let word = entry_info.word.clone();
        let index = entry_info.index.clone();

        let (expected_type, expected_ptr) = if let Some(TableTypes::Function(f)) = self.table.get(fc_target) {
            f.parameters.as_ref()
                .and_then(|params| params.get(fc_params_len))
                .map(|p| match p {
                    TableTypes::Variable(v) => (Some(v.token_type.clone()), v.ptr.clone()),
                    _ => (None, None),
                })
                .unwrap_or((None, None))
        } else {
            (None, None)
        };

        if let Some(expected_type) = expected_type {
            let actual_type = token.literal_type().or_else(|| self.declared_type_of(&index));

            if let Some(actual_type) = actual_type {
                if actual_type != expected_type {
                    let error = Error {
                        msg: format!("Type mismatch for argument {} of function call: expected {:?}, got {:?}", fc_params_len + 1, expected_type, actual_type),
                        line: word.line.unwrap_or(0), 
                        char: word.char_num.unwrap_or(0)
                    };

                    self.error_messages.push(error);
                }
            }

            let actual_ptr = self.ptr_type.clone();
            let ptr_ok = match (&expected_ptr, &actual_ptr) {
                (Some(PointerType::Pointer), Some(PointerType::Reference)) => true,
                (None, Some(PointerType::Pointer)) => true,
                (None, None) => true,
                (a, b) => a == b,
            };

            if !ptr_ok {
                let expected_str = match &expected_ptr {
                    Some(PointerType::Pointer) => "pointer (*T) — pass with &var",
                    Some(PointerType::Reference) => "reference (&T)",
                    None => "plain value",
                };
                let actual_str = match &actual_ptr {
                    Some(PointerType::Pointer) => "dereference (*var)",
                    Some(PointerType::Reference) => "address-of (&var)",
                    None => "plain value",
                };

                let error = Error {
                    msg: format!("Pointer kind mismatch for argument {} of function call: expected {}, got {}", fc_params_len + 1, expected_str, actual_str),
                    line: word.line.unwrap_or(0), 
                    char: word.char_num.unwrap_or(0)
                };

                self.error_messages.push(error);
            }
        }
    }

    fn declared_type_of(&self, index: &Option<(usize, Scope, ResolveType)>) -> Option<TokenType> {
        let (idx, scope, _) = index.as_ref()?;

        let entry = match scope {
            Scope::Root => self.table.get(*idx),
            Scope::Function => self.table.iter().find_map(|t| {
                if let TableTypes::Function(f) = t {
                    f.table.get(*idx)
                } else {
                    None
                }
            }),
            Scope::Parameter => self.table.iter().find_map(|t| {
                if let TableTypes::Function(f) = t {
                    f.parameters.as_ref().and_then(|p| p.get(*idx))
                } else {
                    None
                }
            }),
            Scope::EnbedFunc => {
                todo!()
            }
        };

        match entry {
            Some(TableTypes::Variable(v)) => Some(v.token_type.clone()),
            _ => None,
        }
    }

    fn add_caller_info(new_entry: &mut TableTypes, index: Option<(usize, Scope, ResolveType)>, call_word: &Word) -> Option<(usize, TokenType)> {
        let word = call_word.word.clone();
        let Some((idx, scope, resolve_type)) = index else { return None };
        let is_func = resolve_type == ResolveType::Function;

        match new_entry {
            TableTypes::FunctionCall(fc) => {
                Self::add_caller_info_on_call(fc, Some((idx, scope, resolve_type)), word);
                None
            }
            TableTypes::Variable(var) if is_func => {
                let var_type = var.token_type.clone();
                Self::promote_pending_var_to_call(var, idx, word, scope);
                Some((idx, var_type))
            }
            TableTypes::Reasingment(reasign) if is_func => {
                let reasign_type = reasign.token_type.clone();
                if let Some(last) = reasign.parameters.as_mut().and_then(|p| p.last_mut()) {
                    if let TableTypes::Reasingment(_) = last {
                        *last = TableTypes::FunctionCall(FunctionCall {
                            target: idx, parameters: None, name: word, scope: scope,
                        });
                    }
                } else {
                    reasign.parameters.get_or_insert_with(Vec::new).push(
                        TableTypes::FunctionCall(FunctionCall { target: idx, parameters: None, name: word, scope: scope })
                    );
                }
                if reasign_type != TokenType::Unknow { Some((idx, reasign_type)) } else { None }
            }
            TableTypes::Return(ret) if is_func => {
                if let Some(var) = ret.value.as_mut() {
                    Self::promote_pending_var_to_call(var, idx, word, scope);
                }
                None
            }
            _ => None,
        }
    }

    fn check_func_return_type(&mut self, func_idx: usize, expected_type: TokenType, word: &Word) {
        let return_type = self.table.get(func_idx).and_then(|e| {
            if let TableTypes::Function(f) = e { f.return_type.clone() } else { None }
        });

        if let Some(ret_type) = return_type {
            if ret_type != expected_type {
                let error = Error {
                    msg: format!( "Type mismatch: variable is '{}' but function '{}' returns '{}'",
                        expected_type.to_str(), word.word, ret_type.to_str()),
                    line: word.line.unwrap_or(0),
                    char: word.char_num.unwrap_or(0)


                };

                self.error_messages.push(error);
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

    fn add_caller_info_on_call(fc: &mut FunctionCall, index: Option<(usize, Scope, ResolveType)>, word: String) {
        let Some((idx, scope, resolve_type)) = index else { return };
        let Some(last) = fc.parameters.as_mut().and_then(|p| p.last_mut()) else { return };
        let TableTypes::Reasingment(v) = last else { return };
        let is_func = resolve_type == ResolveType::Function;

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

        if !TokenType::is_value(token.clone()) {
            self.add_entry(token.clone());
        } else if index.is_some() {
            self.set_value = true;

            let (idx, scope, resolve_type) = index.expect("Error finding the index of the value to be reasign");
            
            if resolve_type == ResolveType::Function {
                let func_call = FunctionCall {
                    target: idx,
                    parameters: None,
                    name: word.word,
                    scope: scope,
                };
                self.active_table().push(TableTypes::FunctionCall(func_call));
            } else if resolve_type == ResolveType::Struct {
                self.add_entry(TokenType::StructDef(word.word));
            } else {
                let target_type = match scope {
                    Scope::Root => self.table.get(idx),
                    Scope::Function | Scope::Parameter => self.table.iter().rev().find_map(|t| {
                        if let TableTypes::Function(f) = t { f.table.get(idx) } else { None }
                    }),
                    Scope::EnbedFunc => todo!()
                };
                let token_type = match target_type {
                    Some(TableTypes::Variable(v)) => v.token_type.clone(),
                    _ => TokenType::Unknow,
                };

                let (name, array_index) = match crate::variable_types::parse_array_syntax(&word.word) {
                    Some((arr_name, idx_str)) => (arr_name, Some(idx_str)),
                    None => (word.word, None),
                };

                let reasign = Reasingment {
                    target: idx,
                    target_scope: scope,
                    parameters: None,
                    name,
                    token_type,
                    ptr: self.ptr_type.clone(),
                    array_index,
                };
                self.active_table().push(TableTypes::Reasingment(reasign));
            }
        } else if let Some(inst_name) = self.current_struct_init.clone() {
            let mangled = format!("{}__{}", inst_name, word.word);
            let inst_index = Self::resolve_in_chain(
                &inst_name, &mut self.table, 0, self.max_nesting, self.defining_parameters,
            ).or_else(|| self.resolve_in_parameters(&inst_name));

            let (target, scope) = inst_index.map(|(i, s, _)| (i, s)).unwrap_or((0, Scope::Root));

            let reasign = Reasingment {
                target,
                target_scope: scope,
                parameters: None,
                name: mangled,
                token_type: TokenType::Unknow,
                ptr: self.ptr_type.clone(),
                array_index: None,
            };
            self.active_table().push(TableTypes::Reasingment(reasign));
            self.set_value = true;
        } else {
            let error = Error {
                msg: format!("Undefined symbol: {}", word.word),
                line: word.line.unwrap_or(0), 
                char: word.char_num.unwrap_or(0)
            };

            self.error_messages.push(error);
        }
    }

    fn pending_call_in(entry: &TableTypes) -> Option<&FunctionCall> {
        match entry {
            TableTypes::Variable(v) => v.pending_call(),
            TableTypes::Reasingment(re) => re.pending_call(),
            TableTypes::Return(r) => r.value.as_ref().and_then(|v| v.pending_call()),
            _ => None,
        }
    }

    fn pending_call_in_mut(entry: &mut TableTypes) -> Option<&mut FunctionCall> {
        match entry {
            TableTypes::Variable(v) => v.pending_call_mut(),
            TableTypes::Reasingment(re) => re.pending_call_mut(),
            TableTypes::Return(r) => r.value.as_mut().and_then(|v| v.pending_call_mut()),
            _ => None,
        }
    }

    fn tokenize_line(&mut self, line: &Vec<Word>) {
        let mut normalized_words: Vec<Word> = Vec::new();
        let mut i = 0;
        while i < line.len() {
            let mut w = line[i].clone();
            if w.word.len() > 1 && w.word.ends_with(',') && !w.word.starts_with('"') {
                w.word.pop();
            }
            if w.word == "," || w.word == "{" || w.word == "}" {
                i += 1;
                continue;
            }

            if i + 3 < line.len() && line[i+1].word == "[" && line[i+3].word == "]" {
                w.word = format!("{}[{}]", w.word, line[i+2].word);
                i += 4;
            } else if i + 1 < line.len() && line[i+1].word.starts_with('[') && line[i+1].word.ends_with(']') {
                w.word = format!("{}{}", w.word, line[i+1].word);
                i += 2;
            } else {
                i += 1;
            }

            if w.word.is_empty() { 
                return;
            }
            
            if w.word.len() > 2 && w.word.ends_with("++") {
                let mut var_word = w.clone();
                var_word.word = w.word[..w.word.len()-2].to_string();
                let mut op_word = w.clone();
                op_word.word = "++".to_string();
                normalized_words.push(var_word);
               normalized_words.push(op_word);
            } else if w.word.len() > 2 && w.word.ends_with("--") {
                let mut var_word = w.clone();
                var_word.word = w.word[..w.word.len()-2].to_string();
                let mut op_word = w.clone();
                op_word.word = "--".to_string();
                normalized_words.push(var_word);
                normalized_words.push(op_word);
            } else {
                let parts: Vec<&str> = w.word.split("::").collect();
                let n = parts.len();
                let mut char_count = 0;

                for (i, word) in parts.iter().enumerate() {
                    normalized_words.push(Word {
                        word: word.to_string(),
                        line: w.line,
                        char_num: Some(w.char_num.unwrap_or(0) + char_count),
                    });
                    char_count += word.len();

                    if i < n - 1 {
                        normalized_words.push(Word {
                            word: "::".to_string(),
                            line: w.line,
                            char_num: Some(w.char_num.unwrap_or(0) + char_count),
                        });
                        char_count += 2;
                    }
                }
            }
        }

        for word in normalized_words {
            self.tokenize_word(word);
        }
    }

    fn print_errors(&self, code: Vec<String>, file_name: String) {
    for error in self.error_messages.iter() {
        println!("\n[Error]: {}", error.msg);
        println!("--> {} line:{}, char pos :{}",
            file_name,
            error.line.to_string(),
            error.char.to_string()
        );
        println!();

        let error_line = error.line - 1 as usize; 
        let total_lines = code.len();

        let mut start = if error_line > 2 { error_line - 2 } else { 1 };
        let mut end = start + 4;

        if end > total_lines {
            end = total_lines;
            start = if end > 4 { end - 4 } else { 1 };
        }

        let width = (end + 1).to_string().len();

        for i in start..=end {
            let line_content = &code[i - 1];
            println!("{:>width$} | {}", i + 1, line_content, width = width);

            if i == error_line {
                let (word_start, word_len) = Self::word_span_at(line_content, error.char as usize);
                let gutter_len = width + 3; 
                let leading_spaces = " ".repeat(gutter_len + word_start.saturating_sub(1));
                let squiggles = "~".repeat(word_len.max(1));
                println!("{}{}", leading_spaces, squiggles);
            }
        }
        println!();
    }
}

    fn word_span_at(line: &str, pos: usize) -> (usize, usize) {
        let chars: Vec<char> = line.chars().collect();
        if chars.is_empty() {
            return (pos, 1);
        }

        let idx = pos.saturating_sub(1).min(chars.len() - 1);
        let is_word_char = |c: char| c.is_alphanumeric() || c == '_';

        let mut start = idx;
        while start > 0 && is_word_char(chars[start - 1]) {
            start -= 1;
        }

        let mut end = idx;
        while end < chars.len() && is_word_char(chars[end]) {
            end += 1;
        }

        if end == start {
            end = start + 1;
        }

        (start + 1, end - start)
    }

    pub fn _print(&self) {
        println!("\n---------------------- \n");
        println!("{}", Self::_format_table(&self.table, 0));
        println!("\n---------------------- \n");
    }
 
    fn _format_table(table: &Vec<TableTypes>, level: usize) -> String {
        table
            .iter()
            .map(|item| Self::_format_item(item, level))
            .collect::<Vec<_>>()
            .join("\n")
    }
 
    fn _format_item(item: &TableTypes, level: usize) -> String {
        let p = "    ".repeat(level);
        match item {
            TableTypes::Variable(v) => {
                let array_note = if v.is_array {
                    format!(" [array; size={:?}]", v.array_size)
                } else {
                    String::new()
                };
                format!(
                    "{p}var {}{}: {:?} = {}{}",
                    Self::_format_ptr(&v.ptr),
                    v.name.clone().unwrap_or_else(|| "_".to_string()),
                    v.token_type,
                    Self::_format_values(&v.value),
                    array_note,
                )
            }
 
            TableTypes::Function(f) => {
                let ret = f
                    .return_type
                    .as_ref()
                    .map(|t| t.to_str())
                    .unwrap_or_else(|| "void".to_string());
                let header = format!(
                    "{p}fn {}() -> {} {{",
                    f.name.clone().unwrap_or_else(|| "_".to_string()),
                    ret
                );
                let body = Self::_format_table(&f.table, level + 1);
                format!("{header}\n{body}\n{p}}}")
            }
 
            TableTypes::Reasingment(r) => {
                let params = r
                    .parameters
                    .as_ref()
                    .map(|p| p.iter().map(Self::_format_item_inline).collect::<Vec<_>>().join(", "))
                    .unwrap_or_default();
                let idx = r
                    .array_index
                    .as_ref()
                    .map(|i| format!("[{}]", i))
                    .unwrap_or_default();
                format!(
                    "{p}{}{}{} = {}  ({:?}, scope={:?}, target={})",
                    Self::_format_ptr(&r.ptr),
                    r.name,
                    idx,
                    params,
                    r.token_type,
                    r.target_scope,
                    r.target,
                )
            }
 
            TableTypes::FunctionCall(fc) => {
                format!(
                    "{p}{}  (scope={:?}, target={})",
                    Self::_format_func_call_inline(fc),
                    fc.scope,
                    fc.target
                )
            }
 
            TableTypes::Conditional(c) => {
                let cond = c.condition.iter().map(Self::_format_item_inline).collect::<Vec<_>>().join(" ");
                let header = format!("{p}if {} {{", cond);
                let body = Self::_format_table(&c.table, level + 1);
                format!("{header}\n{body}\n{p}}}")
            }
 
            TableTypes::Loop(l) => {
                let cond = l.condition.iter().map(Self::_format_item_inline).collect::<Vec<_>>().join(" ");
                let header = format!("{p}while {} {{", cond);
                let body = Self::_format_table(&l.table, level + 1);
                format!("{header}\n{body}\n{p}}}")
            }
 
            TableTypes::Return(r) => {
                let val = r
                    .value
                    .as_ref()
                    .map(|v| Self::_format_values(&v.value))
                    .unwrap_or_else(|| "<none>".to_string());
                format!("{p}return {}", val)
            }
 
            TableTypes::StructLiteral(s) => {
                let header = format!("{p}struct {} {{", s.name);
                let mut body = Self::_format_table(&s.arguments, level + 1);
                if !s.functions.is_empty() {
                    body.push_str(&format!("\n{p}    -- methods --\n"));
                    body.push_str(&Self::_format_table(&s.functions, level + 1));
                }
                format!("{header}\n{body}\n{p}}}")
            }
 
            TableTypes::Unknown => {
                format!("{p}<unknown>")
            }
        }
    }
 
    fn _format_item_inline(item: &TableTypes) -> String {
        match item {
            TableTypes::Variable(v) => {
                if let Some(name) = &v.name && name != "_" {
                    name.clone()
                } else {
                    Self::_format_values(&v.value)
                }
            }
            TableTypes::FunctionCall(fc) => Self::_format_func_call_inline(fc),
            TableTypes::Reasingment(r) => r.name.clone(),
            _ => "<expr>".to_string(),
        }
    }
 
    fn _format_func_call_inline(fc: &FunctionCall) -> String {
        let params = fc
            .parameters
            .as_ref()
            .map(|p| p.iter().map(Self::_format_item_inline).collect::<Vec<_>>().join(", "))
            .unwrap_or_default();
        format!("{}({})", fc.name, params)
    }
 
    fn _format_ptr(ptr: &Option<PointerType>) -> &'static str {
        match ptr {
            Some(PointerType::Pointer) => "*",
            Some(PointerType::Reference) => "&",
            None => "",
        }
    }
 
    fn _format_value(value: &Value) -> String {
        match value {
            Value::Var(name) => name.clone(),
            Value::Deref(name) => format!("*{}", name),
            Value::Ref(name) => format!("&{}", name),
            Value::Index(name, idx) => format!("{}[{}]", name, idx),
            Value::FuncCall(fc) => Self::_format_func_call_inline(fc),
        }
    }
 
    fn _format_values(values: &Option<Vec<Value>>) -> String {
        match values {
            Some(v) if !v.is_empty() => v.iter().map(Self::_format_value).collect::<Vec<_>>().join(" "),
            _ => "<none>".to_string(),
        }
    }
}

pub fn analyze_semantically(stack: Vec<Block>, file_str: Vec<String>, file_name: String, debug: bool) -> Result<Vec<TableTypes>, usize>{
    let mut semantic_analyzer: SemanticAnalyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze(stack);

    if debug {
        semantic_analyzer._print();
    }
    let len = semantic_analyzer.error_messages.len();
    if len > 0 {
        semantic_analyzer.print_errors(file_str, file_name);
        return Err(len);
    }

    Ok(semantic_analyzer.table)
}
