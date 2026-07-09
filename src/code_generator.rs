use std::vec::Vec;
use std::collections::HashMap;
use crate::three_address_code_gen::{Tac, Type};

const ARG_REGS: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

enum Slot {
    Mem(i32),
    Const(String),
}

impl Slot {
    fn to_asm(&self) -> String {
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
}

impl CodeGen {
    pub fn new() -> Self {
        Self {
            file: vec![],
            slot_map: HashMap::new(),
            offset: 0,
        }
    }
     
    pub fn generate(&mut self, tac_table: Vec<Tac>) {
        for tac in tac_table.iter() {
            match tac.tac_type {
                Type::Variable => self.add_variable(tac),
                Type::Function => self.add_function(tac),
                Type::Call => self.add_function_call(tac),
                _ => {}
            }
        }
    }

    fn emit(&mut self, instr: &str) {
        self.file.push(instr.to_string());
    }

    fn emit_label(&mut self, label: &str) {
        self.file.push(format!("{}:", label));
    }

    fn code_gen_bin_op(&mut self, op: &str, a: Slot, b: Slot, t_offset: i32) {
        self.emit(&format!("    mov rax, {}", a.to_asm()));
        self.emit(&format!("    {} rax, {}", op, b.to_asm()));
        self.emit(&format!("    mov [rbp - {}], rax", t_offset));
        self.emit("");
    }

    fn code_gen_bin_copy(&mut self, a: Slot, t_offset: i32) {
        self.emit(&format!("    mov rax, {}", a.to_asm()));
        self.emit(&format!("    mov [rbp - {}], rax", t_offset));
        self.emit("");
    }

    fn get_or_alloc_slot(&mut self, name: &str) -> Slot {
        if name.parse::<f64>().is_ok() {
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
            Some(op) => {
                let b_slot = self.get_or_alloc_slot(&variable.arguments[1]);
                self.code_gen_bin_op(op.to_asm().unwrap(), a_slot, b_slot, t_offset);
            }
        }
    }

    fn add_function(&mut self, function: &Tac) {
        let name = function.arguments.get(1).map(String::as_str).unwrap_or("?");
        let params: Vec<String> = function.arguments.iter().skip(2).cloned().collect();

        if name == "main" {
            self.emit_label("_start");
        } else {
            self.emit_label(name);
        }

        self.emit("    push rbp");
        self.emit("    mov rbp, rsp");
        self.emit("    sub rsp, 128"); // Should not use fix size block. TODO: Dinimaic Size block
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

    fn add_function_call(&mut self, call: &Tac) {
        let name = call.arguments.get(0).map(String::as_str).unwrap_or("?");
        let arg_names: Vec<String> = call.arguments.iter().skip(1).cloned().collect();

        for (i, arg_name) in arg_names.iter().enumerate() {
            if i >= ARG_REGS.len() {
                break; // TODO: Need to implement the ability to pass more parameters
            }
            let slot = self.get_or_alloc_slot(arg_name);
            self.emit(&format!("    mov {}, {}", ARG_REGS[i], slot.to_asm()));
        }

        self.emit(&format!("    call {}", name));
        self.emit("");
    }

    // Returns values
    fn add_epilogue(&mut self) {
        self.emit("    mov rsp, rbp");
        self.emit("    pop rbp");
        self.emit("    ret");
        self.emit("");
    }
}

pub fn generate_assembly(tac_table: Vec<Tac>) {
    let mut code_gen = CodeGen::new();
    code_gen.generate(tac_table);

    println!("Assembly output: {:#?}", code_gen.file);
}