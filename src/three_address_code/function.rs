use super::{Tac, ThreeAddressCodeGenerator, Type};
use crate::semantic_analyzer::TableTypes;
use crate::variable_types::Function;

impl ThreeAddressCodeGenerator {
    pub fn add_function(&mut self, function: Function) {
        self.memory_alloc = 0;
        self.current_return_type = function.return_type;

        let tac = Tac {
            tac_type: Type::Function,
            result: None,
            arguments: vec![function.name.unwrap_or_default()],
            operator: None,
            value_type: None,
            is_ptr: false,
        };

        self.tac_table.push(tac);
        let function_def_index = self.tac_table.len() - 1;

        let params = function.parameters.unwrap_or_default();
        self.memory_alloc += params.len() * 16;

        for parameter in params {
            let (param_name, param_type, is_ptr) = match &parameter {
                TableTypes::Variable(var) => (var.name.clone().unwrap_or_default(), Some(var.token_type.clone()), var.ptr.is_some()),
                _ => (Self::extract_operand(&parameter), None, false),
            };
            self.tac_table.push(Tac {
                tac_type: Type::Param,
                arguments: vec![param_name],
                operator: None,
                result: None,
                value_type: param_type,
                is_ptr,
            });
        }

        self.generate(function.table);

        self.tac_table[function_def_index].arguments.push(self.memory_alloc.to_string());
    }
}
