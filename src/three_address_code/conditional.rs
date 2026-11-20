use super::{Tac, ThreeAddressCodeGenerator, Type};
use crate::semantic_analyzer::{TableTypes, TokenType};
use crate::variable_types::Conditional;

impl ThreeAddressCodeGenerator {
    pub fn add_conditional(&mut self, conditional: Conditional) {
        self.add_conditional_block(Type::Conditional, Type::ConditionalEnd, conditional.condition, conditional.table);
    }

    fn is_plain_else(cond: &Conditional) -> bool {
        match cond.condition.first() {
            Some(TableTypes::Variable(v)) => {
                matches!(v.token_type, TokenType::BoolLiteral)
                    && v.name.as_deref() == Some("_")
                    && v.value.as_ref().map_or(true, |vals| vals.is_empty())
            }
            _ => false,
        }
    }

    pub(crate) fn add_conditional_chain(&mut self, chain: Vec<Conditional>) {
        if chain.is_empty() {
            return;
        }

        if chain.len() == 1 {
            self.add_conditional(chain.into_iter().next().unwrap());
            return;
        }

        let done_label = self.next_label();
        let mut branch_labels: Vec<String> = (0..chain.len() - 1).map(|_| self.next_label()).collect();
        branch_labels.push(done_label.clone());

        for (idx, conditional) in chain.into_iter().enumerate() {
            let is_last = idx == branch_labels.len() - 1;
            let my_false_label = branch_labels[idx].clone();

            if Self::is_plain_else(&conditional) {
                self.generate(conditional.table);
            } else {
                let mut tac = Tac {
                    tac_type: Type::Conditional,
                    result: None,
                    arguments: vec![my_false_label.clone()],
                    operator: None,
                    value_type: None,
                    is_ptr: false,
                };

                if let Some(TableTypes::Variable(var)) = conditional.condition.first() {
                    self.add_variable(var.clone());
                    self.attach_condition_info(&mut tac);
                } else if let Some(TableTypes::Reasingment(re)) = conditional.condition.first() {
                    self.add_reasingment(re.clone());
                    self.attach_condition_info(&mut tac);
                }

                self.tac_table.push(tac);
                self.generate(conditional.table);
            }

            if !is_last {
                self.tac_table.push(Tac {
                    tac_type: Type::Jump,
                    arguments: vec![done_label.clone()],
                    operator: None,
                    result: None,
                    value_type: None,
                    is_ptr: false,
                });
            }

            self.tac_table.push(Tac {
                tac_type: Type::ConditionalEnd,
                arguments: vec![my_false_label],
                operator: None,
                result: None,
                value_type: None,
                is_ptr: false,
            });
        }
    }

    pub(crate) fn add_conditional_block(&mut self, start: Type, end: Type, condition: Vec<TableTypes>, table: Vec<TableTypes>) {
        let label = self.next_label();

        if start == Type::Loop {
            let tac_label = Tac {
                tac_type: Type::Label,
                result: None,
                arguments: vec![format!("{}_loop", label.clone())],
                operator: None,
                value_type: None,
                is_ptr: false,
            };

            self.tac_table.push(tac_label);
        }

        let mut tac = Tac {
            tac_type: start,
            result: None,
            arguments: vec![label.clone()],
            operator: None,
            value_type: None,
            is_ptr: false,
        };

        if let Some(TableTypes::Variable(var)) = condition.first() {
            self.add_variable(var.clone());
            self.attach_condition_info(&mut tac);
        } else if let Some(TableTypes::Reasingment(re)) = condition.first() {
            self.add_reasingment(re.clone());
            self.attach_condition_info(&mut tac);
        }

        self.tac_table.push(tac);

        self.generate(table);

        self.tac_table.push(Tac {
            tac_type: end,
            arguments: vec![label],
            operator: None,
            result: None,
            value_type: None,
            is_ptr: false,
        });
    }

    fn attach_condition_info(&mut self, tac: &mut Tac) {
        if let Some(last) = self.tac_table.last() {
            tac.arguments.push(last.result.clone().unwrap_or_else(|| "0".to_string()));
        }
    }
}
