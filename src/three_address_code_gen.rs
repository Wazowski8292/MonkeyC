use crate::variable_types::{Variable, Function, Reasingment, FunctionCall, Conditional, Loop, Return, Value, PointerType};
use crate::semantic_analyzer::{TableTypes, Scope, TokenType};
use std::vec::Vec;

#[derive(Debug, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Multiplication,
    Division,
    Equals,

    LogicalEquals,
    NotEquals,
    GreaterThan,
    LessThan,
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
            "!=" => Operator::NotEquals,
            ">" => Operator::GreaterThan,
            "<" => Operator::LessThan,
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

    fn _as_str(&self) -> &'static str {
        match self {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Multiplication => "*",
            Operator::Division => "/",
            Operator::Equals => "=",
            Operator::LogicalEquals => "==",
            Operator::NotEquals => "!=",
            Operator::GreaterThan => ">",
            Operator::LessThan => "<",
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

    pub fn to_asm_op_f32(&self) -> Option<&'static str> {
        match self {
            Operator::Plus => Some("addss"),
            Operator::Minus => Some("subss"),
            Operator::Multiplication => Some("mulss"),
            Operator::Division => Some("divss"),
            _ => None,
        }
    }

    pub fn to_asm_op_f64(&self) -> Option<&'static str> {
        match self {
            Operator::Plus => Some("addsd"),
            Operator::Minus => Some("subsd"),
            Operator::Multiplication => Some("mulsd"),
            Operator::Division => Some("divsd"),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Function,
    Param,
    Variable,
    ArrayDecl,
    ArrayIndex,
    ArrayAssign,
    AddressOf,
    Deref,
    DerefAssign,
    Call,
    Reasingment,
    Conditional,
    ConditionalEnd,
    Loop,
    LoopEnd,
    Label,
    GetReturn,
    Return,
}

#[derive(Debug, Clone)]
pub struct Tac {
    pub tac_type: Type,
    pub arguments: Vec<String>,
    pub operator: Option<Operator>,
    pub result: Option<String>,
    pub value_type: Option<TokenType>,
    pub is_ptr: bool,
}

struct ThreeAddressCodeGenerator {
    tac_table: Vec<Tac>,
    temp_count: usize,
    label_count: usize,
    memory_alloc: usize,
    current_return_type: Option<TokenType>,
}

impl ThreeAddressCodeGenerator {
    pub fn new() -> Self {
        Self {
            tac_table: vec![], 
            temp_count: 0,
            label_count: 0,
            memory_alloc: 0,
            current_return_type: None,
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
                TableTypes::Return(returns) => self.add_return(returns.clone()),
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
            Operator::LogicalEquals | Operator::NotEquals | Operator::GreaterThan | Operator::LessThan => 5,
            Operator::LeftBitShift | Operator::RightBitShift => 6,
            Operator::Plus | Operator::Minus => 7,
            Operator::Multiplication | Operator::Division => 8,
            _ => 0,
        }
    }

