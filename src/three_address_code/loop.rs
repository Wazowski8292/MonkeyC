use super::{ThreeAddressCodeGenerator, Type};
use crate::semantic_analyzer::{TableTypes, TokenType};
use crate::variable_types::Loop;

impl ThreeAddressCodeGenerator {
    pub fn add_loop(&mut self, loop_node: Loop) {
        let mut loop_node = loop_node;
        let is_for_loop = loop_node.token_type == TokenType::ForLoop;
        let mut conditional = loop_node.condition.clone();
        if is_for_loop {
            if let TableTypes::Variable(var) = conditional[0].clone() {
                self.add_variable(var.clone());
                conditional.remove(0);
                conditional.remove(conditional.len() - 1);

                loop_node.table.push(loop_node.condition.last().unwrap().clone());
            }
        }

        self.add_conditional_block(Type::Loop, Type::LoopEnd, conditional.clone(), loop_node.table);
    }
}
