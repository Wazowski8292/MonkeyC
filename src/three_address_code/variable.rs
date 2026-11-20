use super::{Tac, ThreeAddressCodeGenerator, Type};
use crate::semantic_analyzer::{TableTypes, TokenType};
use crate::variable_types::{PointerType, Value, Variable};

impl ThreeAddressCodeGenerator {
    pub fn add_variable(&mut self, variable: Variable) {
        let last_temp = self.temp_count;

        let name = { if variable.name == Some("_".to_string()) { self.next_temp() } else { variable.name.clone().unwrap_or_default() } };

        let value_type = Some(variable.clone().token_type);

        if variable.is_array {
            self.add_array_type(variable.clone(), name, value_type);
        } else if let TokenType::StructDef(ref struct_name) = variable.token_type {
            self.add_struct_type(variable.clone(), struct_name, name);
        } else {
            let tokens = variable.value.unwrap_or_default();
            let tac_type = Type::Variable;

            self.build_expression_chain(tokens, name, tac_type, value_type);
        }

        self.memory_alloc += (self.temp_count - last_temp + 1) * 16; //TODO: this should depend on the variable and need to remember that before calling a function that it should be a multiple of 16
    }

    fn add_array_type(&mut self, variable: Variable, name: String, value_type: Option<TokenType>) {
        let size = variable.array_size.unwrap_or(1);
        self.tac_table.push(Tac {
            tac_type: Type::ArrayDecl,
            arguments: vec![size.to_string()],
            operator: None,
            result: Some(name.clone()),
            value_type: value_type.clone(),
            is_ptr: false,
        });

        self.memory_alloc += size * 16;

        if let Some(tokens) = variable.value {
            for (idx, token) in tokens.iter().enumerate() {
                let (val_str, _) = self.evaluate_value(token, value_type.clone());
                self.tac_table.push(Tac {
                    tac_type: Type::ArrayAssign,
                    arguments: vec![idx.to_string(), val_str],
                    operator: None,
                    result: Some(name.clone()),
                    value_type: value_type.clone(),
                    is_ptr: false,
                });
            }
        }
    }

    fn add_struct_type(&mut self, variable: Variable, struct_name: &str, name: String) {
        let fields = self.struct_fields.get(struct_name).cloned().unwrap_or_default();
        let size = fields.len(); // TODO: Make variables have diferent size

        self.memory_alloc += size * 16;

        self.tac_table.push(Tac {
            tac_type: Type::ArrayDecl,
            arguments: vec![size.to_string()],
            operator: None,
            result: Some(name.clone()),
            value_type: Some(variable.token_type.clone()),

            is_ptr: false,
        });

        for (idx, (_, field_type, field_default)) in fields.into_iter().enumerate() {
            if let Some(value) = field_default {
                let (val_str, _) = self.evaluate_value(&value, Some(field_type.clone()));
                self.tac_table.push(Tac {
                    tac_type: Type::ArrayAssign,
                    arguments: vec![idx.to_string(), val_str],
                    operator: None,
                    result: Some(name.clone()),
                    value_type: Some(field_type.clone()),
                    is_ptr: false,
                });
            }
        }
    }

    pub(crate) fn table_type_to_value(e: &TableTypes) -> Value {
        match e {
            TableTypes::FunctionCall(call) => Value::FuncCall(call.clone()),
            TableTypes::Reasingment(r) if r.ptr == Some(PointerType::Pointer) => Value::Deref(r.name.clone()),
            TableTypes::Reasingment(r) if r.ptr == Some(PointerType::Reference) && r.name != "self" => Value::Ref(r.name.clone()),
            TableTypes::Reasingment(r) => {
                if let Some(idx) = &r.array_index {
                    Value::Index(r.name.clone(), idx.clone())
                } else {
                    Value::Var(r.name.clone())
                }
            }
            TableTypes::Variable(v) => {
                if let Some(val) = v.value.as_ref().and_then(|v| v.first()) {
                    val.clone()
                } else if let Some(name) = &v.name {
                    if let Some((arr_name, idx_str)) = crate::variable_types::parse_array_syntax(name) {
                        Value::Index(arr_name, idx_str)
                    } else {
                        Value::Var(name.clone())
                    }
                } else {
                    Value::Var(String::new())
                }
            }
            _ => {
                let op = Self::extract_operand(e);
                if let Some((arr_name, idx_str)) = crate::variable_types::parse_array_syntax(&op) {
                    Value::Index(arr_name, idx_str)
                } else {
                    Value::Var(op)
                }
            }
        }
    }
}
