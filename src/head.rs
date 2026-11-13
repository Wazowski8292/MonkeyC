mod import;

use crate::code_generator::generate_assembly;
use crate::error_handler;
use crate::head::import::{FileLoader, FileHeader};
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
        if !self.first_half_of_compilation(file_name) {
            return;
        };
        
        while let Some(file) = self.loader.next_unprocessed_file() {
            if !self.first_half_of_compilation(file) {
                return;
            }
        }

        
        //self.finish_compilation(file.to_string(), header.table_type);
    }

    fn first_half_of_compilation(&mut self, file_name: String) -> bool {
        let Ok(header) = self.analyze_file_semanticly(file_name.clone()) else {
            return false;
        };
            
        self.loader.handle_new_file(header.clone());

        if Self::print_errors_and_warnings() {
            return false;
        }

        true
    }

    fn analyze_file_semanticly(&self, file_name: String) -> Result<FileHeader, ()> {
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
        let (type_table, modules_needed, pending_checks, public_modules) =
            analyze_semantically(parsed_text, file_name.clone(), flags.semantic_analyzer_debug);

        //if Self::print_errors_and_warnings() {
        //    return Err(());
        //}

        let header = FileHeader {
            file_name: file_name.clone(),
            modules_needed,
            public_modules: public_modules.clone(),
            pending_checks,
            table_type: type_table.clone(),
        };

        Ok(header)
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
