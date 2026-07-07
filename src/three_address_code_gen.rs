use crate::variable_types::{Variable, Function, Reasingment, FunctionCall, Conditional, Loop};
use crate::semantic_analyzer::{TableTypes, Scope};

use std::vec::Vec;

#[derive(Debug, Clone)]
enum Operator {
    Plus,
    Minus,
    Multiplication,
    Division,
    Equals,

    LogicalEquals,
    LogicalAnd,
    LogicalOr,
    Not,

    RightBitShift,
    LeftBitShift,
    And,
    Or,

    Unknow,
}

impl Operator {
    pub fn from_str(operator: &str) -> Operator{
        match operator {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            "*" => Operator::Multiplication,
            "/" => Operator::Division,
            "=" => Operator::Equals,

            "==" => Operator::LogicalEquals,
            "&&" => Operator::LogicalAnd,
            "||" => Operator::LogicalOr,
            "!" => Operator::Not,

            ">>" => Operator::RightBitShift,
            "<<" => Operator::LeftBitShift,
            "&" => Operator::And,
            "|" => Operator::Or,

            _ => Operator::Unknow
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Multiplication => "*",
            Operator::Division => "/",
            Operator::Equals => "=",
            Operator::LogicalEquals => "==",
            Operator::LogicalAnd => "&&",
            Operator::LogicalOr => "||",
            Operator::Not => "!",
            Operator::RightBitShift => ">>",
            Operator::LeftBitShift => "<<",
            Operator::And => "&",
            Operator::Or => "|",
            Operator::Unknow => "?",
        }
    }
}

#[derive(Debug, Clone)]
enum Type {
    Function,
    FunctionEnd,
    Variable,
    Call,
    Reasingment,
    Conditional,
    ConditionalEnd,
    Loop,
    LoopEnd,
}

#[derive(Debug, Clone)]
struct Tac {
    tac_type: Type,
    arguments: Vec<String>,
    operator: Option<Operator>,
    result: Option<String>,
}

struct ThreeAddressCodeGenerator {
    tac_table: Vec<Tac>,
    temp_count: usize,
    label_count: usize,
}

impl ThreeAddressCodeGenerator {
    pub fn new() -> Self {
        Self {
            tac_table: vec![], 
            temp_count: 0,
            label_count: 0,
        }
    }

    pub fn generate(&mut self, type_table: Vec<TableTypes>) {
        for entry in type_table.iter() {
            match entry {
                TableTypes::Variable(var) => self.add_variable(var.clone()),
                TableTypes::Function(func) => self.add_function(func.clone()),
                TableTypes::FunctionCall(call) => self.add_function_call(call.clone()),
                TableTypes::Reasingment(reassignment) => self.add_reasingment(reassignment.clone()),
                TableTypes::Conditional(cond) => self.add_conditional(cond.clone()),
                TableTypes::Loop(lp) => self.add_loop(lp.clone()),
                _ => {}
            }
        }
    }

    fn extract_operand(entry: &TableTypes) -> String {
        if let TableTypes::Variable(var) = entry {
            if let Some(name) = &var.name {
                return name.clone();
            }
            if let Some(values) = &var.value {
                if let Some(first) = values.first() {
                    return first.clone();
                }
            }
        }
        String::new()
    }

    fn symbol_ref(target: usize, scope: &Scope) -> String {
        format!("{:?}#{}", scope, target)
    }
    
    fn next_label(&mut self) -> String {
        let label = format!("L{}", self.label_count);
        self.label_count += 1;
        label
    }

    fn next_temp(&mut self) -> String {
        let temp = format!("_t{}", self.temp_count);
        self.temp_count += 1;
        temp
    }

