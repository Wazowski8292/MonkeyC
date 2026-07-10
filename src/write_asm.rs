use std::fs::File;
use std::io::prelude::*;

pub fn write_asm(file_name: &str, code: &Vec<String>) {
    let mut file = File::create(file_name).expect("Failed to create file");
    for line in code {
        file.write_all(line.as_bytes()).expect("Failed to write to file");
        file.write_all(b"\n").expect("Failed to write to file");
    }
}