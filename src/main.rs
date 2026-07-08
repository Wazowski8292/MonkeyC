use std::env;
use monkey_c::parser::parse_text;
use monkey_c::semantic_analyzer::analyze_semantically;
use monkey_c::three_address_code_gen::generate_three_address_code;
use monkey_c::code_generator::generate_assembly;

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
    generate_assembly(tac);
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
