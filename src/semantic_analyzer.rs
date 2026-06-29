use crate::parser::Block;
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
        token == TokenType::UNKNOW || token == TokenType::INTEGER_LITERAL || token == TokenType::FLOAT_LITERAL
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
         matches!(TokenType::from_str(&arg), TokenType::UNKNOW)
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

enum ActiveTable {
    Root,
    FunctionTable,
    FunctionParameters,
    ReassignmentParameters,
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

    fn resolve(&mut self, name: String) -> Option<(usize, Scope)> {
        let local_result = self.active_table().iter().position(|entry| match entry {
            TableTypes::VARIABLE(v) => v.name == Some(name.clone()),
            TableTypes::FUNCTION(f) => f.name == Some(name.clone()),
            _ => false,
        });

        if let Some(idx) = local_result {
            if !self.defining_fn {
                return Some((idx, Scope::Root));
            }
            return Some((idx, Scope::Function));
        }

        // Search function parameters if we're inside a function
        if self.defining_fn {
            if let Some(TableTypes::FUNCTION(f)) = self.table.last() {
                if let Some(params) = &f.parameters {
                    let param_result = params.iter().position(|entry| match entry {
                        TableTypes::VARIABLE(v) => v.name == Some(name.clone()),
                        _ => false,
                    });
                    if let Some(idx) = param_result {
                        return Some((idx, Scope::Parameter));
                    }
                }
            }
        }
        
        let root_result = self.table.iter().position(|entry| match entry {
            TableTypes::VARIABLE(v) => v.name == Some(name.clone()),
            TableTypes::FUNCTION(f) => f.name == Some(name.clone()),
            _ => false,
        });

        root_result.map(|idx| (idx, Scope::Root))
    }

    fn add_entry(&mut self, token: TokenType) {
        let table_type: TableTypes = TableTypes::from_token(token);
        self.active_table().push(table_type)
    }

    fn active_table(&mut self) -> &mut Vec<TableTypes> {
        let which = if self.defining_fn {
            match self.table.last() {
                Some(TableTypes::FUNCTION(_)) => {
                    if self.defining_parameters {
                        ActiveTable::FunctionParameters
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
            ActiveTable::ReassignmentParameters => {
                if let Some(TableTypes::REASIGNMENT(r)) = self.table.last_mut() {
                    r.parameters.get_or_insert_with(Vec::new)
                } else { unreachable!()}
            }
        }
    }

    fn tokenize_word(&mut self, word: String) {
        let token = TokenType::from_str(&word);

        if token == TokenType::EQUALS {
            self.set_value = true;
            return;
        }

        let index = self.resolve(word.clone());
        let last_finished = self.active_table().last().map_or(true, |e| e.finished_definition());
        let in_reasignment = matches!(self.active_table().last(), Some(TableTypes::REASIGNMENT(_)));
        
        if !last_finished || self.set_value || in_reasignment {
            self.handle_argument(word, token, index, in_reasignment);
        } else {
            self.handle_new_entry(word, token, index);
        }
    }

    fn handle_argument(&mut self, word: String, token: TokenType, index: Option<(usize, Scope)>, in_reasignment: bool) {
        if in_reasignment && token == TokenType::UNKNOW && index.is_none() {
            self.error_messages.push(format!("Undefined symbol: '{}'", word));
        } else {
            let new_entry = self.active_table().last_mut().unwrap();
            new_entry.add_arguments(word);
            if let TableTypes::REASIGNMENT(r) = new_entry && index.is_some() {
                if let Some(last) = r.parameters.as_deref_mut().unwrap_or(&mut []).last_mut() {
                    if let TableTypes::REASIGNMENT(v) = last {
                        let (idx, scope) = index.unwrap();
                        v.target = idx;
                        v.target_scope = scope;
                    }
                }
            }
        }

        self.set_value = false;
    }

    fn handle_new_entry(&mut self, word: String, token: TokenType, index: Option<(usize, Scope)>) {
        let is_known = token != TokenType::UNKNOW || index.is_some();

        if !TokenType::is_value(token) {
            self.add_entry(token);
        } else if is_known {
            self.set_value = true;

            let (idx, scope) = index.expect("Error finding the index of the value to be reasign");
            let reasign = Reasingment {
                target: idx,
                target_scope: scope,
                parameters: None,
            };

            self.active_table().push(TableTypes::REASIGNMENT(reasign));
        } else {
            self.error_messages.push(format!("Undefined symbol: '{}'", word));
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