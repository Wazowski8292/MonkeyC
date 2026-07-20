use std::env;
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
    match type_table {
        Err(len) => {panic!("There {} {} compiler errors. Please fix the compiler error before compiling.", {if len == 1 { "is" } else {"are"}} , len);},
        _ => {}
    }

    println!("Translating into a three address code...");
    let tac = generate_three_address_code(type_table.expect("Compiler errors"));

    println!("Translating into assembly...");
    let asm = generate_assembly(tac);

    let asm_path = args[1].replace(".MC", ".asm");


    println!("Writing assembly to file...");
    write_asm(&asm_path, &asm);
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
