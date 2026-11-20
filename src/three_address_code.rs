mod conditional;
mod debug;
mod function;
mod function_calls;
mod in_line_assembly;
mod r#loop;
mod reasignment;
mod r#return;
mod r#struct;
pub mod tac;
mod variable;

pub use tac::*;

use crate::semantic_analyzer::{Scope, TableTypes, TokenType};
use crate::variable_types::{Conditional, Value};
use std::collections::HashMap;
use std::vec::Vec;

#[derive(Debug)]
pub struct ThreeAddressCodeGenerator {
    pub(crate) tac_table: Vec<Tac>,
    pub(crate) temp_count: usize,
    pub(crate) label_count: usize,
    pub(crate) memory_alloc: usize,
    pub(crate) current_return_type: Option<TokenType>,
    pub(crate) struct_fields: HashMap<String, Vec<(String, TokenType, Option<Value>)>>,
}

impl ThreeAddressCodeGenerator {
    pub fn new() -> Self {
        Self {
            tac_table: vec![],
            temp_count: 0,
            label_count: 0,
            memory_alloc: 0,
            current_return_type: None,
            struct_fields: HashMap::new(),
        }
    }

    pub fn generate(&mut self, type_table: Vec<TableTypes>) {
        for entry in type_table.iter() {
            if let TableTypes::StructLiteral(struct_literal) = entry {
                self.add_structs(struct_literal.clone());
            }
        }

        let mut i = 0;
        while i < type_table.len() {
            let entry = &type_table[i];
            match entry {
                TableTypes::Variable(var) => self.add_variable(var.clone()),
                TableTypes::Function(func) => self.add_function(func.clone()),
                TableTypes::FunctionCall(call) => self.add_function_call(call.clone()),
                TableTypes::Reasingment(reassignment) => self.add_reasingment(reassignment.clone()),
                TableTypes::Conditional(cond) => {
                    let mut chain: Vec<Conditional> = vec![cond.clone()];
                    let mut j = i + 1;
                    while j < type_table.len() {
                        if let TableTypes::Conditional(next_cond) = &type_table[j] {
                            if next_cond.token_type == TokenType::Else {
                                chain.push(next_cond.clone());
                                j += 1;
                                continue;
                            }
                        }
                        break;
                    }
                    self.add_conditional_chain(chain);
                    i = j;
                    continue;
                }
                TableTypes::Loop(lp) => self.add_loop(lp.clone()),
                TableTypes::Return(returns) => self.add_return(returns.clone()),
                TableTypes::InlineAssembly(asm) => self.add_inline_asm(asm.clone()),
                _ => {}
            }
            i += 1;
        }
    }

    pub(crate) fn extract_operand(entry: &TableTypes) -> String {
        if let TableTypes::Variable(var) = entry {
            if let Some(name) = &var.name {
                return name.clone();
            }
            if let Some(values) = &var.value {
                if let Some(first) = values.first() {
                    if let Value::Var(var) = first {
                        return var.clone();
                    }
                }
            }
        } else if let TableTypes::Reasingment(r) = entry {
            return r.name.clone();
        }
        String::new()
    }

    pub(crate) fn symbol_ref(target: usize, scope: &Scope) -> String {
        format!("{:?}#{}", scope, target)
    }

    pub(crate) fn next_label(&mut self) -> String {
        let label = format!("L{}", self.label_count);
        self.label_count += 1;
        label
    }

    pub(crate) fn next_temp(&mut self) -> String {
        let temp = format!("_t{}", self.temp_count);
        self.temp_count += 1;
        temp
    }

    pub(crate) fn precedence(op: &Operator) -> u8 {
        match op {
            Operator::LogicalOr => 1,
            Operator::LogicalAnd => 2,
            Operator::Or => 3,
            Operator::And => 4,
            Operator::LogicalEquals | Operator::NotEquals | Operator::GreaterThan | Operator::LessThan => 5,
            Operator::LeftBitShift | Operator::RightBitShift => 6,
            Operator::Plus | Operator::Minus => 7,
            Operator::Multiplication | Operator::Division => 8,
            _ => 0,
        }
    }

