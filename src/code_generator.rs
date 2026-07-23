use std::vec::Vec;
use std::collections::HashMap;
use crate::three_address_code_gen::{Tac, Type, Operator};
use crate::semantic_analyzer::TokenType;
use crate::enbeded_funcs::FUNCTIONS;

const ARG_REGS: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
const FP_ARG_REGS: [&str; 8] = ["xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5", "xmm6", "xmm7"];

enum Slot {
    Mem(i32),
    Const(String),
    Data(String),
}

impl Slot {
    fn to_asm_op(&self) -> String {
        match self {
            Slot::Mem(off) => format!("[rbp - {}]", off),
            Slot::Const(val) => val.clone(),
            Slot::Data(label) => format!("[rel {}]", label),
        }
    }
}

struct CodeGen {
    file: Vec<String>,
    rodata: Vec<String>,
    slot_map: HashMap<String, (i32, TokenType)>,
    offset: i32,
    current_fn: String,
    enbeded_funcs: Vec<String>,
    fp_const_count: usize,
    param_int_idx: usize,
    param_fp_idx: usize,
}

impl CodeGen {
    pub fn new() -> Self {
        Self {
            file: vec![],
            rodata: vec![],
            slot_map: HashMap::new(),
            offset: 0,
            current_fn: String::new(),
            enbeded_funcs: vec![],
            fp_const_count: 0,
            param_int_idx: 0,
            param_fp_idx: 0,
        }
    }
     
    pub fn generate(&mut self, tac_table: Vec<Tac>) {
        for (_, tac) in tac_table.iter().enumerate() {
            match tac.tac_type {
                Type::Variable | Type::Reasingment => self.add_variable(tac),
                Type::Function => {
                    self.slot_map.clear();
                    self.offset = 0;
                    self.param_int_idx = 0;
                    self.param_fp_idx = 0;
                    self.add_function(tac);
                }
                Type::Param => self.add_param(tac),
                Type::Call => self.add_function_call(tac),
                Type::Conditional => self.add_conditional(tac),
                Type::Loop => self.add_loop_start(tac),
                Type::ConditionalEnd => self.add_conditional_end(tac),
                Type::LoopEnd => self.add_loop_end(tac),
                Type::Label => self.emit_label(&tac.arguments[0]),
                Type::GetReturn => self.add_get_return(tac),
                Type::Return => self.add_return(tac),
            }
        }
        if !self.current_fn.is_empty() {
            if self.current_fn == "main" {
                self.add_exit_syscall();
            } else {
                self.add_epilogue();
            }
        }
    }

    fn emit(&mut self, instr: &str) {
        self.file.push(instr.to_string());
    }

    fn emit_label(&mut self, label: &str) {
        self.file.push(format!("    {}:", label));
    }

    fn tac_is_float(tac: &Tac) -> bool {
        matches!(
            tac.value_type,
            Some(TokenType::Float) | Some(TokenType::FloatLiteral)
        )
    }

    fn tac_is_double(tac: &Tac) -> bool {
        matches!(
            tac.value_type,
            Some(TokenType::Double) | Some(TokenType::DoubleLiteral)
        )
    }

    fn code_gen_bin_op(&mut self, op: &str, a: Slot, b: Slot, t_offset: i32) {
        self.emit(&format!("    mov rax, {}", a.to_asm_op()));
        self.emit(&format!("    {} rax, {}", op, b.to_asm_op()));
        self.emit(&format!("    mov [rbp - {}], rax", t_offset));
        self.emit("");
    }

    fn code_gen_bin_copy(&mut self, a: Slot, t_offset: i32) {
        self.emit(&format!("    mov rax, {}", a.to_asm_op()));
        self.emit(&format!("    mov [rbp - {}], rax", t_offset));
        self.emit("");
    }

    fn code_gen_logical_op(&mut self, combine: &str, a: Slot, b: Slot, t_offset: i32) {
        self.emit(&format!("    mov rax, {}", a.to_asm_op()));
        self.emit("    test rax, rax");
        self.emit("    setne al");
        self.emit("    movzx rax, al");
        self.emit(&format!("    mov rcx, {}", b.to_asm_op()));
        self.emit("    test rcx, rcx");
        self.emit("    setne cl");
        self.emit("    movzx rcx, cl");
        self.emit(&format!("    {} rax, rcx", combine));
        self.emit(&format!("    mov [rbp - {}], rax", t_offset));
        self.emit("");
    }

