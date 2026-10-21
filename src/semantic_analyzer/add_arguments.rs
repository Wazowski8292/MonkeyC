use super::{SemanticAnalyzer, TableTypes, ResolveType, Entry, TokenType, Scope, Modifiers, Word};
use crate::variable_types::{FunctionCall, Variable, PointerType, Value, Types};
use crate::report;

impl SemanticAnalyzer {
    pub fn handle_argument(
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
