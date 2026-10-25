use super::{ArgumentVariableInfo, Entry, Modifiers, ResolveType, Scope, SemanticAnalyzer, TableTypes, TokenType};
use crate::parser::Word;
use crate::report;
use crate::variable_types::PointerType;

impl SemanticAnalyzer {
    pub(super) fn check_parameters(&self, entry_info: Entry, fc_params_len: usize, fc_target: usize) {
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

    pub(super) fn check_func_return_type(&mut self, func_idx: usize, expected_type: TokenType, word: &Word) {
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

    pub fn check_parameter_error(&self, entry_info: Entry, argument_variable_info: ArgumentVariableInfo) {
        let token = entry_info.token.clone();
        let word = entry_info.word.clone();
        let index = entry_info.index.clone();

        let in_call_or_reasign =
            argument_variable_info.in_reasignment || argument_variable_info.in_function_call || argument_variable_info.in_nested_call;

        if argument_variable_info.in_function_call || argument_variable_info.in_nested_call {
            if argument_variable_info.fc_params_len >= argument_variable_info.expected_fc_params {
                report!(
                    self.file_name.clone(),
                    word.line.unwrap_or(0),
                    word.char_num.unwrap_or(0),
                    "Too many arguments for function call. Expected: {}, Found: {}",
                    argument_variable_info.expected_fc_params,
                    argument_variable_info.fc_params_len + 1,
                );
            } else {
                self.check_parameters(entry_info, argument_variable_info.fc_params_len, argument_variable_info.fc_target);
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
    }

    pub fn check_pointer_type(&mut self, entry_info: Entry, argument_variable_info: ArgumentVariableInfo) {
        let word = entry_info.word.clone();

        let rhs_ptr_type = self.ptr_type.clone();
        let mut mismatch: Option<(TokenType, Option<TokenType>, Option<PointerType>)> = None;
        match self.active_table().last_mut() {
            Some(TableTypes::Variable(var)) if !argument_variable_info.in_nested_call => {
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
            Some(TableTypes::Reasingment(reasign)) if !argument_variable_info.in_nested_call => {
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

    /// Check that a variable declaration's literal value matches the declared type.
    /// Called when a new `Variable` entry gets its first value word and already has a name.
    pub(super) fn check_variable_literal_type(&self, var_type: &TokenType, value_word: &Word) {
        let literal_token = TokenType::from_str(&value_word.word);
        let literal_declared = match literal_token.literal_type() {
            Some(t) => t,
            None => return, // not a literal — nothing to check here
        };

        if &literal_declared != var_type {
            report!(
                self.file_name.clone(),
                value_word.line.unwrap_or(0),
                value_word.char_num.unwrap_or(0),
                "Type mismatch. Expected: {}, Found: {}",
                var_type.to_str(),
                literal_declared.to_str(),
            );
        }
    }

    /// Check for variable redeclaration inside the currently active scope.
    pub(super) fn check_redeclared_variable(&self, name: &str, word: &Word) {
        let table = self.active_table_ref();
        // Check if another Variable with the same name already exists in this scope.
        let already_declared = table.iter().any(|entry| {
            matches!(entry, TableTypes::Variable(v) if v.name.as_deref() == Some(name))
        });
        if already_declared {
            report!(
                self.file_name.clone(),
                word.line.unwrap_or(0),
                word.char_num.unwrap_or(0),
                "Variable '{}' is already declared in this scope",
                name,
            );
        }
    }

    /// Check for a duplicate function name at root level.
    pub(super) fn check_duplicate_function(&self, name: &str, word: &Word) {
        let already_declared = self.table.iter().any(|entry| {
            matches!(entry, TableTypes::Function(f) if f.name.as_deref() == Some(name))
        });
        if already_declared {
            report!(
                self.file_name.clone(),
                word.line.unwrap_or(0),
                word.char_num.unwrap_or(0),
                "Function '{}' is already defined",
                name,
            );
        }
    }

    /// Post-analysis check: make sure a `main` function was defined.
    pub(super) fn check_main_exists(&self) {
        let has_main = self.table.iter().any(|entry| {
            matches!(entry, TableTypes::Function(f) if f.name.as_deref() == Some("main"))
        });
        if !has_main {
            report!(
                self.file_name.clone(),
                0,
                0,
                "No 'main' function found",
            );
        }
    }

    /// Check that a completed function call received at least as many arguments as required.
    /// Call this after the closing `)` of a call is processed.
    pub(super) fn check_too_few_arguments(
        &self,
        fc_name: &str,
        fc_scope: Scope,
        fc_target: usize,
        actual_len: usize,
        word: &Word,
    ) {
        use crate::enbeded_funcs::FUNCTIONS;
        let expected = if fc_scope == Scope::EnbedFunc {
            FUNCTIONS[fc_target].parameters.len()
        } else {
            let (table, is_method) = self.callee_table(fc_name);
            if let Some(TableTypes::Function(f)) = table.get(fc_target) {
                f.parameters.as_ref().map_or(0, |p| p.len()) + is_method as usize
            } else {
                return;
            }
        };

        if actual_len < expected {
            report!(
                self.file_name.clone(),
                word.line.unwrap_or(0),
                word.char_num.unwrap_or(0),
                "Too few arguments for function call. Expected: {}, Found: {}",
                expected,
                actual_len,
            );
        }
    }

    /// Check that a symbol referenced inside a `return` expression exists in the current
    /// function's scope (or at root level). Reports an error when the identifier is
    /// unknown — exactly the same rule as for identifiers in assignments.
    pub(super) fn check_return_expression_defined(&self, word: &Word, index: &Option<(usize, Scope, ResolveType)>) {
        if index.is_none() {
            let token = TokenType::from_str(&word.word);
            // Only flag plain identifiers (not operators, literals, …)
            if token == TokenType::Unknow
                && !word.word.starts_with("self[")
                && word.word != "self"
                && !self.modifiers.contains(&Modifiers::InForLoop)
            {
                // Also search struct method parameters — they are not found by the normal
                // resolve chain when we are inside a struct literal body.
                let found_in_struct_method = self.table.iter().find_map(|entry| {
                    if let TableTypes::StructLiteral(s) = entry {
                        s.functions.iter().find_map(|f| {
                            if let TableTypes::Function(func) = f {
                                func.parameters.as_ref().and_then(|params| {
                                    params.iter().find(|p| {
                                        matches!(p, TableTypes::Variable(v) if v.name.as_deref() == Some(word.word.as_str()))
                                    })
                                })
                            } else {
                                None
                            }
                        })
                    } else {
                        None
                    }
                });

                if found_in_struct_method.is_none() {
                    report!(
                        self.file_name.clone(),
                        word.line.unwrap_or(0),
                        word.char_num.unwrap_or(0),
                        "Undefined symbol: {}",
                        word.word,
                    );
                }
            }
        }
    }

    /// Check that `var::member` where `var` is not a struct-typed variable emits an error.
    pub(super) fn check_member_on_non_struct(&self, inst_name: &str, word: &Word) {
        // If inst_name resolves to a Variable whose type is NOT StructDef, it's an error.
        let resolved_type = self
            .table
            .iter()
            .chain(self.table.iter().flat_map(|t| {
                if let TableTypes::Function(f) = t { f.table.iter() } else { [].iter() }
            }))
            .find_map(|entry| match entry {
                TableTypes::Variable(v) if v.name.as_deref() == Some(inst_name) => Some(&v.token_type),
                _ => None,
            });

        if let Some(token_type) = resolved_type {
            if !matches!(token_type, TokenType::StructDef(_)) {
                report!(
                    self.file_name.clone(),
                    word.line.unwrap_or(0),
                    word.char_num.unwrap_or(0),
                    "Cannot call method '{}' on non-struct variable '{}' of type '{}'",
                    word.word,
                    inst_name,
                    token_type.to_str(),
                );
            }
        }
    }
}
