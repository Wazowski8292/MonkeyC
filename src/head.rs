mod import;

use crate::code_generator::generate_assembly;
use crate::error_handler;
use crate::head::import::FileLoader;
use crate::parser::parse_text;
use crate::semantic_analyzer::{analyze_semantically, table::TableTypes};
use crate::three_address_code_gen::generate_three_address_code;
use crate::write_asm::write_asm;

use Vec;

#[derive(Clone)]
pub struct Flags {
    pub parser_debug: bool,
    pub semantic_analyzer_debug: bool,
    pub tac_debug: bool,
    pub full_debug: bool,
}

pub struct Dependencies {
    file: String,
    modules: Vec<String>,
    pending_checks: Vec<String>,
}

pub struct Head {
    loader: FileLoader,
    flags: Flags,
}

impl Head {
    pub fn new(flags: Flags) -> Self {
        Self {
            loader: FileLoader::new(),
            flags,
        }
    }

    pub fn compile(&mut self, file_name: String) {
        self.loader.handle_new_file(file_name);

        while let Some(file) = self.loader.next_unprocessed_file() {
            let Ok((table_type, dependencies)) = self.analyze_file_semanticly(file.clone()) else {
                return;
            };
            for dependency in dependencies.iter() {
                self.loader.handle_new_file(dependency.file.clone())
            }
            self.finish_compilation(file.to_string(), table_type);
        }
    }

    fn analyze_file_semanticly(&self, file_name: String) -> Result<(Vec<TableTypes>, Vec<Dependencies>), ()> {
        let flags = self.flags.clone();

        let parsed_text = parse_text(&file_name.clone(), flags.parser_debug);
        match parsed_text {
            Err(msg) => {
                println!("{}", msg);
                return Err(());
            }
            _ => {}
        }

        let (parsed_text, file_str) = parsed_text.expect("parsing failed");

        error_handler::with_handler(|h| h.set_source(file_str));
        let (type_table, depen, pending_checks) = analyze_semantically(parsed_text, file_name.clone(), flags.semantic_analyzer_debug);

        if Self::print_errors_and_warnings() {
            return Err(());
        }

        
        let mut dependencies = vec![];
        for dependency in depen.iter() {
            dependencies.push( Dependencies {
                file: file_name.clone(),
                modules: dependency.modules.clone(),
                pending_checks: pending_checks.clone() 
            });
        }

        Ok((type_table, dependencies))
    }

    fn finish_compilation(&self, file_name: String, table_type: Vec<TableTypes>) {
        let flags = self.flags.clone();

        let tac = generate_three_address_code(table_type, flags.tac_debug);
        let asm = generate_assembly(tac);
        let asm_path = file_name.replace(".MC", ".asm");
        write_asm(&asm_path, &asm);
    }

    fn print_errors_and_warnings() -> bool {
        if error_handler::with_handler(|h| h.has_warnings()) {
            error_handler::with_handler(|h| h.print_all_warnings());
        }
        if error_handler::with_handler(|h| h.has_errors()) {
            error_handler::with_handler(|h| h.print_all_errors());
            return true;
        }

        false
    }
}
