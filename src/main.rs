use monkey_c::code_generator::generate_assembly;
use monkey_c::parser::parse_text;
use monkey_c::semantic_analyzer::analyze_semantically;
use monkey_c::three_address_code_gen::generate_three_address_code;
use monkey_c::write_asm::write_asm;
use std::env;

struct Flags {
    parser_debug: bool,
    semantic_analyzer_debug: bool,
    tac_debug: bool,
    simple_debug: bool,
    full_debug: bool,
    file_name: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let Some(flags) = parse_parameters(args) else { return };

    if flags.simple_debug {
        println!("Parsing...");
    }
    let parsed_text = parse_text(&flags.file_name, flags.parser_debug);
    match parsed_text {
        Err(msg) => {
            println!("{}", msg);
            return;
        }
        _ => {}
    }

    if flags.simple_debug {
        println!("Analyzing semanticly...");
    }
    let (parsed_text, file_str) = parsed_text.expect("parsing failed");
    let type_table = analyze_semantically(parsed_text, file_str, flags.file_name.clone(), flags.semantic_analyzer_debug);
    match type_table {
        Err(len) => {
            println!(
                "There {} {} compiler error{}. Please fix the compiler error{} before compiling.",
                { if len == 1 { "is" } else { "are" } },
                len,
                { if len == 1 { "" } else { "s" } },
                { if len == 1 { "" } else { "s" } }
            );
            return;
        }
        _ => {}
    }

    if flags.simple_debug {
        println!("Translating into a three address code...");
    }
    let tac = generate_three_address_code(type_table.expect("Compiler errors"), flags.tac_debug);

    if flags.simple_debug {
        println!("Translating into assembly...");
    }
    let asm = generate_assembly(tac);

    let asm_path = flags.file_name.replace(".MC", ".asm");

    if flags.simple_debug {
        println!("Writing assembly to file...");
    }
    write_asm(&asm_path, &asm, flags.simple_debug);
}

fn parse_parameters(args: Vec<String>) -> Option<Flags> {
    let mut flags = Flags {
        simple_debug: false,
        parser_debug: false,
        semantic_analyzer_debug: false,
        tac_debug: false,
        full_debug: false,
        file_name: "".to_string(),
    };

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                return None;
            }
            "-pd" => {
                if !flags.parser_debug {
                    flags.parser_debug = true
                } else {
                    eprintln!("-pd tag has been repeated");
                    return None;
                }
            }
            "-sd" => {
                if !flags.semantic_analyzer_debug {
                    flags.semantic_analyzer_debug = true
                } else {
                    eprintln!("-sd tag has been repeated");
                    return None;
                }
            }
            "-td" => {
                if !flags.tac_debug {
                    flags.tac_debug = true
                } else {
                    eprintln!("-td tag has been repeated");
                    return None;
                }
            }
            "-v" => {
                if !flags.simple_debug {
                    flags.simple_debug = true
                } else {
                    eprintln!("-v tag has been repeated");
                    return None;
                }
            }
            "-dd" => {
                if !flags.full_debug {
                    flags.full_debug = true;
                    flags.simple_debug = true;
                    flags.tac_debug = true;
                    flags.semantic_analyzer_debug = true;
                    flags.parser_debug = true;
                } else {
                    eprint!("-dd tag has been repeated");
                    return None;
                }
            }

            _ if arg.ends_with(".MC") => {
                let name = arg.trim_matches('"');
                flags.file_name = name.to_string();
            }
            _ => {
                eprintln!(
                    "[Error]: {} isn't a real flag, if you need help please add the -h or -help flag to see all of the posible flags",
                    arg
                );
                return None;
            }
        }
    }

    if !flags.file_name.len() < 3 {
        eprintln!("You have not passed a file");
        return None;
    }

    Some(flags)
}

fn print_help() {
    println!(
        "
        
        MonkeyC is a simple C style compiler (in the middle of c and c++, hopefully, and inspired in Rust)
        
        This compilers comes with diferent tags to help debug errors in code and or in the compiler.
        <This tags are mainly usefull to compiler developers>

        The tags are:

        -h or --help: This print out this text.
        
        -dd: This will do every debug print with 1 flag.
        -pd: This shows the output of the parser ones it has all ready proces the code.
        -sd: This shows the output of the semantic debuger and all of the vatiables when the process is finished.
        -td: This shows the output of the three address code generator.

        -v: This does some mini prints which tells you where in the proces of comiping you are currently on and some extra information.

        If you want some extra information or just want to look at the documentation you can read 
        the readme and the docs folder or go to this link: https://github.com/Wazowski8292/MonkeyC
        
        "
    );
}