    fn code_gen_f32_bin_op(&mut self, op: &str, a: Slot, b: Slot, t_offset: i32) {
        self.emit(&format!("    movss xmm0, dword {}", a.to_asm_op()));
        self.emit(&format!("    movss xmm1, dword {}", b.to_asm_op()));
        self.emit(&format!("    {} xmm0, xmm1", op));
        self.emit(&format!("    movss dword [rbp - {}], xmm0", t_offset));
        self.emit("");
    }

    fn code_gen_f32_copy(&mut self, a: Slot, t_offset: i32) {
        self.emit(&format!("    movss xmm0, dword {}", a.to_asm_op()));
        self.emit(&format!("    movss dword [rbp - {}], xmm0", t_offset));
        self.emit("");
    }

    fn code_gen_f64_bin_op(&mut self, op: &str, a: Slot, b: Slot, t_offset: i32) {
        self.emit(&format!("    movsd xmm0, qword {}", a.to_asm_op()));
        self.emit(&format!("    movsd xmm1, qword {}", b.to_asm_op()));
        self.emit(&format!("    {} xmm0, xmm1", op));
        self.emit(&format!("    movsd qword [rbp - {}], xmm0", t_offset));
        self.emit("");
    }

    fn code_gen_f64_copy(&mut self, a: Slot, t_offset: i32) {
        self.emit(&format!("    movsd xmm0, qword {}", a.to_asm_op()));
        self.emit(&format!("    movsd qword [rbp - {}], xmm0", t_offset));
        self.emit("");
    }

    fn get_or_alloc_slot(&mut self, name: &str) -> Slot {
        let token = TokenType::from_str(name);

        if token == TokenType::FloatLiteral {
            let label = format!("__flt_{}", self.fp_const_count);
            self.fp_const_count += 1;
            let val = if name.ends_with('f') || name.ends_with('F') {
                &name[..name.len()-1]
            } else {
                name
            };
            self.rodata.push(format!("    {} dd {}", label, val));
            return Slot::Data(label);
        }

        if token == TokenType::DoubleLiteral {
            let label = format!("__dbl_{}", self.fp_const_count);
            self.fp_const_count += 1;
            self.rodata.push(format!("    {} dq {}", label, name));
            return Slot::Data(label);
        }

        if TokenType::is_value(token) && token != TokenType::Unknow {
            if token == TokenType::BoolLiteral {
                if name == "true" {
                    return Slot::Const(1.to_string());
                } else {
                    return Slot::Const(0.to_string());
                }
            }

            return Slot::Const(name.to_string());
        }

        if let Some(&existing) = self.slot_map.get(name) {
            return Slot::Mem(-existing.0);
        }
        self.offset -= 8;
        self.slot_map.insert(name.to_string(), (self.offset, token));
        Slot::Mem(-self.offset)
    }

