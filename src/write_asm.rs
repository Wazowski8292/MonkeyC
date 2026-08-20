use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

pub fn write_asm(file_name: &str, code: &Vec<String>, simple_debug: bool) {
    let mut file = File::create(file_name).expect("Failed to create file");
    for line in code {
        file.write_all(line.as_bytes()).expect("Failed to write to file");
        file.write_all(b"\n").expect("Failed to write to file");
    }

    let obj_path = file_name.replace(".asm", ".o");
    let bin_path = file_name.replace(".asm", "");

    if simple_debug { println!("Assembling with nasm..."); }
    let nasm_status = Command::new("nasm")
        .args(["-f", "elf64", &file_name, "-o", &obj_path])
        .status()
        .expect("Failed to run nasm — is it installed?");
    if !nasm_status.success() {
        eprintln!("nasm failed with exit code: {}", nasm_status);
        return;
    }

    if simple_debug { println!("Linking with gcc..."); }
    let link_status = Command::new("gcc")
        .args([&obj_path, "-no-pie", "-o", &bin_path])
        .status()
        .expect("Failed to run gcc — is it installed?");
    if !link_status.success() {
        eprintln!("gcc failed with exit code: {}", link_status);
        return;
    }

    println!("Done! Binary Compiled to: {}", bin_path);
}