use super::{SemanticAnalyzer, TableTypes, ResolveType, Entry, TokenType, Scope};
use crate::variable_types::{Reasingment, FunctionCall, PointerType};
use crate::report;

impl SemanticAnalyzer {
    pub fn handle_new_entry(&mut self, entry_info: Entry) {
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
}