    fn precedence(op: &Operator) -> u8 {
        match op {
            Operator::LogicalOr => 1,
            Operator::LogicalAnd => 2,
            Operator::Or => 3,
            Operator::And => 4,
            Operator::LogicalEquals => 5,
            Operator::LeftBitShift | Operator::RightBitShift => 6,
            Operator::Plus | Operator::Minus => 7,
            Operator::Multiplication | Operator::Division => 8,
            _ => 0,
        }
    }

    fn parse_expr(&mut self, tokens: &[String], pos: &mut usize, min_prec: u8, tac_type: Type) -> String {
        let mut left = tokens[*pos].clone();
        *pos += 1;
 
        loop {
            let op = match tokens.get(*pos) {
                Some(s) => Operator::from_str(s),
                None => break,
            };
 
            let prec = Self::precedence(&op);
            if prec < min_prec {
                break;
            }
            *pos += 1;
 
            if *pos >= tokens.len() {
                break;
            }
 
            let right = self.parse_expr(tokens, pos, prec + 1, tac_type.clone());
 
            let result_name = self.next_temp();
            self.tac_table.push(Tac {
                tac_type: tac_type.clone(),
                arguments: vec![left, right],
                operator: Some(op),
                result: Some(result_name.clone()),
            });
            left = result_name;
        }
 
        left
    }

    fn build_expression_chain( &mut self, tokens: Vec<String>, target: Option<String>, tac_type: Type) -> Option<String> {
        if tokens.is_empty() {
            return None;
        }
 
        if tokens.len() == 1 {
            let operand = tokens[0].clone();
            return match target {
                Some(result_name) => {
                    self.tac_table.push(Tac {
                        tac_type,
                        arguments: vec![operand],
                        operator: None,
                        result: Some(result_name.clone()),
                    });
                    Some(result_name)
                }
                None => Some(operand),
            };
        }
 
        let mut pos = 0;
        let final_result = self.parse_expr(&tokens, &mut pos, 0, tac_type.clone());
 
        match target {
            Some(target_name) => {
                if let Some(last) = self.tac_table.last_mut() {
                    if last.result.as_deref() == Some(final_result.as_str()) {
                        last.result = Some(target_name.clone());
                        return Some(target_name);
                    }
                }
                self.tac_table.push(Tac {
                    tac_type,
                    arguments: vec![final_result],
                    operator: None,
                    result: Some(target_name.clone()),
                });
                Some(target_name)
            }
            None => Some(final_result),
        }
    }
 
    fn add_function(&mut self, function: Function) {
        let label = self.next_label();

        let mut tac = Tac {
            tac_type: Type::Function,
            result: None,
            arguments: vec![label.clone(), function.name.unwrap_or_default()],
            operator: None,
        };
 
        for parameter in function.parameters.unwrap_or_default() {
            tac.arguments.push(Self::extract_operand(&parameter));
        }
 
        self.tac_table.push(tac);
 
        self.generate(function.table);

        self.tac_table.push(Tac {
            tac_type: Type::FunctionEnd,
            arguments: vec![label],
            operator: None,
            result: None,
        });
    }
 
    fn add_variable(&mut self, variable: Variable) {
        let name;
        if variable.name == Some("_".to_string()) {
            name = self.next_temp();
        } else {
            name = variable.name.unwrap_or_default();
        }

        let tokens = variable.value.unwrap_or_default();
        self.build_expression_chain(tokens, Some(name), Type::Variable);
    }
 
    fn add_function_call(&mut self, call: FunctionCall) {
        let target_ref = Self::symbol_ref(call.target, &call.target_scope);
 
        let mut tac = Tac {
            tac_type: Type::Call,
            result: None,
            arguments: vec![target_ref],
            operator: None,
        };
 
        for parameter in call.parameters.unwrap_or_default() {
            tac.arguments.push(Self::extract_operand(&parameter));
        }
 
        self.tac_table.push(tac);
    }
 