    fn add_variable(&mut self, variable: &Tac) {
        let name = variable.clone().result.unwrap();
        let t_offset = match self.get_or_alloc_slot(&name) {
            Slot::Mem(off) => off,
            Slot::Const(_) => panic!("assignment target cannot be a constant"),
            Slot::Data(_) => panic!("assignment target cannot be a rodata label"),
        };

        let a_arg = &variable.arguments[0];
        let a_tok = self.slot_map.get(a_arg).map(|(_, t)| *t).unwrap_or_else(|| TokenType::from_str(a_arg));
        let is_f32 = Self::tac_is_float(variable) || matches!(a_tok, TokenType::Float | TokenType::FloatLiteral);
        let is_f64 = Self::tac_is_double(variable) || matches!(a_tok, TokenType::Double | TokenType::DoubleLiteral);

        let a_slot = self.get_or_alloc_slot(&variable.arguments[0]);

        self.slot_map.insert(name.to_string(), (-t_offset, a_tok));

        match &variable.operator {
            None => {
                if is_f32 {
                    self.code_gen_f32_copy(a_slot, t_offset);
                } else if is_f64 {
                    self.code_gen_f64_copy(a_slot, t_offset);
                } else {
                    self.code_gen_bin_copy(a_slot, t_offset);
                }
            }
            Some(Operator::LogicalAnd) => {
                let b_slot = self.get_or_alloc_slot(&variable.arguments[1]);
                self.code_gen_logical_op("and", a_slot, b_slot, t_offset);
            }
            Some(Operator::LogicalOr) => {
                let b_slot = self.get_or_alloc_slot(&variable.arguments[1]);
                self.code_gen_logical_op("or", a_slot, b_slot, t_offset);
            }
            Some(Operator::LogicalEquals) => {
                let b_slot = self.get_or_alloc_slot(&variable.arguments[1]);
                if is_f32 {
                    self.emit(&format!("    movss xmm0, dword {}", a_slot.to_asm_op()));
                    self.emit(&format!("    ucomiss xmm0, dword {}", b_slot.to_asm_op()));
                    self.emit("    sete al");
                    self.emit("    movzx rax, al");
                    self.emit(&format!("    mov [rbp - {}], rax", t_offset));
                    self.emit("");
                } else if is_f64 {
                    self.emit(&format!("    movsd xmm0, qword {}", a_slot.to_asm_op()));
                    self.emit(&format!("    ucomisd xmm0, qword {}", b_slot.to_asm_op()));
                    self.emit("    sete al");
                    self.emit("    movzx rax, al");
                    self.emit(&format!("    mov [rbp - {}], rax", t_offset));
                    self.emit("");
                } else {
                    self.emit(&format!("    mov rax, {}", a_slot.to_asm_op()));
                    self.emit(&format!("    cmp rax, {}", b_slot.to_asm_op()));
                    self.emit("    sete al");
                    self.emit("    movzx rax, al");
                    self.emit(&format!("    mov [rbp - {}], rax", t_offset));
                    self.emit("");
                }
            }
            Some(op) => {
                let b_slot = self.get_or_alloc_slot(&variable.arguments[1]);
                if is_f32 {
                    if let Some(asm_op) = op.to_asm_op_f32() {
                        self.code_gen_f32_bin_op(asm_op, a_slot, b_slot, t_offset);
                    } else {
                        self.code_gen_f32_bin_op("addss", a_slot, b_slot, t_offset);
                    }
                } else if is_f64 {
                    if let Some(asm_op) = op.to_asm_op_f64() {
                        self.code_gen_f64_bin_op(asm_op, a_slot, b_slot, t_offset);
                    } else {
                        self.code_gen_f64_bin_op("addsd", a_slot, b_slot, t_offset);
                    }
                } else {
                    self.code_gen_bin_op(op.to_asm_op().unwrap_or("?"), a_slot, b_slot, t_offset);
                }
            }
        }
    }

    fn add_function(&mut self, function: &Tac) {
        let name = function.arguments.get(0).map(String::as_str).unwrap_or("?");
        let memory_alloc = function.arguments.get(1).map(String::as_str).unwrap_or("?");

        self.current_fn = name.to_string();

        if name == "main" {
            self.emit("main:");
        } else {
            self.emit(&format!("{}:", name));
        }

        self.emit("    push rbp");
        self.emit("    mov rbp, rsp");
        self.emit(&format!("    sub rsp, {}", memory_alloc));
        self.emit("");
    }

    fn add_param(&mut self, param: &Tac) {
        let param_name = param.arguments.get(0).map(String::as_str).unwrap_or("?");
        let param_type = param.value_type.unwrap_or(TokenType::Int);

        self.offset -= 8;
        let offset = self.offset;
        self.slot_map.insert(param_name.to_string(), (offset, param_type));

        let is_f32 = matches!(param_type, TokenType::Float);
        let is_f64 = matches!(param_type, TokenType::Double);

        if is_f32 {
            if self.param_fp_idx < FP_ARG_REGS.len() {
                self.emit(&format!("    movss dword [rbp - {}], {}", -offset, FP_ARG_REGS[self.param_fp_idx]));
                self.param_fp_idx += 1;
            }
        } else if is_f64 {
            if self.param_fp_idx < FP_ARG_REGS.len() {
                self.emit(&format!("    movsd qword [rbp - {}], {}", -offset, FP_ARG_REGS[self.param_fp_idx]));
                self.param_fp_idx += 1;
            }
        } else {
            if self.param_int_idx < ARG_REGS.len() {
                self.emit(&format!("    mov [rbp - {}], {}", -offset, ARG_REGS[self.param_int_idx]));
                self.param_int_idx += 1;
            }
        }
    }

    fn add_exit_syscall(&mut self) {
        self.emit("    ; exit main");
        self.emit("    mov rax, 0");
        self.emit("    mov rsp, rbp");
        self.emit("    pop rbp");
        self.emit("    ret");
        self.emit("");
    }

