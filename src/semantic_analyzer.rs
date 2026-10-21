mod debug;
mod normalize;
mod asm;
mod resolve;
mod cursor;
pub mod table;
pub mod tokens;

pub use table::*;
pub use tokens::*; 

use crate::enbeded_funcs::FUNCTIONS;
use crate::parser::{Block, Word};
use crate::report;
use crate::warn;
use crate::variable_types::{
    FunctionCall, PointerType, Reasingment, Types, Value, Variable,
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

    fn eliminate_instance_placeholder(&mut self, name: &str) {
        match self.active_table().last_mut() {
            Some(TableTypes::FunctionCall(fc)) => {
                if let Some(params) = fc.parameters.as_mut() {
                    if matches!(params.last(), Some(TableTypes::Reasingment(r)) if r.name == name && r.parameters.as_ref().map_or(true, |p| p.is_empty()))
                    {
                        params.pop();
                        return;
                    }
                }
            }
            Some(TableTypes::Reasingment(reasing)) => {
                if let Some(params) = reasing.parameters.as_mut() {
                    if matches!(params.last(), Some(TableTypes::Reasingment(r)) if r.name == name && r.parameters.as_ref().map_or(true, |p| p.is_empty()))
                    {
                        params.pop();
                        return;
                    }
                } else if reasing.name == name && reasing.parameters.as_ref().map_or(true, |p| p.is_empty()) {
                    self.modifiers.push(Modifiers::StandAloneStructCall);
                    //self.active_table().pop();
                }
            }
            Some(TableTypes::Variable(v)) => {
                if let Some(vals) = v.value.as_mut() {
                    if matches!(vals.last(), Some(Value::Var(n)) if n == name) {
                        vals.pop();
                    }
                }
            }
            Some(TableTypes::Return(r)) => {
                if let Some(vals) = r.value.as_mut().and_then(|v| v.value.as_mut()) {
                    if matches!(vals.last(), Some(Value::Var(n)) if n == name) {
                        vals.pop();
                    }
                }
            }
            Some(TableTypes::Conditional(c)) => {
                if matches!(c.condition.last(), Some(TableTypes::Reasingment(r)) if r.name == name && r.parameters.as_ref().map_or(true, |p| p.is_empty()))
                {
                    c.condition.pop();
                }
            }
            Some(TableTypes::Loop(l)) => {
                if matches!(l.condition.last(), Some(TableTypes::Reasingment(r)) if r.name == name && r.parameters.as_ref().map_or(true, |p| p.is_empty()))
                {
                    l.condition.pop();
                }
            }
            _ => {}
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
                    if let Value::Var(name) = vals { Some(name.clone()) } else { v.name.clone() }
                } else {
                    v.name.clone()
                }
            }
            Some(TableTypes::Return(r)) => {
                let var = r.value.as_ref()?;
                match var.value.as_ref().and_then(|v| v.last()) {
                    Some(Value::Var(name)) => Some(name.clone()),
                    _ => var.name.clone(),
                }
            }
            Some(TableTypes::Conditional(c)) => match c.condition.last() {
                Some(TableTypes::Reasingment(r)) => Some(r.name.clone()),
                Some(TableTypes::Variable(v)) => v.name.clone(),
                _ => None,
            },
            Some(TableTypes::Loop(l)) => match l.condition.last() {
                Some(TableTypes::Reasingment(r)) => Some(r.name.clone()),
                Some(TableTypes::Variable(v)) => v.name.clone(),
                _ => None,
            },
            _ => None,
        }
    }

    fn handle_argument(
        &mut self, entry_info: Entry, in_reasignment: bool, in_function_call: bool, in_nested_call: bool, expected_fc_params: usize,
        fc_params_len: usize, fc_target: usize,
    ) {
        let token = entry_info.token.clone();
        let word = entry_info.word.clone();
        let index = entry_info.index.clone();

        self.set_return_value = false;

        if token == TokenType::PlusPlus || token == TokenType::MinusMinus {
            self.set_value = false;
        } else {
            self.set_value &= TokenType::is_operator(token.clone());
        }
        let in_call_or_reasign = in_reasignment || in_function_call || in_nested_call;

        if in_function_call || in_nested_call {
            if fc_params_len >= expected_fc_params {
                report!(
                    self.file_name.clone(),
                    word.line.unwrap_or(0),
                    word.char_num.unwrap_or(0),
                    "Too many arguments for function call. Expected: {}, Found: {}",
                    expected_fc_params,
                    fc_params_len + 1,
                );
            } else {
                self.check_parameters(entry_info, fc_params_len, fc_target);
            }
        }

        if in_call_or_reasign
            && token == TokenType::Unknow
            && index.is_none()
            && !self.modifiers.contains(&Modifiers::InForLoop)
            && !word.word.starts_with("self[")
            && word.word != "self"
        {
            report!(self.file_name.clone(), word.line.unwrap_or(0), word.char_num.unwrap_or(0), "Undefined symbol: {}", word.word,);
        }

        let rhs_ptr_type = self.ptr_type.clone();
        let mut mismatch: Option<(TokenType, Option<TokenType>, Option<PointerType>)> = None;
        match self.active_table().last_mut() {
            Some(TableTypes::Variable(var)) if !in_nested_call => {
                let value_type = TokenType::from_str(&word.word.clone()).literal_type();

                if var.name.is_some()
                    && let Some(vt) = value_type
                {
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
                report!(self.file_name.clone(), word.line.unwrap_or(0), word.char_num.unwrap_or(0), "There wasn't a last entry",);

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
            report!(
                self.file_name.clone(),
                word.line.unwrap_or(0),
                word.char_num.unwrap_or(0),
                "Type mismatch. Expected: {}{}, Found: {}",
                ptr_type_str,
                expected.to_str(),
                found.unwrap_or(TokenType::Unknow).to_str()
            );
        }

        let new_entry = match self.active_table().last_mut() {
            Some(entry) => entry,
            None => {
                report!(self.file_name.clone(), word.line.unwrap_or(0), word.char_num.unwrap_or(0), "There wasn't a last entry");
                return;
            }
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
            f.parameters
                .as_ref()
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
                    report!(
                        self.file_name.clone(),
                        word.line.unwrap_or(0),
                        word.char_num.unwrap_or(0),
                        "Type mismatch for argument {} of function call: expected {:?}, got {:?}",
                        fc_params_len + 1,
                        expected_type,
                        actual_type,
                    );
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

                report!(
                    self.file_name.clone(),
                    word.line.unwrap_or(0),
                    word.char_num.unwrap_or(0),
                    "Pointer kind mismatch for argument {} of function call: expected {}, got {}",
                    fc_params_len + 1,
                    expected_str,
                    actual_str,
                );
            }
        }
    }

    fn declared_type_of(&self, index: &Option<(usize, Scope, ResolveType)>) -> Option<TokenType> {
        let (idx, scope, _) = index.as_ref()?;

        let entry = match scope {
            Scope::Root => self.table.get(*idx),
            Scope::Function => self.table.iter().find_map(|t| if let TableTypes::Function(f) = t { f.table.get(*idx) } else { None }),
            Scope::Parameter => self
                .table
                .iter()
                .find_map(|t| if let TableTypes::Function(f) = t { f.parameters.as_ref().and_then(|p| p.get(*idx)) } else { None }),
            Scope::EnbedFunc => {
                todo!()
            }
        };

        match entry {
            Some(TableTypes::Variable(v)) => Some(v.token_type.clone()),
            _ => None,
        }
    }

    fn add_caller_info(
        new_entry: &mut TableTypes, index: Option<(usize, Scope, ResolveType)>, call_word: &Word,
    ) -> Option<(usize, TokenType)> {
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
                            target: idx,
                            parameters: None,
                            name: word,
                            scope: scope,
                        });
                    }
                } else {
                    reasign.parameters.get_or_insert_with(Vec::new).push(TableTypes::FunctionCall(FunctionCall {
                        target: idx,
                        parameters: None,
                        name: word,
                        scope: scope,
                    }));
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
        let return_type = self.table.get(func_idx).and_then(|e| if let TableTypes::Function(f) = e { f.return_type.clone() } else { None });

        if let Some(ret_type) = return_type {
            if ret_type != expected_type {
                report!(
                    self.file_name.clone(),
                    word.line.unwrap_or(0),
                    word.char_num.unwrap_or(0),
                    "Type mismatch: variable is '{}' but function '{}' returns '{}'",
                    expected_type.to_str(),
                    word.word,
                    ret_type.to_str(),
                );
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
        } else if let Some(inst_name) = self.current_struct_init.clone() {
            self.current_struct_init = None;
            self.set_value = false;

            let inst_index = Self::resolve_in_chain(&inst_name, &mut self.table, 0, self.max_nesting, self.defining_parameters)
                .or_else(|| self.resolve_in_parameters(&inst_name));

            let (target, scope) = inst_index.clone().map(|(i, s, _)| (i, s)).unwrap_or((0, Scope::Root));

            enum Member {
                Field(usize),
                Method(usize),
                Missing(String),
                Unresolved,
            }

            let struct_def = if inst_name == "self" {
                self.table.iter().rev().find(|t| matches!(t, TableTypes::StructLiteral(_)))
            } else {
                Self::struct_name_of(&self.table, &inst_name)
                    .and_then(|n| self.table.iter().find(|t| matches!(t, TableTypes::StructLiteral(s) if s.name == n)))
            };

            let mut struct_def_name = String::new();
            let member = match struct_def {
                Some(TableTypes::StructLiteral(s)) => {
                    struct_def_name = s.name.clone();
                    if let Some(pos) = s
                        .arguments
                        .iter()
                        .position(|a| matches!(a, TableTypes::Variable(v) if v.name.as_deref() == Some(word.word.as_str())))
                    {
                        Member::Field(pos)
                    } else if let Some(pos) = s
                        .functions
                        .iter()
                        .position(|f| matches!(f, TableTypes::Function(func) if func.name.as_deref() == Some(word.word.as_str())))
                    {
                        Member::Method(pos)
                    } else {
                        Member::Missing(s.name.clone())
                    }
                }
                _ => Member::Unresolved,
            };

            match member {
                Member::Field(pos) => {
                    let reasign = Reasingment {
                        target,
                        target_scope: scope,
                        parameters: None,
                        name: inst_name,
                        token_type: TokenType::Unknow,
                        ptr: self.ptr_type.clone(),
                        array_index: Some(pos.to_string()),
                    };

                    self.push_member_entry(TableTypes::Reasingment(reasign));
                }
                Member::Method(pos) => {
                    let name = format!("{}.{}", struct_def_name, &word.word.clone());
                    let reasingment = Reasingment {
                        target: target,
                        target_scope: scope,
                        parameters: None,
                        name: inst_name,
                        token_type: TokenType::StructDef(struct_def_name.clone()),
                        ptr: Some(PointerType::Reference),
                        array_index: None,
                    };

                    let call = FunctionCall {
                        target: pos,
                        parameters: Some(vec![TableTypes::Reasingment(reasingment)]),
                        name: name,
                        scope: Scope::Function,
                    };
                    self.push_member_entry(TableTypes::FunctionCall(call));
                }
                Member::Missing(struct_name) => {
                    report!(
                        self.file_name.clone(),
                        word.line.unwrap_or(0),
                        word.char_num.unwrap_or(0),
                        "'{}' is neither a field nor a method of struct '{}'",
                        word.word,
                        struct_name,
                    );
                }
                Member::Unresolved => {
                    let reasign = Reasingment {
                        target,
                        target_scope: scope,
                        parameters: None,
                        name: inst_name,
                        token_type: TokenType::Unknow,
                        ptr: self.ptr_type.clone(),
                        array_index: None,
                    };
                    self.push_member_entry(TableTypes::Reasingment(reasign));
                    self.set_value = true;
                }
            }
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
                    Scope::Function | Scope::Parameter => {
                        self.table.iter().rev().find_map(|t| if let TableTypes::Function(f) = t { f.table.get(idx) } else { None })
                    }
                    Scope::EnbedFunc => todo!(),
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
        } else {
            report!(self.file_name.clone(), word.line.unwrap_or(0), word.char_num.unwrap_or(0), "Undefined symbol: {}", word.word,);
        }
    }

    fn struct_name_of(table: &Vec<TableTypes>, name: &str) -> Option<String> {
        table.iter().rev().find_map(|entry| match entry {
            TableTypes::Variable(v) if v.name.as_deref() == Some(name) => match &v.token_type {
                TokenType::StructDef(s) => Some(s.clone()),
                _ => None,
            },
            TableTypes::Function(f) => {
                Self::struct_name_of(&f.table, name).or_else(|| f.parameters.as_ref().and_then(|p| Self::struct_name_of(p, name)))
            }
            TableTypes::Conditional(c) => Self::struct_name_of(&c.table, name),
            TableTypes::Loop(l) => Self::struct_name_of(&l.table, name),
            TableTypes::StructLiteral(s) => Self::struct_name_of(&s.functions, name),
            _ => None,
        })
    }

    fn push_member_entry(&mut self, entry: TableTypes) {
        let last_finished = self.last_finished;
        let def_parameters = self.defining_parameters;

        if let Some(pos) = self.modifiers.iter().rposition(|e| matches!(e, Modifiers::StandAloneStructCall)) {
            self.modifiers.remove(pos);
            self.active_table().pop();
        }

        let leftover = match self.active_table().last_mut() {
            Some(last) => Self::attach_member(last, entry, last_finished, def_parameters),
            None => Some(entry),
        };

        if let Some(entry) = leftover {
            self.active_table().push(entry);
        }
    }

    fn attach_member(target: &mut TableTypes, entry: TableTypes, last_finished: bool, defining_parameters: bool) -> Option<TableTypes> {
        if let Some(fc) = Self::pending_call_in_mut(target) {
            fc.parameters.get_or_insert_with(Vec::new).push(entry);
            return None;
        }

        match target {
            TableTypes::FunctionCall(call) => {
                call.parameters.get_or_insert_with(Vec::new).push(entry);
                None
            }
            TableTypes::Reasingment(reasign) => {
                if let TokenType::StructDef(_) = reasign.token_type.clone()
                    && let TableTypes::Reasingment(reasing_entry) = entry
                {
                    reasign.token_type = TokenType::Unknow;
                    reasign.array_index = reasing_entry.array_index;
                } else {
                    reasign.parameters.get_or_insert_with(Vec::new).push(entry);
                }
                None
            }
            TableTypes::Variable(var) if !last_finished => match Self::member_to_values(entry) {
                Ok(values) => {
                    var.value.get_or_insert_with(Vec::new).extend(values);
                    None
                }
                Err(entry) => Some(entry),
            },
            TableTypes::Return(ret) => match ret.value.as_mut() {
                Some(var) => match Self::member_to_values(entry) {
                    Ok(values) => {
                        var.value.get_or_insert_with(Vec::new).extend(values);
                        None
                    }
                    Err(entry) => Some(entry),
                },
                None => Some(entry),
            },
            TableTypes::Conditional(c) => match c.condition.last_mut() {
                Some(last) => match Self::attach_member(last, entry, last_finished, defining_parameters) {
                    Some(entry) => {
                        c.condition.push(entry);
                        None
                    }
                    None => None,
                },
                None => {
                    c.condition.push(entry);
                    None
                }
            },
            TableTypes::Loop(l) => match l.condition.last_mut() {
                Some(last) => match Self::attach_member(last, entry, last_finished, defining_parameters) {
                    Some(entry) => {
                        l.condition.push(entry);
                        None
                    }
                    None => None,
                },
                None => {
                    l.condition.push(entry);
                    None
                }
            },
            _ => Some(entry),
        }
    }

    fn member_to_values(entry: TableTypes) -> Result<Vec<Value>, TableTypes> {
        match entry {
            TableTypes::FunctionCall(fc) => Ok(vec![Value::FuncCall(fc)]),
            TableTypes::Reasingment(r) => {
                let name = r.name;
                let value = match (r.array_index, r.ptr) {
                    (Some(idx), _) => Value::Index(name, idx),
                    (None, Some(PointerType::Reference)) => Value::Ref(name),
                    (None, Some(PointerType::Pointer)) => Value::Deref(name),
                    (None, None) => Value::Var(name),
                };
                Ok(vec![value])
            }
            TableTypes::Variable(mut v) => match v.value.take() {
                Some(vals) if !vals.is_empty() => Ok(vals),
                other => {
                    v.value = other;
                    Err(TableTypes::Variable(v))
                }
            },
            other => Err(other),
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
