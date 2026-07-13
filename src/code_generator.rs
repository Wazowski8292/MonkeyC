use std::vec::Vec;
use std::collections::HashMap;
use crate::three_address_code_gen::{Tac, Type, Operator};
use crate::semantic_analyzer::TokenType;
use crate::enbeded_funcs::FUNCTIONS;

const ARG_REGS: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

enum Slot {
    Mem(i32),
    Const(String),
}

impl Slot {
    fn to_asm_op(&self) -> String {
        match self {
            Slot::Mem(off) => format!("[rbp - {}]", off),
            Slot::Const(val) => val.clone(),
        }
    }
}

struct CodeGen {
    file: Vec<String>,
    slot_map: HashMap<String, i32>,
    offset: i32,
    current_fn: String,
    enbeded_funcs: Vec<String>,
}

impl CodeGen {
    pub fn new() -> Self {
        Self {
            file: vec![],
            slot_map: HashMap::new(),
            offset: 0,
            current_fn: String::new(),
            enbeded_funcs: vec![],
        }
    }
     
    pub fn generate(&mut self, tac_table: Vec<Tac>) {
        let len = tac_table.len();
        for (i, tac) in tac_table.iter().enumerate() {
            match tac.tac_type {
                Type::Variable | Type::Reasingment => self.add_variable(tac),
                Type::Function => {
                    if self.current_fn == "main" {
                        self.add_exit_syscall();
                    } else if !self.current_fn.is_empty() {
                        self.add_epilogue();
                    }
                    self.slot_map.clear();
                    self.offset = 0;
                    self.add_function(tac);
                }
                Type::Call => self.add_function_call(tac),
                Type::Conditional => self.add_conditional(tac),
                Type::Loop => self.add_loop_start(tac),
                Type::ConditionalEnd => {
                    let label = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                    self.emit_label(label);
                    self.emit("");
                    if i == len - 1 && self.current_fn == "main" {
                        self.add_exit_syscall();
                    }
                }
                Type::LoopEnd => {
                    let label = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                    // jump back to re-evaluate the condition
                    self.emit(&format!("    jmp {}_loop", label));
                    self.emit("");
                    self.emit_label(label);
                    self.emit("");
                    if i == len - 1 && self.current_fn == "main" {
                        self.add_exit_syscall();
                    }
                }
            }
        }
        if !self.current_fn.is_empty() {
            let last_needs_epilogue = tac_table.last().map_or(false, |t| {
                !matches!(t.tac_type, Type::ConditionalEnd | Type::LoopEnd)
            });
            if last_needs_epilogue {
                if self.current_fn == "main" {
                    self.add_exit_syscall();
                } else {
                    self.add_epilogue();
                }
            }
        }
    }

    fn emit(&mut self, instr: &str) {
        self.file.push(instr.to_string());
    }