    fn add_function_call(&mut self, call: &Tac) {
        let name = call.arguments.get(0).map(String::as_str).unwrap_or("?");
        let arg_names: Vec<String> = call.arguments.iter().skip(1).cloned().collect();

        let mut int_i = 0;
        let mut fp_i = 0;

        for arg_name in arg_names.iter() {
            let slot = self.get_or_alloc_slot(arg_name);

            let tok = self.slot_map.get(arg_name).map(|(_, t)| *t).unwrap_or_else(|| TokenType::from_str(arg_name));
            let is_f32 = matches!(tok, TokenType::Float | TokenType::FloatLiteral);
            let is_f64 = matches!(tok, TokenType::Double | TokenType::DoubleLiteral);

            if is_f32 {
                if fp_i < FP_ARG_REGS.len() {
                    self.emit(&format!("    movss {}, dword {}", FP_ARG_REGS[fp_i], slot.to_asm_op()));
                    fp_i += 1;
                }
            } else if is_f64 {
                if fp_i < FP_ARG_REGS.len() {
                    self.emit(&format!("    movsd {}, qword {}", FP_ARG_REGS[fp_i], slot.to_asm_op()));
                    fp_i += 1;
                }
            } else {
                if int_i < ARG_REGS.len() {
                    self.emit(&format!("    mov {}, {}", ARG_REGS[int_i], slot.to_asm_op()));
                    int_i += 1;
                }
            }
        }

        self.emit(&format!("    call {}", name));
        self.emit("");

        self.add_enbeded_func(name.to_string())
    }

    fn add_enbeded_func(&mut self, name: String) {
        for func in FUNCTIONS.iter() {
            if func.name == name && !self.enbeded_funcs.contains(&name) {
                let mut current_line: String = Default::default();
                let mut counter = 0;

                for line in func.function.chars() {
                    current_line.push(line.clone());

                    if line == '\n' {
                        if counter == 0 {
                            counter = 1;
                        } else {
                            self.enbeded_funcs.push(current_line.clone());
                            current_line.clear();
                            counter = 0;
                        }
                    }
                }
                return;
            }
        }
    }

    fn add_get_return(&mut self, get_return: &Tac) {
        let t_offset = match self.get_or_alloc_slot(&get_return.clone().result.unwrap()) {
            Slot::Mem(off) => off,
            Slot::Const(_) | Slot::Data(_) => panic!("assignment target cannot be a constant"),
        };

        if Self::tac_is_float(get_return) {
            self.emit(&format!("    movss dword [rbp - {}], xmm0", t_offset));
        } else if Self::tac_is_double(get_return) {
            self.emit(&format!("    movsd qword [rbp - {}], xmm0", t_offset));
        } else {
            self.emit(&format!("    mov [rbp - {}], rax", t_offset));
        }
    }

    fn add_return(&mut self, tac: &Tac) {
        let is_f32 = Self::tac_is_float(tac);
        let is_f64 = Self::tac_is_double(tac);

        if let Some(value) = &tac.result {
            self.add_variable(&Tac {
                tac_type: Type::Variable,
                arguments: tac.arguments.clone(),
                operator: tac.operator.clone(),
                result: tac.result.clone(),
                value_type: tac.value_type.clone(),
            });

            let slot = self.get_or_alloc_slot(value);
            if is_f32 {
                self.emit(&format!("    movss xmm0, dword {}", slot.to_asm_op()));
            } else if is_f64 {
                self.emit(&format!("    movsd xmm0, qword {}", slot.to_asm_op()));
            } else {
                self.emit(&format!("    mov rax, {}", slot.to_asm_op()));
            }
            self.emit("");
        }

        if self.current_fn == "main" {
            self.add_exit_syscall();
        } else {
            self.add_epilogue();
        }
    }

    // Returns values
    fn add_epilogue(&mut self) {
        self.emit("    mov rsp, rbp");
        self.emit("    pop rbp");
        self.emit("    ret");
        self.emit("");
    }

