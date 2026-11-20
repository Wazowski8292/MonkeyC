use super::{Tac, ThreeAddressCodeGenerator, Type};
use crate::variable_types::Return;

impl ThreeAddressCodeGenerator {
    pub fn add_return(&mut self, returns: Return) {
        let value_type = self.current_return_type.clone().or_else(|| returns.value.as_ref().map(|v| v.token_type.clone()));
        let tokens = returns.value.and_then(|v| v.value).unwrap_or_default();

        if tokens.is_empty() {
            self.tac_table.push(Tac {
                tac_type: Type::Return,
                arguments: vec![],
                operator: None,
                result: None,
                value_type: self.current_return_type.clone(),
                is_ptr: false,
            });
            return;
        }

        let target = self.next_temp();
        self.build_expression_chain(tokens, target, Type::Return, value_type);
    }
}
