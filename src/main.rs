use monkey_c::head::{Flags, Head};
use std::env; // Todo: I dont like having flags inside there but it is
// practical

fn main() {
    let args: Vec<String> = env::args().collect();

    let Some((flags, file_name)) = parse_parameters(args) else { return };
    let mut head = Head::new(flags);
    head.compile(file_name);
}

fn parse_parameters(args: Vec<String>) -> Option<(Flags, String)> {
    let mut flags = Flags {
        parser_debug: false,
        semantic_analyzer_debug: false,
        tac_debug: false,
        full_debug: false,
    };

    let mut file_name: String = "".to_string();
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
            "-dd" | "-v" => {
                if !flags.full_debug {
                    flags.full_debug = true;
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
                file_name = name.to_string();
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

    if file_name.len() < 3 {
        eprintln!("You have not passed a file");
        return None;
    }

    Some((flags, file_name))
}

fn print_help() {
    println!(
        "
        
        MonkeyC is a simple C style compiler (in the middle of c and c++, hopefully, and inspired in Rust)
        
        This compilers comes with diferent tags to help debug errors in code and or in the compiler.
        <This tags are mainly usefull to compiler developers>

        The tags are:

        -h or --help: This print out this text.
        
        ## Compiler development

        -dd or -v: This will do every debug print with 1 flag.
        -pd: This shows the output of the parser ones it has all ready proces the code.
        -sd: This shows the output of the semantic debuger and all of the vatiables when the process is finished.
        -td: This shows the output of the three address code generator.

        ## Compiler 

        : This warns you of all the posible errors there maight be.
        : This doesn't let you compile with warnings.

        If you want some extra information or just want to look at the documentation you can read 
        the readme and the docs folder or go to this link: https://github.com/Wazowski8292/MonkeyC
        
        "
    );
}