    pub(crate) fn evaluate_value(&mut self, value: &Value, value_type: Option<TokenType>) -> (String, bool) {
        match value {
            Value::Var(val) => (val.clone(), false),
            Value::FuncCall(fncall) => {
                let tmp = self.next_temp();
                self.add_function_call(fncall.clone());
                self.tac_table.push(Tac {
                    tac_type: Type::GetReturn,
                    arguments: vec![],
                    operator: None,
                    result: Some(tmp.clone()),
                    value_type,
                    is_ptr: false,
                });
                (tmp, false)
            }
            Value::Deref(name) => {
                let tmp = self.next_temp();
                self.tac_table.push(Tac {
                    tac_type: Type::Deref,
                    arguments: vec![name.clone()],
                    operator: None,
                    result: Some(tmp.clone()),
                    value_type,
                    is_ptr: true,
                });
                (tmp, true)
            }
            Value::Ref(name) => {
                let tmp = self.next_temp();
                self.tac_table.push(Tac {
                    tac_type: Type::AddressOf,
                    arguments: vec![name.clone()],
                    operator: None,
                    result: Some(tmp.clone()),
                    value_type,
                    is_ptr: true,
                });
                (tmp, true)
            }
            Value::Index(array_name, index_expr) => {
                let tmp = self.next_temp();
                self.tac_table.push(Tac {
                    tac_type: Type::ArrayIndex,
                    arguments: vec![array_name.clone(), index_expr.clone()],
                    operator: None,
                    result: Some(tmp.clone()),
                    value_type,
                    is_ptr: false,
                });
                (tmp, false)
            }
        }
    }

    pub(crate) fn parse_expr(
        &mut self, tokens: &[Value], pos: &mut usize, min_prec: u8, tac_type: Type, target: String, is_top: bool,
        value_type: Option<TokenType>,
    ) -> String {
        let (mut left, _) = self.evaluate_value(&tokens[*pos], value_type.clone());
        *pos += 1;

        loop {
            let op = match tokens.get(*pos) {
                Some(Value::Var(s)) => Operator::from_str(s),
                _ => break,
            };

            let prec = Self::precedence(&op);
            if prec < min_prec || *pos == tokens.len() {
                break;
            }
            *pos += 1;

            let right = self.parse_expr(tokens, pos, prec + 1, tac_type.clone(), target.clone(), false, value_type.clone());

            let next_continues = match tokens.get(*pos) {
                Some(Value::Var(s)) => Self::precedence(&Operator::from_str(s)) >= min_prec,
                _ => false,
            };

            let is_final = is_top && !next_continues;
            let result_name = if is_final { target.clone() } else { self.next_temp() };
            let this_type = if tac_type == Type::Return && !is_final { Type::Variable } else { tac_type.clone() };

            self.tac_table.push(Tac {
                tac_type: this_type,
                arguments: vec![left, right],
                operator: Some(op),
                result: Some(result_name.clone()),
                value_type: value_type.clone(),
                is_ptr: false,
            });
            left = result_name;
        }

        left
    }

    pub(crate) fn build_expression_chain(&mut self, tokens: Vec<Value>, target: String, tac_type: Type, value_type: Option<TokenType>) {
        if tokens.is_empty() {
            return;
        }

        if tokens.len() == 1 {
            match &tokens[0] {
                Value::Deref(name) => {
                    self.tac_table.push(Tac {
                        tac_type: Type::Deref,
                        arguments: vec![name.clone()],
                        operator: None,
                        result: Some(target),
                        value_type,
                        is_ptr: true,
                    });
                }
                Value::Ref(name) => {
                    self.tac_table.push(Tac {
                        tac_type: Type::AddressOf,
                        arguments: vec![name.clone()],
                        operator: None,
                        result: Some(target),
                        value_type,
                        is_ptr: true,
                    });
                }
                _ => {
                    let (operand, is_ptr) = self.evaluate_value(&tokens[0], value_type.clone());
                    self.tac_table.push(Tac {
                        tac_type,
                        arguments: vec![operand],
                        operator: None,
                        result: Some(target),
                        value_type,
                        is_ptr,
                    });
                }
            }
            return;
        }

        let mut pos = 0;
        let _ = self.parse_expr(&tokens, &mut pos, 0, tac_type.clone(), target, true, value_type);
    }
}

pub fn generate_three_address_code(type_table: Vec<TableTypes>, debug: bool) -> Vec<Tac> {
    let mut generator = ThreeAddressCodeGenerator::new();
    generator.generate(type_table);

    if debug {
        generator._print();
        println!("{:#?}", generator.tac_table);
    }

    generator.tac_table
}

