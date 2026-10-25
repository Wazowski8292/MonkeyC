use super::{Entry, ResolveType, Scope, SemanticAnalyzer, TableTypes, TokenType, Word};
use crate::report;
use crate::variable_types::{FunctionCall, PointerType, Types, Value, Variable};

#[derive(Clone)]
pub struct ArgumentVariableInfo {
    pub in_reasignment: bool,
    pub in_function_call: bool,
    pub in_nested_call: bool,
    pub expected_fc_params: usize,
    pub fc_params_len: usize,
    pub fc_target: usize,
}

impl SemanticAnalyzer {
    pub fn handle_argument(&mut self, entry_info: Entry, argument_variable_info: ArgumentVariableInfo) {
        let token = entry_info.token.clone();
        let word = entry_info.word.clone();
        let index = entry_info.index.clone();

        self.update_set_variable(token.clone());
        self.check_parameter_error(entry_info.clone(), argument_variable_info.clone());
        self.check_pointer_type(entry_info.clone(), argument_variable_info.clone());

        // Peek at the last entry immutably to extract info needed for checks,
        // before taking a mutable borrow below.
        let (var_needs_name, var_type_for_literal, fn_needs_name, in_return_expr) =
            match self.active_table_ref().last() {
                Some(TableTypes::Variable(var)) => {
                    // var_needs_name: variable exists but hasn't received its name yet.
                    let needs_name = var.name.is_none() && token == TokenType::Unknow;
                    let var_type = if var.name.is_some() { Some(var.token_type.clone()) } else { None };
                    (needs_name, var_type, false, false)
                }
                Some(TableTypes::Function(f)) => (false, None, f.name.is_none(), false),
                Some(TableTypes::Return(_)) => (false, None, false, true),
                _ => (false, None, false, false),
            };

        // --- check: undefined symbol in return expression ---
        if in_return_expr {
            self.check_return_expression_defined(&word, &index);
        }

        // --- check: variable redeclaration ---
        if var_needs_name {
            self.check_redeclared_variable(&word.word, &word);
        }

        // --- check: duplicate function name ---
        if fn_needs_name {
            self.check_duplicate_function(&word.word, &word);
        }

        // --- check: literal type mismatch on variable declaration ---
        if let Some(var_type) = var_type_for_literal {
            self.check_variable_literal_type(&var_type, &word);
        }

        let new_entry = match self.active_table().last_mut() {
            Some(entry) => entry,
            None => {
                report!(self.file_name.clone(), word.line.unwrap_or(0), word.char_num.unwrap_or(0), "There wasn't a last entry");
                return;
            }
        };

        if argument_variable_info.in_nested_call {
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

    fn update_set_variable(&mut self, token: TokenType) {
        self.set_return_value = false;

        if token == TokenType::PlusPlus || token == TokenType::MinusMinus {
            self.set_value = false;
        } else {
            self.set_value &= TokenType::is_operator(token.clone());
        }
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
}
