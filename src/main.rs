use std::env;
use monkey_c::parser::parse_text;
use monkey_c::semantic_analyzer::analyze_semantically;
use monkey_c::three_address_code_gen::generate_three_address_code;
use monkey_c::code_generator::generate_assembly;
use monkey_c::write_asm::write_asm;

struct Flags {
    parser_debug: bool,
    semantic_analyzer_debug: bool,
    tac_debug: bool,
    file_name: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let Some(flags) = parse_parameters(args) else { return };

    println!("Parsing...");
    let parsed_text = parse_text(&flags.file_name, flags.parser_debug);
    match parsed_text {
        Err(msg) => {println!("{}", msg); return;},
        _ => {}
    }

    println!("Analyzing semanticly...");
    let type_table = analyze_semantically(parsed_text.expect("parsing failed"), flags.semantic_analyzer_debug);
    match type_table {
        Err(len) => {println!("There {} {} compiler error{}. Please fix the compiler error{} before compiling.", {if len == 1 { "is" } else {"are"}} , len, {if len == 1 { "" } else {"s"}}, {if len == 1 { "" } else {"s"}} ); return;},
        _ => {}
    }

    println!("Translating into a three address code...");
    let tac = generate_three_address_code(type_table.expect("Compiler errors"), flags.tac_debug);

    println!("Translating into assembly...");
    let asm = generate_assembly(tac);

    let asm_path = flags.file_name.replace(".MC", ".asm");


    println!("Writing assembly to file...");
    write_asm(&asm_path, &asm);
}

fn parse_parameters(args: Vec<String>) -> Option<Flags> {
    let mut flags = Flags{
        parser_debug: false,
        semantic_analyzer_debug: false,
        tac_debug: false,
        file_name: "".to_string(),
    };

    for arg in args.iter() {
        match arg.as_str() {
            "-pd" => flags.parser_debug = true,
            "-sd" => flags.semantic_analyzer_debug = true,
            "-td" => flags.tac_debug = true,
            _=> { let name = arg.trim_matches('"'); flags.file_name = name.to_string()}
        }
    }

    if !flags.file_name.ends_with(".MC") {
        println!("File is not a monkey_c program");
        return None;
    }

    Some(flags)
}
