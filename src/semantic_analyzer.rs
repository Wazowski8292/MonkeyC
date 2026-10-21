mod debug;
mod normalize;
mod asm;
mod resolve;
mod cursor;
mod members;
mod checks;
mod new_entry;
mod add_arguments;
pub mod table;
pub mod tokens;

pub use table::*;
pub use tokens::*; 

use crate::enbeded_funcs::FUNCTIONS;
use crate::parser::{Block, Word};
use crate::warn;
use crate::variable_types::{
    FunctionCall, PointerType,
};
use std::vec::Vec;

#[derive(Debug, PartialEq, Clone)]
enum Modifiers {
    InForLoop,
    StandAloneStructCall,
}

struct Entry {
    word: Word,
    token: TokenType,
    index: Option<(usize, Scope, ResolveType)>,
}

#[derive(Debug, PartialEq, Clone)]
struct SemanticAnalyzer {
    file_name: String,
    table: Vec<TableTypes>,
    set_value: bool,
    set_return_value: bool,
    defining_fn: bool,
    defining_parameters: bool,
    modifiers: Vec<Modifiers>,
    max_nesting: usize,
    ptr_type: Option<PointerType>,
    current_struct_init: Option<String>,
    last_finished: bool,
}

impl SemanticAnalyzer {
    pub fn new(file_name: String) -> Self {
        Self {
            file_name,
            table: vec![],
            set_value: false,
            set_return_value: false,
            defining_fn: false,
            defining_parameters: false,
            modifiers: vec![],
            max_nesting: 1,
            ptr_type: None,
            current_struct_init: None,
            last_finished: false,
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
                    if let Some(TableTypes::InlineAssembly(asm)) = self.active_table().last_mut() {
                        for block in blocks {
                            asm::add_inline_asm(asm, block.clone());
                        }
                    } else {
                        self.max_nesting += 1;
                        self.analyze(blocks.to_vec());
                        self.max_nesting -= 1;
                        self.current_struct_init = None;

                        let nesting_funtion_end = if matches!(self.table.last(), Some(TableTypes::StructLiteral(_))) { 1 } else { 0 };

                        if self.max_nesting <= nesting_funtion_end {
                            self.defining_fn = false
                        }
                    }
                }
                Block::Parameter(blocks) => {
                    self.defining_parameters = true;
                    self.analyze(blocks.to_vec());
                    self.defining_parameters = false;
                }
            }
        }
    }

    fn tokenize_line(&mut self, line: &Vec<Word>) {
        self.update_line_flags();

        let Some(mut words) = normalize::normalize_line(line) else { return };
        self.rewrite_self_fields(&mut words);

        for word in words {
            self.tokenize_word(word);
            self.last_finished = false;
        }
    }

    fn update_line_flags(&mut self) {
        if self.set_value || self.set_return_value {
            self.last_finished = false;
        } else {
            self.last_finished = !self.defining_parameters;
            self.set_value = false;
        }
    }

    fn rewrite_self_fields(&self, words: &mut Vec<Word>) {
        let mut i = 0;
        while i + 2 < words.len() {
            if words[i].word == "self" && words[i + 1].word == "::" {
                if let Some(pos) = self.self_field_index(&words[i + 2].word) {
                    words[i].word = format!("self[{}]", pos);
                    words.drain(i + 1..i + 3);
                }
            }
            i += 1;
        }
    }

    fn self_field_index(&self, field: &str) -> Option<usize> {
        let Some(TableTypes::StructLiteral(s)) = self.table.last() else { return None };
        s.arguments.iter().position(|a| matches!(a, TableTypes::Variable(v) if v.name.as_deref() == Some(field)))
    }

    fn tokenize_word(&mut self, mut word: Word) {
        let length = word.word.len();

        match word.word.clone().chars().next() {
            Some('&') if length > 1 => {
                self.ptr_type = Some(PointerType::Reference);
                word.word = word.word.replace("&", "");
            }
            Some('*') if length > 1 => {
                self.ptr_type = Some(PointerType::Pointer);
                word.word = word.word.replace("*", "");
            }
            _ => self.ptr_type = None,
        }

        let token = TokenType::from_str(&word.word);

        self.set_value |= TokenType::is_operator(token.clone());

        match token {
            TokenType::Equals => return,
            TokenType::ReturnType => {
                self.set_return_value = true;
                return;
            }
            TokenType::Dots => {
                let inst_name = self.find_preceding_instance_name();
                if let Some(name) = &inst_name {
                    self.eliminate_instance_placeholder(name);
                }
                self.current_struct_init = inst_name;
                return;
            }
            TokenType::Assembly => {
                warn!(
                    self.file_name.clone(),
                    word.line.unwrap_or(0).clone(),
                    word.char_num.unwrap_or(0).clone(),
                    "The Monkey C compiler does not verify asmembly, you must check it by your self and remeber that the asmbler will return an error if it is the case, but it will not go through this compiler"
                )
            }
            _ => {}
        }

        let index = self.resolve(word.word.clone());

        let mut is_fc = false;
        let mut in_nested_call = false;
        let mut fc_target = 0;
        let mut fc_scope = Scope::Root;
        let mut fc_params_len = 0;
        let mut fc_name = String::new();

        let defining_parameters = self.defining_parameters;

        match self.active_table().last() {
            Some(TableTypes::FunctionCall(fc)) if defining_parameters => {
                is_fc = true;
                fc_target = fc.target;
                fc_scope = fc.scope;
                fc_params_len = fc.parameters.as_ref().map_or(0, |p| p.len());
                fc_name = fc.name.clone();
            }
            Some(entry @ (TableTypes::Variable(_) | TableTypes::Return(_))) if defining_parameters => {
                if let Some(pending) = Self::pending_call_in(entry) {
                    in_nested_call = true;
                    fc_target = pending.target;
                    fc_params_len = pending.parameters.as_ref().map_or(0, |p| p.len());
                    fc_name = pending.name.clone();
                }
            }
            _ => {}
        }

        let mut expected_fc_params = 0;
        if is_fc || in_nested_call {
            if fc_scope == Scope::EnbedFunc {
                expected_fc_params = FUNCTIONS[fc_target].parameters.len();
            } else {
                let (table, is_method) = self.callee_table(&fc_name);
                if let Some(TableTypes::Function(f)) = table.get(fc_target) {
                    expected_fc_params = f.parameters.as_ref().map_or(0, |p| p.len()) + is_method as usize;
                }
            }
        }

        let entry = Entry {
            word: word.clone(),
            token: token,
            index: index,
        };

        let in_reasignment = matches!(self.active_table().last(), Some(TableTypes::Reasingment(_))) && !self.last_finished;
        let in_function_call = matches!(self.active_table().last(), Some(TableTypes::FunctionCall(_))) && self.defining_parameters;
        let in_argument = self.get_if_in_argument(in_nested_call.clone(), in_reasignment.clone(), in_function_call);

        if in_argument && self.current_struct_init.is_none() {
            self.handle_argument(entry, in_reasignment, in_function_call, in_nested_call, expected_fc_params, fc_params_len, fc_target);
        } else {
            self.handle_new_entry(entry);
        }
    }

    fn get_if_in_argument(&mut self, in_nested_call: bool, in_reasignment: bool, in_function_call: bool) -> bool {
        let in_conditional = matches!(self.active_table().last(), Some(TableTypes::Conditional(_))) && self.defining_parameters;
        let in_for_loop = matches!(
            self.active_table().last(),
            Some(TableTypes::Loop(loop_node)) if matches!(loop_node.token_type, TokenType::ForLoop)
        );

        let pending_param = self.current_function().filter(|f| f.table.is_empty() && f.name.is_some()).map(|f| {
            matches!(
                f.parameters.as_ref().and_then(|p| p.last()),
                Some(TableTypes::Variable(v)) if v.name.is_none()
            )
        });

        if let Some(false) = pending_param {
            self.last_finished = true;
        }

        if in_for_loop && !self.modifiers.contains(&Modifiers::InForLoop) {
            self.modifiers.push(Modifiers::InForLoop);
        } else if !in_for_loop && let Some(index) = self.modifiers.iter().position(|value| *value == Modifiers::InForLoop) {
            self.modifiers.remove(index);
        }

        let in_call = in_reasignment || in_function_call || in_conditional || in_nested_call;

        !self.last_finished || self.set_value || self.set_return_value || in_call
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
}

pub fn analyze_semantically(stack: Vec<Block>, file_name: String, debug: bool) -> Vec<TableTypes> {
    let mut semantic_analyzer: SemanticAnalyzer = SemanticAnalyzer::new(file_name);
    semantic_analyzer.analyze(stack);

    if debug {
        debug::print(&semantic_analyzer.table.clone());
        //println!("{:#?}", semantic_analyzer.table);
    }

    semantic_analyzer.table
}
