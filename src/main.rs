use std::env;
use MonkeyC::parser::{parse_text};
use MonkeyC::semantic_analyzer::analyze_semantically;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("You did not pass the file directory");
        return;
    }
    if !args[1].ends_with(".MC") {
        println!("File is not a MonkeyC program");
        return;
    } 

    println!("Parsing...");
    let parsed_text = parse_text(&args[1]);
    match parsed_text {
        Err(msg) => {println!("{}", msg); return;},
        _ => {}
    }

    println!("Analyzing semanticly...");

    analyze_semantically(parsed_text.expect("parsing failed"));
}
