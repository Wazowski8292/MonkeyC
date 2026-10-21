use super::{Entry, ResolveType, Scope, SemanticAnalyzer, TableTypes, TokenType};
use crate::parser::Word;
use crate::report;
use crate::variable_types::PointerType;

impl SemanticAnalyzer {
    pub(super) fn check_parameters(&mut self, entry_info: Entry, fc_params_len: usize, fc_target: usize) {
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
}
