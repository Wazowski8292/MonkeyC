use super::{Tac, ThreeAddressCodeGenerator, Type};
use crate::semantic_analyzer::TableTypes;
use crate::variable_types::FunctionCall;

impl ThreeAddressCodeGenerator {
    pub fn add_function_call(&mut self, call: FunctionCall) {
        let mut tac = Tac {
            tac_type: Type::Call,
            result: None,
            arguments: vec![call.name],
            operator: None,
            value_type: None,
            is_ptr: false,
        };

        for parameter in call.parameters.unwrap_or_default() {
            let value = Self::table_type_to_value(&parameter);

            let value_type = match &parameter {
                TableTypes::Variable(var) => Some(var.token_type.clone()),
                TableTypes::Reasingment(reassignment) => Some(reassignment.token_type.clone()),
                _ => None,
            };

            let (arg, _is_ptr) = self.evaluate_value(&value, value_type);
            tac.arguments.push(arg);
        }

        self.tac_table.push(tac);
    }
}