    fn add_reasingment(&mut self, reassignment: Reasingment) {
        let target_ref = Self::symbol_ref(reassignment.target, &reassignment.target_scope);
        let tokens: Vec<String> = reassignment
            .parameters
            .unwrap_or_default()
            .iter()
            .map(Self::extract_operand)
            .collect();
 
        self.build_expression_chain(tokens, Some(target_ref), Type::Reasingment);
    }
 
    fn add_conditional(&mut self, conditional: Conditional) {
        self.add_conditional_block(Type::Conditional, Type::ConditionalEnd, conditional.condition, conditional.table);
    }
 
    fn add_loop(&mut self, loop_node: Loop) {
        self.add_conditional_block(Type::Loop, Type::LoopEnd, loop_node.condition, loop_node.table);
    }

    fn add_conditional_block(&mut self, start: Type, end: Type, condition: Vec<TableTypes>, table: Vec<TableTypes>) {
        let label = self.next_label();

        let mut tac = Tac {
            tac_type: start,
            result: None,
            arguments: vec![label.clone()],
            operator: None,
        };

        if let Some(TableTypes::Variable(var)) = condition.first() {
            self.add_variable(var.clone());
            
            tac.arguments.push(self.tac_table.last().unwrap().result.clone().unwrap_or("0".to_string()));
        }

        self.tac_table.push(tac);
 
        self.generate(table);

        self.tac_table.push(Tac {
            tac_type: end,
            arguments: vec![label],
            operator: None,
            result: None,
        });
    }

    pub fn print(&self) {
        let mut indent: usize = 0;
 
        for tac in &self.tac_table {
            if matches!(tac.tac_type, Type::FunctionEnd | Type::LoopEnd | Type::ConditionalEnd ) {
                indent = indent.saturating_sub(1);
            }
 
            let pad = "    ".repeat(indent);
            println!("{}", Self::format_tac(tac, &pad));
 
            if matches!( tac.tac_type, Type::Function | Type::Loop | Type::Conditional ) {
                indent += 1;
            }
        }
    }
 
    fn format_tac(tac: &Tac, pad: &str) -> String {
        match &tac.tac_type {
            Type::Function => {
                let label = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                let name = tac.arguments.get(1).map(String::as_str).unwrap_or("?");
                let params = tac.arguments.get(2..).unwrap_or(&[]).join(", ");
                format!("{pad}{label}: function {name}({params})")
            }
            Type::FunctionEnd => {
                let label = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                format!("{pad}{label}: end function")
            }
            Type::Loop => {
                let label = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                let cond = tac.arguments.get(1).map(String::as_str).unwrap_or("?");
                format!("{pad}{label}: while ({cond})")
            }
            Type::LoopEnd => {
                let label = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                format!("{pad}{label}: end while")
            }
            Type::Conditional => {
                let label = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                //let cond = tac.arguments.get(1).map(String::as_str).unwrap_or("?");
                let cond = tac.arguments.get(1..).unwrap_or(&[]).join(", ");

                format!("{pad}{label}: if ({cond})")
            }
            Type::ConditionalEnd => {
                let label = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                format!("{pad}{label}: end if")
            }
            Type::Call => {
                let target = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                let args = tac.arguments.get(1..).unwrap_or(&[]).join(", ");
                format!("{pad}call {target}({args})")
            }
            Type::Variable | Type::Reasingment => {
                let result = tac.result.as_deref().unwrap_or("?");
                match (&tac.operator, tac.arguments.as_slice()) {
                    (Some(op), [left, right]) => {
                        format!("{pad}{result} = {left} {} {right}", op.as_str())
                    }
                    (None, [value]) => format!("{pad}{result} = {value}"),
                    _ => format!("{pad}{result} = {:?}", tac.arguments),
                }
            }
        }
    }
}
 
pub fn generate_three_address_code(type_table: Vec<TableTypes>) {
    let mut generator = ThreeAddressCodeGenerator::new();
    generator.generate(type_table);
    generator.print();
    //println!("{:#?}", generator.tac_table);
}
