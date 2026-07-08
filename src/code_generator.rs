use std::vec::Vec;
use std::collections::HashMap;
use crate::three_address_code_gen::{Tac, Type};

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
}

pub fn generate_assembly(tac_table: Vec<Tac>) {
    let mut code_gen = CodeGen::new();
    code_gen.generate(tac_table);

    println!("Assembly output: {:#?}", code_gen.file);
}