    fn emit_condition_jump(&mut self, label: &str, tac: &Tac) {
        match (&tac.operator, tac.arguments.get(1..).unwrap_or(&[])) {
            (Some(op), [left, right]) => {
                let a_slot = self.get_or_alloc_slot(left);
                let b_slot = self.get_or_alloc_slot(right);

                let a_tok = self.slot_map.get(left).map(|(_, t)| *t).unwrap_or_else(|| TokenType::from_str(left));
                let b_tok = self.slot_map.get(right).map(|(_, t)| *t).unwrap_or_else(|| TokenType::from_str(right));

                let is_f32 = matches!(a_tok, TokenType::Float | TokenType::FloatLiteral) || matches!(b_tok, TokenType::Float | TokenType::FloatLiteral);
                let is_f64 = matches!(a_tok, TokenType::Double | TokenType::DoubleLiteral) || matches!(b_tok, TokenType::Double | TokenType::DoubleLiteral);

                if is_f32 {
                    self.emit(&format!("    movss xmm0, dword {}", a_slot.to_asm_op()));
                    self.emit(&format!("    ucomiss xmm0, dword {}", b_slot.to_asm_op()));
                } else if is_f64 {
                    self.emit(&format!("    movsd xmm0, qword {}", a_slot.to_asm_op()));
                    self.emit(&format!("    ucomisd xmm0, qword {}", b_slot.to_asm_op()));
                } else {
                    self.emit(&format!("    mov rax, {}", a_slot.to_asm_op()));
                    self.emit(&format!("    cmp rax, {}", b_slot.to_asm_op()));
                }

                let jump_mnemonic = match op {
                    Operator::LogicalEquals => "jne",
                    _ => panic!("unsupported conditional operator: {:?}", op),
                };
                self.emit(&format!("    {} {}", jump_mnemonic, label));
            }
            (_, [value]) => {
                let slot = self.get_or_alloc_slot(value);
                let tok = self.slot_map.get(value).map(|(_, t)| *t).unwrap_or_else(|| TokenType::from_str(value));
                let is_f32 = matches!(tok, TokenType::Float | TokenType::FloatLiteral);
                let is_f64 = matches!(tok, TokenType::Double | TokenType::DoubleLiteral);

                if is_f32 {
                    self.emit(&format!("    movss xmm0, dword {}", slot.to_asm_op()));
                    self.emit("    xorps xmm1, xmm1");
                    self.emit("    ucomiss xmm0, xmm1");
                    self.emit(&format!("    je {}", label));
                } else if is_f64 {
                    self.emit(&format!("    movsd xmm0, qword {}", slot.to_asm_op()));
                    self.emit("    xorpd xmm1, xmm1");
                    self.emit("    ucomisd xmm0, xmm1");
                    self.emit(&format!("    je {}", label));
                } else {
                    self.emit(&format!("    mov rax, {}", slot.to_asm_op()));
                    self.emit("    test rax, rax");
                    self.emit(&format!("    je {}", label));
                }
            }
            _ => panic!("TAC has no usable condition"),
        }
        self.emit("");
    }

    fn add_conditional(&mut self, conditional: &Tac) {
        let label = conditional.arguments.get(0).expect("conditional TAC needs a label").clone();
        self.emit_condition_jump(&label, conditional);
    }

    fn add_conditional_end(&mut self, conditional: &Tac) {
        let label = conditional.arguments.get(0).map(String::as_str).unwrap_or("?");
        self.emit_label(label);
        self.emit("");
    }

    fn add_loop_start(&mut self, loop_tac: &Tac) {
        let label = loop_tac.arguments.get(0).expect("loop TAC needs a label").clone();
        self.emit_condition_jump(&label, loop_tac);
    }

    fn add_loop_end(&mut self, loop_tac: &Tac) {
        let label = loop_tac.arguments.get(0).map(String::as_str).unwrap_or("?");

        self.emit(&format!("    jmp {}_loop", label));
        self.emit("");
        self.emit_label(label);
        self.emit("");
    }
}



pub fn generate_assembly(tac_table: Vec<Tac>) -> Vec<String> {
    let mut code_gen = CodeGen::new();
    code_gen.generate(tac_table);

    let mut out = vec![
        "bits 64".to_string(),
        "".to_string(),
        "section .data".to_string(),
        "    fmt_int    db \"%d\", 10, 0".to_string(),
        "    fmt_float    db \"%f\", 10, 0".to_string(),
        "    fmt_bool   db \"%s\", 10, 0".to_string(),
        "    fmt_char   db \"%c\", 10, 0".to_string(),
        "    fmt_string db \"%s\", 10, 0".to_string(),
        "    str_true   db \"true\", 0".to_string(),
        "    str_false  db \"false\", 0".to_string(),
        "".to_string(),
    ];

    if !code_gen.rodata.is_empty() {
        out.push("section .rodata".to_string());
        out.extend(code_gen.rodata);
        out.push("".to_string());
    }

    out.extend(vec![
        "section .text".to_string(),
        "    extern printf".to_string(),
        "    global main".to_string(),
        String::new(),
    ]);
    out.extend(code_gen.enbeded_funcs);
    out.extend(code_gen.file);
    out
}