    fn evaluate_value(&mut self, value: &Value, value_type: Option<TokenType>) -> (String, bool) {
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

    fn parse_expr(&mut self, tokens: &[Value], pos: &mut usize, min_prec: u8, tac_type: Type, target: String, is_top: bool, value_type: Option<TokenType>) -> String {
        let (mut left, _) = self.evaluate_value(&tokens[*pos], value_type.clone());
        *pos += 1;
 
        loop {
            let op = match tokens.get(*pos) {
                Some(Value::Var(s)) => Operator::from_str(s),
                _ => break,
            };
 
            let prec = Self::precedence(&op);
            if prec < min_prec  || *pos  == tokens.len(){
                break;
            }
            *pos += 1;
 
            let right = self.parse_expr(tokens, pos, prec + 1, tac_type.clone(), target.clone(), false, value_type.clone());
 
            let next_continues = match tokens.get(*pos) {
                Some(Value::Var(s)) => Self::precedence(&Operator::from_str(s)) >= min_prec,
                _ => false,
            };

            let result_name = if is_top && !next_continues {
                target.clone()
            } else {
                self.next_temp()
            };

            self.tac_table.push(Tac {
                tac_type: tac_type.clone(),
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

    fn build_expression_chain(&mut self, tokens: Vec<Value>, target: String, tac_type: Type, value_type: Option<TokenType>) {
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

    fn add_function(&mut self, function: Function) {
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
                TableTypes::Variable(var) => (
                    var.name.clone().unwrap_or_default(),
                    Some(var.token_type),
                    var.ptr.is_some(),
                ),
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
 
    fn add_variable(&mut self, variable: Variable) {
        let last_temp = self.temp_count;

        let name;
        if variable.name == Some("_".to_string()) {
            name = self.next_temp();
        } else {
            name = variable.name.clone().unwrap_or_default();
        }

        let value_type = Some(variable.token_type);

        if variable.is_array {
            let size = variable.array_size.unwrap_or(1);
            self.tac_table.push(Tac {
                tac_type: Type::ArrayDecl,
                arguments: vec![size.to_string()],
                operator: None,
                result: Some(name.clone()),
                value_type: value_type.clone(),
                is_ptr: false,
            });

            self.memory_alloc += size * 16;

            if let Some(tokens) = variable.value {
                for (idx, token) in tokens.iter().enumerate() {
                    let (val_str, _) = self.evaluate_value(token, value_type.clone());
                    self.tac_table.push(Tac {
                        tac_type: Type::ArrayAssign,
                        arguments: vec![idx.to_string(), val_str],
                        operator: None,
                        result: Some(name.clone()),
                        value_type: value_type.clone(),
                        is_ptr: false,
                    });
                }
            }
            return;
        }

        let tokens = variable.value.unwrap_or_default();
        let tac_type = Type::Variable;

        self.build_expression_chain(tokens, name, tac_type, value_type);

        self.memory_alloc += (self.temp_count - last_temp + 1) * 16; //TODO: this should depend on the variable and need to remember that before calling a function that it should be a multiple of 16
    }
 
    fn extract_call_arg(entry: &TableTypes) -> String {
        match entry {
            TableTypes::Reasingment(r) => {
                let prefix = match &r.ptr {
                    Some(PointerType::Reference) => "&",
                    Some(PointerType::Pointer)   => "*",
                    None => "",
                };
                format!("{}{}", prefix, r.name)
            }
            _ => Self::extract_operand(entry),
        }
    }

    fn table_type_to_value(e: &TableTypes) -> Value {
        match e {
            TableTypes::FunctionCall(call) => Value::FuncCall(call.clone()),
            TableTypes::Reasingment(r) if r.ptr == Some(PointerType::Pointer)   => Value::Deref(r.name.clone()),
            TableTypes::Reasingment(r) if r.ptr == Some(PointerType::Reference) => Value::Ref(r.name.clone()),
            TableTypes::Variable(v) => {
                if let Some(val) = v.value.as_ref().and_then(|v| v.first()) {
                    val.clone()
                } else if let Some(name) = &v.name {
                    if let Some((arr_name, idx_str)) = crate::variable_types::parse_array_syntax(name) {
                        Value::Index(arr_name, idx_str)
                    } else {
                        Value::Var(name.clone())
                    }
                } else {
                    Value::Var(String::new())
                }
            }
            _ => {
                let op = Self::extract_operand(e);
                if let Some((arr_name, idx_str)) = crate::variable_types::parse_array_syntax(&op) {
                    Value::Index(arr_name, idx_str)
                } else {
                    Value::Var(op)
                }
            }
        }
    }

    fn add_function_call(&mut self, call: FunctionCall) {
        let mut tac = Tac {
            tac_type: Type::Call,
            result: None,
            arguments: vec![call.name],
            operator: None,
            value_type: None,
            is_ptr: false,
        };
 
        for parameter in call.parameters.unwrap_or_default() {
            if let TableTypes::Reasingment(ref r) = parameter {
                if r.ptr == Some(PointerType::Pointer) || r.ptr == Some(PointerType::Reference) {
                    tac.arguments.push(Self::extract_call_arg(&parameter));
                    continue;
                }
            }
            let val = Self::table_type_to_value(&parameter);
            match val {
                Value::Index(arr_name, idx_str) => {
                    let tmp = self.next_temp();
                    self.tac_table.push(Tac {
                        tac_type: Type::ArrayIndex,
                        arguments: vec![arr_name, idx_str],
                        operator: None,
                        result: Some(tmp.clone()),
                        value_type: None,
                        is_ptr: false,
                    });
                    tac.arguments.push(tmp);
                }
                _ => {
                    tac.arguments.push(Self::extract_call_arg(&parameter));
                }
            }
        }
 
        self.tac_table.push(tac);
    }
 
    fn add_reasingment(&mut self, reassignment: Reasingment) {
        let target_ref = Self::symbol_ref(reassignment.target, &reassignment.target_scope);
        let value_type = Some(reassignment.token_type);

        let raw_tokens: Vec<Value> = reassignment.parameters.unwrap_or_default().iter().map(|e| Self::table_type_to_value(e)).collect();

        let left_op = if let Some(ref idx_str) = reassignment.array_index {
            Value::Index(reassignment.name.clone(), idx_str.clone())
        } else {
            Value::Var(reassignment.name.clone())
        };

        let tokens = if let Some(Value::Var(op_str)) = raw_tokens.first() {
            match op_str.as_str() {
                "+=" => {
                    let mut desugared = vec![left_op, Value::Var("+".to_string())];
                    desugared.extend(raw_tokens.into_iter().skip(1));
                    desugared
                }
                "-=" => {
                    let mut desugared = vec![left_op, Value::Var("-".to_string())];
                    desugared.extend(raw_tokens.into_iter().skip(1));
                    desugared
                }
                "++" => {
                    vec![left_op, Value::Var("+".to_string()), Value::Var("1".to_string())]
                }
                "--" => {
                    vec![left_op, Value::Var("-".to_string()), Value::Var("1".to_string())]
                }
                _ => raw_tokens,
            }
        } else {
            raw_tokens
        };

        if let Some(array_idx) = reassignment.array_index {
            let rhs_val = if tokens.len() == 1 {
                let (val, _) = self.evaluate_value(&tokens[0], value_type.clone());
                val
            } else {
                let tmp = self.next_temp();
                self.build_expression_chain(tokens, tmp.clone(), Type::Variable, value_type.clone());
                tmp
            };

            self.tac_table.push(Tac {
                tac_type: Type::ArrayAssign,
                arguments: vec![array_idx, rhs_val],
                operator: None,
                result: Some(reassignment.name),
                value_type,
                is_ptr: false,
            });
            return;
        }

        if reassignment.ptr == Some(PointerType::Pointer) {
            if tokens.len() == 1 {
                let rhs_val = match &tokens[0] {
                    Value::Var(s) => s.clone(),
                    Value::FuncCall(f) => {
                        let tmp = self.next_temp();
                        self.add_function_call(f.clone());
                        self.tac_table.push(Tac {
                            tac_type: Type::GetReturn,
                            arguments: vec![],
                            operator: None,
                            result: Some(tmp.clone()),
                            value_type: value_type.clone(),
                            is_ptr: false,
                        });
                        tmp
                    }
                    Value::Deref(_) | Value::Ref(_) | Value::Index(_, _) => {
                        let tmp = self.next_temp();
                        self.build_expression_chain(tokens, tmp.clone(), Type::Variable, value_type.clone());
                        tmp
                    }
                };
                self.tac_table.push(Tac {
                    tac_type: Type::DerefAssign,
                    arguments: vec![reassignment.name.clone(), rhs_val],
                    operator: None,
                    result: Some(reassignment.name),
                    value_type,
                    is_ptr: false,
                });
            } else {
                let tmp = self.next_temp();
                self.build_expression_chain(tokens, tmp.clone(), Type::Variable, value_type.clone());
                self.tac_table.push(Tac {
                    tac_type: Type::DerefAssign,
                    arguments: vec![reassignment.name.clone(), tmp],
                    operator: None,
                    result: Some(reassignment.name),
                    value_type,
                    is_ptr: false,
                });
            }
            return;
        }

        self.build_expression_chain(tokens, target_ref, Type::Reasingment, value_type);
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
            tac.arguments.push(
                last.result.clone().unwrap_or_else(|| "0".to_string())
            );
        }
    }

    fn add_return(&mut self, returns: Return) {
        let value_type = self.current_return_type.or_else(|| returns.value.as_ref().map(|v| v.token_type));
        let tokens = returns.value.and_then(|v| v.value).unwrap_or_default();

        if tokens.is_empty() {
            self.tac_table.push(Tac {
                tac_type: Type::Return,
                arguments: vec![],
                operator: None,
                result: None,
                value_type: self.current_return_type,
                is_ptr: false,
            });
            return;
        }

        let target = self.next_temp();
        self.build_expression_chain(tokens, target, Type::Return, value_type);
    }

    pub fn _print(&self) {
        let mut indent: usize = 0;
        println!("\n---------------------- \n");
        for tac in &self.tac_table {
            if matches!(tac.tac_type, Type::LoopEnd | Type::ConditionalEnd | Type::Function) {
                indent = indent.saturating_sub(1);
            }
 
            let pad = "    ".repeat(indent);
            println!("{}", Self::_format_tac(tac, &pad));
 
            if matches!( tac.tac_type, Type::Loop | Type::Conditional ) {
                indent += 1;
            } else if matches!( tac.tac_type, Type::Function) {
                indent = 1;
            }
        }
        println!("\n---------------------- \n");
    }
 
    fn _format_tac(tac: &Tac, pad: &str) -> String {
        match &tac.tac_type {
            Type::Function => {
                let name = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                format!("{pad}function {name}")
            }
            Type::Param => {
                let name = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                let ty = tac.value_type.map_or("unknown".to_string(), |t| t.to_str());
                format!("{pad}param {ty} {name}")
            }
            Type::LoopEnd => {
                let label = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                format!("{pad}{label}: end while")
            }
            Type::Conditional => {
                match (&tac.operator, tac.arguments.get(1..).unwrap_or(&[])) {
                    (Some(op), [left, right]) => {
                        format!("{pad} if ({left} {} {right})", op._as_str())
                    }
                    (_, rest) => format!("{pad} if ({})", rest.join(", ")),
                }
            }
            Type::Loop => {
                match (&tac.operator, tac.arguments.get(1..).unwrap_or(&[])) {
                    (Some(op), [left, right]) => {
                        format!("{pad} while ({left} {} {right})", op._as_str())
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
            Type::Variable | Type::Reasingment | Type::Return => {
                let result = tac.result.as_deref().unwrap_or("?");
                match (&tac.operator, tac.arguments.as_slice()) {
                    (Some(op), [left, right]) => {
                        format!("{pad}{result} = {left} {} {right}", op._as_str())
                    }
                    (None, [value]) => format!("{pad}{result} = {value}"),
                    _ => format!("{pad}{result} = {:?}", tac.arguments),
                }
            }
            Type::AddressOf => {
                let result = tac.result.as_deref().unwrap_or("?");
                let src = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                format!("{pad}{result} = &{src}")
            }
            Type::Deref => {
                let result = tac.result.as_deref().unwrap_or("?");
                let src = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                format!("{pad}{result} = *{src}")
            }
            Type::DerefAssign => {
                let ptr = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                let val = tac.arguments.get(1).map(String::as_str).unwrap_or("?");
                format!("{pad}*{ptr} = {val}")
            }
            Type::ArrayDecl => {
                let result = tac.result.as_deref().unwrap_or("?");
                let size = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                format!("{pad}array {result}[{size}]")
            }
            Type::ArrayIndex => {
                let result = tac.result.as_deref().unwrap_or("?");
                let arr = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                let idx = tac.arguments.get(1).map(String::as_str).unwrap_or("?");
                format!("{pad}{result} = {arr}[{idx}]")
            }
            Type::ArrayAssign => {
                let arr = tac.result.as_deref().unwrap_or("?");
                let idx = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                let val = tac.arguments.get(1).map(String::as_str).unwrap_or("?");
                format!("{pad}{arr}[{idx}] = {val}")
            }
            Type::Label => {
                let label = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                format!("{pad}{label}:")
            }
            Type::GetReturn => {
                format!("{pad}get return value")
            }
        }
    }
}
 
pub fn generate_three_address_code(type_table: Vec<TableTypes>, debug: bool) -> Vec<Tac>{
    let mut generator = ThreeAddressCodeGenerator::new();
    generator.generate(type_table);
    
    if debug { 
        generator._print(); 
    }
    
    generator.tac_table
}
