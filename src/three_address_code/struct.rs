use super::ThreeAddressCodeGenerator;
use crate::semantic_analyzer::{TableTypes, TokenType};
use crate::variable_types::{PointerType, StructLiteral, Value, Variable};

impl ThreeAddressCodeGenerator {
    pub fn add_structs(&mut self, mut struct_literal: StructLiteral) {
        let fields: Vec<(String, TokenType, Option<Value>)> = struct_literal
            .arguments
            .iter()
            .filter_map(|arg| {
                if let TableTypes::Variable(v) = arg {
                    let name = v.name.clone()?;
                    let ty = v.token_type.clone();
                    let default = v.value.as_ref().and_then(|vals| vals.first().cloned());
                    Some((name, ty, default))
                } else {
                    None
                }
            })
            .collect();
        self.struct_fields.insert(struct_literal.name.clone(), fields);

        let struct_literal_name = struct_literal.name.clone();
        let struct_literal_length = struct_literal.arguments.len();

        for literal in struct_literal.functions.iter_mut() {
            if let TableTypes::Function(function) = literal {
                if let Some(name) = function.name.as_mut() {
                    *name = format!("{}.{}", struct_literal_name.clone(), name);
                }
                let variable = Variable {
                    token_type: TokenType::StructDef(struct_literal_name.clone()),
                    value: None,
                    name: Some("self".to_string()),
                    ptr: Some(PointerType::Reference),
                    is_array: true,
                    array_size: Some(struct_literal_length),
                    modifire: vec![],
                };
                function.parameters.get_or_insert_with(Vec::new).insert(0, TableTypes::Variable(variable));
            }
        }
        self.generate(struct_literal.functions);
    }
}
