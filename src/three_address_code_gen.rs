use crate::variable_types::{Variable, Function, Reasingment, FunctionCall, Conditional, Loop, Value};
use crate::semantic_analyzer::{TableTypes, Scope};

use std::vec::Vec;

#[derive(Debug, Clone)]
pub enum Operator {
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

    pub fn to_asm_op(&self) -> Option<&'static str> {
        match self {
            Operator::Plus => Some("add"),
            Operator::Minus => Some("sub"),
            Operator::Multiplication => Some("imul"),
            Operator::RightBitShift => Some("sar"),
            Operator::LeftBitShift => Some("shl"),
            Operator::And => Some("and"),
            Operator::Or => Some("or"),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Function,
    Variable,
    Call,
    Reasingment,
    Conditional,
    ConditionalEnd,
    Loop,
    LoopEnd,
    Label,
    GetReturn,
}

#[derive(Debug, Clone)]
pub struct Tac {
    pub tac_type: Type,
    pub arguments: Vec<String>,
    pub operator: Option<Operator>,
    pub result: Option<String>,
}

struct ThreeAddressCodeGenerator {
    tac_table: Vec<Tac>,
    temp_count: usize,
    label_count: usize,
    memory_alloc: usize,
}

impl ThreeAddressCodeGenerator {
    pub fn new() -> Self {
        Self {
            tac_table: vec![], 
            temp_count: 0,
            label_count: 0,
            memory_alloc: 0,
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

    fn parse_expr(&mut self, tokens: &[Value], pos: &mut usize, min_prec: u8, tac_type: Type, target: String) -> String {
        let mut left = {
            match tokens[*pos].clone() {
                Value::Var(val) => val,
                Value::FuncCall(fncall) => {
                    let tmp = self.next_temp();

                    self.add_function_call(fncall);

                    self.tac_table.push(Tac {
                        tac_type: Type::GetReturn,
                        arguments: vec![],
                        operator: None,
                        result: Some(tmp),
                    });

                    tmp
                } 
            }
        };
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
 
            let right = self.parse_expr(tokens, pos, prec + 1, tac_type.clone(), target.clone());
 
            let result_name = if *pos == tokens.len() - 2{ self.next_temp() } else { target.clone() };
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

    fn build_expression_chain( &mut self, tokens: Vec<Value>, target: String, tac_type: Type) {
        if tokens.is_empty() {
            return;
        }
 
        if tokens.len() == 1 {
            let operand = tokens[0].clone();
            
            self.tac_table.push(Tac {
                tac_type,
                arguments: vec![operand],
                operator: None,
                result: Some(target),
            });

            return;
        }
 
        let mut pos = 0;
        let _ = self.parse_expr(&tokens, &mut pos, 0, tac_type.clone(), target);
    }

    fn get_return_value() {

    }
 
    fn add_function(&mut self, function: Function) {
        self.memory_alloc = 0;

        let tac = Tac {
            tac_type: Type::Function,
            result: None,
            arguments: vec![function.name.unwrap_or_default()],
            operator: None,
        };
 
        self.tac_table.push(tac);
        let function_def_index = self.tac_table.len() - 1;

        self.generate(function.table);

        self.tac_table[function_def_index].arguments.push(self.memory_alloc.to_string());
        
        for parameter in function.parameters.unwrap_or_default() {
            self.tac_table[function_def_index].arguments.push(Self::extract_operand(&parameter));
        }
    }
 
    fn add_variable(&mut self, variable: Variable) {
        let last_temp = self.temp_count;

        let name;
        if variable.name == Some("_".to_string()) {
            name = self.next_temp();
        } else {
            name = variable.name.unwrap_or_default();
        }

        let tokens = variable.value.unwrap_or_default();
        self.build_expression_chain(tokens, name, Type::Variable);

        self.memory_alloc += (self.temp_count - last_temp + 1) * 16; //TODO: this should depend on the variable and need to remember that before calling a function that it should be a multiple of 16
    }
 
    fn add_function_call(&mut self, call: FunctionCall) {
        //let target_ref = Self::symbol_ref(call.target, &call.target_scope);
        
        let mut tac = Tac {
            tac_type: Type::Call,
            result: None,
            arguments: vec![call.name],
            operator: None,
        };
 
        for parameter in call.parameters.unwrap_or_default() {
            tac.arguments.push(Self::extract_operand(&parameter));
        }
 
        self.tac_table.push(tac);
    }
 
    fn add_reasingment(&mut self, reassignment: Reasingment) {
        let target_ref = Self::symbol_ref(reassignment.target, &reassignment.target_scope);
        let tokens: Vec<Value> = reassignment.parameters.unwrap_or_default().iter().map(Self::extract_operand).collect();
 
        self.build_expression_chain(tokens, target_ref, Type::Reasingment);
        self.tac_table.last_mut().unwrap().result = Some(reassignment.name);
    }
 
    fn add_conditional(&mut self, conditional: Conditional) {
        self.add_conditional_block(Type::Conditional, Type::ConditionalEnd, conditional.condition, conditional.table);
    }
 
    fn add_loop(&mut self, loop_node: Loop) {

        self.add_conditional_block(Type::Loop, Type::LoopEnd, loop_node.condition, loop_node.table);
    }

    fn add_conditional_block(&mut self, start: Type, end: Type, condition: Vec<TableTypes>, table: Vec<TableTypes>) {
        let label = self.next_label();

        if start == Type::Loop {
            let tac_label = Tac {
                tac_type: Type::Label,
                result: None,
                arguments: vec![format!("{}_loop", label.clone())],
                operator: None,
            };

            self.tac_table.push(tac_label);
        }

        let mut tac = Tac {
            tac_type: start,
            result: None,
            arguments: vec![label.clone()],
            operator: None,
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
        });
    }

    fn attach_condition_info(&mut self, tac: &mut Tac) {
        if let Some(last) = self.tac_table.last() {
            let is_logical_combinator = matches!(
                last.operator,
                Some(Operator::LogicalAnd) | Some(Operator::LogicalOr)
            );

            if !is_logical_combinator {
                if let (Some(op), [left, right]) = (&last.operator, last.arguments.as_slice()) {
                    tac.operator = Some(op.clone());
                    tac.arguments.push(left.clone());
                    tac.arguments.push(right.clone());
                    return;
                }
            }

            tac.arguments.push(
                last.result.clone().unwrap_or_else(|| "0".to_string())
            );
        }
    }

    pub fn print(&self) {
        let mut indent: usize = 0;
 
        for tac in &self.tac_table {
            if matches!(tac.tac_type, Type::LoopEnd | Type::ConditionalEnd | Type::Function) {
                indent = indent.saturating_sub(1);
            }
 
            let pad = "    ".repeat(indent);
            println!("{}", Self::format_tac(tac, &pad));
 
            if matches!( tac.tac_type, Type::Loop | Type::Conditional ) {
                indent += 1;
            } else if matches!( tac.tac_type, Type::Function) {
                indent = 1;
            }
        }
    }
 
    fn format_tac(tac: &Tac, pad: &str) -> String {
        match &tac.tac_type {
            Type::Function => {
                let name = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                let params = tac.arguments.get(2..).unwrap_or(&[]).join(", ");
                format!("{pad}: function {name}({params})")
            }
            Type::LoopEnd => {
                let label = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                format!("{pad}{label}: end while")
            }
            Type::Conditional => {
                match (&tac.operator, tac.arguments.get(1..).unwrap_or(&[])) {
                    (Some(op), [left, right]) => {
                        format!("{pad} if ({left} {} {right})", op.as_str())
                    }
                    (_, rest) => format!("{pad} if ({})", rest.join(", ")),
                }
            }
            Type::Loop => {
                match (&tac.operator, tac.arguments.get(1..).unwrap_or(&[])) {
                    (Some(op), [left, right]) => {
                        format!("{pad} while ({left} {} {right})", op.as_str())
                    }
                    (_, rest) => format!("{pad} while ({})", rest.join(", ")),
                }
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
            Type::Label => {
                let label = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                format!("{pad}{label}:")
            }
            Type::GetReturn => {
                format!("{pad} get return value")
            }
        }
    }
}
 
pub fn generate_three_address_code(type_table: Vec<TableTypes>) -> Vec<Tac>{
    let mut generator = ThreeAddressCodeGenerator::new();
    generator.generate(type_table);
    generator.print();

    //println!("{:#?}", generator.tac_table);
    generator.tac_table
}