    fn emit_label(&mut self, label: &str) {
        self.file.push(format!("    {}:", label));
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

    fn get_or_alloc_slot(&mut self, name: &str) -> Slot {
        let token = TokenType::from_str(name);
        if TokenType::is_value(token) && name.chars().next() != Some('_') {
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
            return Slot::Mem(-existing);
        }
        self.offset -= 8;
        self.slot_map.insert(name.to_string(), self.offset);
        Slot::Mem(-self.offset)
    }

    fn add_variable(&mut self, variable: &Tac) {
        let t_offset = match self.get_or_alloc_slot(&variable.clone().result.unwrap()) {
            Slot::Mem(off) => off,
            Slot::Const(_) => panic!("assignment target cannot be a constant"),
        };

        let a_slot = self.get_or_alloc_slot(&variable.arguments[0]);

        match &variable.operator {
            None => {
                self.code_gen_bin_copy(a_slot, t_offset);
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
                self.emit(&format!("    mov rax, {}", a_slot.to_asm_op()));
                self.emit(&format!("    cmp rax, {}", b_slot.to_asm_op()));
                self.emit("    sete al");
                self.emit("    movzx rax, al");
                self.emit(&format!("    mov [rbp - {}], rax", t_offset));
                self.emit("");
            }
            Some(op) => {
                let b_slot = self.get_or_alloc_slot(&variable.arguments[1]);
                self.code_gen_bin_op(op.to_asm_op().unwrap_or("?"), a_slot, b_slot, t_offset);
            }
        }
    }

    fn add_function(&mut self, function: &Tac) {
        let name = function.arguments.get(0).map(String::as_str).unwrap_or("?");
        let memory_alloc = function.arguments.get(1).map(String::as_str).unwrap_or("?");
        let params: Vec<String> = function.arguments.iter().skip(2).cloned().collect();

        self.current_fn = name.to_string();

        if name == "main" {
            self.emit("main:");
        } else {
            self.emit(name);
        }

        self.emit("    push rbp");
        self.emit("    mov rbp, rsp");
        self.emit(&format!("    sub rsp, {}", memory_alloc));
        self.emit("");

        for (i, param_name) in params.iter().enumerate() {
            if i >= ARG_REGS.len() {
                break;  // TODO: Need to implement this.
            }
            let slot = self.get_or_alloc_slot(param_name);
            let offset = match slot {
                Slot::Mem(off) => off,
                Slot::Const(_) => unreachable!("param name should never be a constant"),
            };
            self.emit(&format!("    mov [rbp - {}], {}", offset, ARG_REGS[i]));
        }
        self.emit("");
    }

    fn add_exit_syscall(&mut self) {
        self.emit("    ; exit(0)");
        self.emit("    mov rax, 60");
        self.emit("    xor rdi, rdi");
        self.emit("    syscall");
        self.emit("");
    }

    fn add_function_call(&mut self, call: &Tac) {
        let name = call.arguments.get(0).map(String::as_str).unwrap_or("?");
        let arg_names: Vec<String> = call.arguments.iter().skip(1).cloned().collect();

        for (i, arg_name) in arg_names.iter().enumerate() {
            if i >= ARG_REGS.len() {
                break; // TODO: Need to implement the ability to pass more parameters
            }
            let slot = self.get_or_alloc_slot(arg_name);
            self.emit(&format!("    mov {}, {}", ARG_REGS[i], slot.to_asm_op()));
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

                self.emit(&format!("    mov rax, {}", a_slot.to_asm_op()));
                self.emit(&format!("    cmp rax, {}", b_slot.to_asm_op()));

                let jump_mnemonic = match op {
                    Operator::LogicalEquals => "jne",
                    _ => panic!("unsupported conditional operator: {:?}", op),
                };
                self.emit(&format!("    {} {}", jump_mnemonic, label));
            }
            (_, [value]) => {
                let slot = self.get_or_alloc_slot(value);
                self.emit(&format!("    mov rax, {}", slot.to_asm_op()));
                self.emit("    test rax, rax");
                self.emit(&format!("    je {}", label));
            }
            _ => panic!("TAC has no usable condition"),
        }
        self.emit("");
    }

    fn add_conditional(&mut self, conditional: &Tac) {
        let label = conditional.arguments.get(0).expect("conditional TAC needs a label").clone();
        self.emit_condition_jump(&label, conditional);
    }

    fn add_loop_start(&mut self, loop_tac: &Tac) {
        let label = loop_tac.arguments.get(0).expect("loop TAC needs a label").clone();
        self.emit_label(&format!("{}_loop", label));
        self.emit("");
        self.emit_condition_jump(&label, loop_tac);
    }
}



pub fn generate_assembly(tac_table: Vec<Tac>) -> Vec<String> {
    let mut code_gen = CodeGen::new();
    code_gen.generate(tac_table);

    let mut out = vec![
        "bits 64".to_string(),
        "".to_string(),
        "section .data".to_string(),
        "    fmt_int   db \"%d\", 10, 0".to_string(),
        "".to_string(),
        "section .text".to_string(),
        "    extern printf".to_string(),
        "    global main".to_string(),
        String::new(),
    ];
    out.extend(code_gen.enbeded_funcs);
    out.extend(code_gen.file);
    out
}