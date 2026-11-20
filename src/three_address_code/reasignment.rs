use super::{Tac, ThreeAddressCodeGenerator, Type};
use crate::semantic_analyzer::TokenType;
use crate::variable_types::{PointerType, Reasingment, Value};

impl ThreeAddressCodeGenerator {
    pub fn add_reasingment(&mut self, reassignment: Reasingment) {
        let target_ref = Self::symbol_ref(reassignment.target, &reassignment.target_scope);
        let value_type = Some(reassignment.clone().token_type);

        let raw_tokens: Vec<Value> =
            reassignment.clone().parameters.unwrap_or_default().iter().map(|e| Self::table_type_to_value(e)).collect();

        let left_op = if let Some(ref idx_str) = reassignment.array_index {
            Value::Index(reassignment.name.clone(), idx_str.clone())
        } else {
            Value::Var(reassignment.name.clone())
        };

        let tokens = if let Some(Value::Var(op_str)) = raw_tokens.first() {
            match op_str.as_str() {
                "+=" => {
                    let mut desugared = vec![left_op, Value::Var("+".to_string())];
                    desugared.extend(raw_tokens.into_iter().skip(1));
                    desugared
                }
                "-=" => {
                    let mut desugared = vec![left_op, Value::Var("-".to_string())];
                    desugared.extend(raw_tokens.into_iter().skip(1));
                    desugared
                }
                "++" => {
                    vec![left_op, Value::Var("+".to_string()), Value::Var("1".to_string())]
                }
                "--" => {
                    vec![left_op, Value::Var("-".to_string()), Value::Var("1".to_string())]
                }
                _ => raw_tokens,
            }
        } else {
            raw_tokens
        };

        if let Some(ref array_idx) = reassignment.array_index {
            self.add_reasing_array(reassignment.clone(), array_idx.to_string(), tokens.clone(), value_type.clone());
            return;
        }

        if reassignment.ptr == Some(PointerType::Pointer) {
            self.add_reasing_ptr(reassignment.clone(), tokens.clone());
            return;
        }

        self.build_expression_chain(tokens, target_ref, Type::Reasingment, value_type);
        self.tac_table.last_mut().unwrap().result = Some(reassignment.name);
    }

    fn add_reasing_array(&mut self, reassignment: Reasingment, array_idx: String, tokens: Vec<Value>, value_type: Option<TokenType>) {
        let rhs_val = if tokens.len() == 1 {
            let (val, _) = self.evaluate_value(&tokens[0], value_type.clone());
            val
        } else {
            let tmp = self.next_temp();
            self.build_expression_chain(tokens, tmp.clone(), Type::Variable, value_type.clone());
            tmp
        };

        self.tac_table.push(Tac {
            tac_type: Type::ArrayAssign,
            arguments: vec![array_idx, rhs_val],
            operator: None,
            result: Some(reassignment.name),
            value_type,
            is_ptr: false,
        });
    }

    fn add_reasing_ptr(&mut self, reassignment: Reasingment, tokens: Vec<Value>) {
        let value_type = Some(reassignment.token_type);
        if tokens.len() == 1 {
            let rhs_val = match &tokens[0] {
                Value::Var(s) => s.clone(),
                Value::FuncCall(f) => {
                    let tmp = self.next_temp();
                    self.add_function_call(f.clone());
                    self.tac_table.push(Tac {
                        tac_type: Type::GetReturn,
                        arguments: vec![],
                        operator: None,
                        result: Some(tmp.clone()),
                        value_type: value_type.clone(),
                        is_ptr: false,
                    });
                    tmp
                }
                Value::Deref(_) | Value::Ref(_) | Value::Index(_, _) => {
                    let tmp = self.next_temp();
                    self.build_expression_chain(tokens, tmp.clone(), Type::Variable, value_type.clone());
                    tmp
                }
            };

            self.tac_table.push(Tac {
                tac_type: Type::DerefAssign,
                arguments: vec![reassignment.name.clone(), rhs_val],
                operator: None,
                result: Some(reassignment.name),
                value_type: value_type.clone(),
                is_ptr: false,
            });
        } else {
            let tmp = self.next_temp();
            self.build_expression_chain(tokens, tmp.clone(), Type::Variable, value_type.clone());
            self.tac_table.push(Tac {
                tac_type: Type::DerefAssign,
                arguments: vec![reassignment.name.clone(), tmp],
                operator: None,
                result: Some(reassignment.name),
                value_type: value_type.clone(),
                is_ptr: false,
            });
        }
    }
}
