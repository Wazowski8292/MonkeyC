use super::{Tac, ThreeAddressCodeGenerator, Type};
use crate::variable_types::InlineAssembly;

impl ThreeAddressCodeGenerator {
    pub fn add_inline_asm(&mut self, inline_asm: InlineAssembly) {
        let asm_tac = Tac {
            tac_type: Type::InlineAssembly,
            arguments: inline_asm.table,
            operator: None,
            result: None,
            value_type: None,
            is_ptr: false,
        };

        let power_of_two_alloc_amount = (inline_asm.alloc_size.ilog2() + 1) as usize;

        self.temp_count += power_of_two_alloc_amount;
        self.tac_table.push(asm_tac);
    }
}
