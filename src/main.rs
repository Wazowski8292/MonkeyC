use std::env;
use std::process::Command;
use monkey_c::parser::parse_text;
use monkey_c::semantic_analyzer::analyze_semantically;
use monkey_c::three_address_code_gen::generate_three_address_code;
use monkey_c::code_generator::generate_assembly;
use monkey_c::write_asm::write_asm;

fn main() {
    let args: Vec<String> = env::args().collect();

    check_args(args.clone());

    println!("Parsing...");
    let parsed_text = parse_text(&args[1]);
    match parsed_text {
        Err(msg) => {println!("{}", msg); return;},
        _ => {}
    }

    println!("Analyzing semanticly...");
    let type_table = analyze_semantically(parsed_text.expect("parsing failed"));

    println!("Translating into a three address code...");
    let tac = generate_three_address_code(type_table);

    println!("Translating into assembly...");
    let asm = generate_assembly(tac);

    let asm_path = args[1].replace(".MC", ".asm");
    let obj_path = args[1].replace(".MC", ".o");
    let bin_path = args[1].replace(".MC", "");

    println!("Writing assembly to file...");
    write_asm(&asm_path, &asm);

    println!("Assembling with nasm...");
    let nasm_status = Command::new("nasm")
        .args(["-f", "elf64", &asm_path, "-o", &obj_path])
        .status()
        .expect("Failed to run nasm — is it installed?");
    if !nasm_status.success() {
        eprintln!("nasm failed with exit code: {}", nasm_status);
        return;
    }

    println!("Linking with gcc...");
    let link_status = Command::new("gcc")
        .args([&obj_path, "-no-pie", "-o", &bin_path])
        .status()
        .expect("Failed to run gcc — is it installed?");
    if !link_status.success() {
        eprintln!("gcc failed with exit code: {}", link_status);
        return;
    }

    println!("Done! Binary written to: {}", bin_path);
}


fn check_args(args: Vec<String>) {
    if args.len() == 1 {
        println!("You did not pass the file directory");
        return;
    }
    if !args[1].ends_with(".MC") {
        println!("File is not a monkey_c program");
        return;
    } 
}
