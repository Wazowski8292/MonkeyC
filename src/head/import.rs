use crate::report;
use crate::semantic_analyzer::TableTypes;
use crate::variable_types::Include;
use std::fs;

#[derive(Clone)]
pub struct FileHeader {
    pub file_name: String,
    pub public_modules: Vec<TableTypes>,
    pub modules_needed: Vec<Include>,
    pub pending_checks: Vec<String>,
    pub table_type: Vec<TableTypes>,
}

impl FileHeader {
    pub fn new(file_name: String) -> Self {
        Self {
            file_name,
            public_modules: vec![],
            modules_needed: vec![],
            pending_checks: vec![],
            table_type: vec![],
        }
    }
}

pub struct FileLoader {
    token_cache: Vec<FileHeader>,
    index: usize,
}

impl FileLoader {
    pub fn new() -> Self {
        Self {
            token_cache: vec![],
            index: 0,
        }
    }

    fn check_file_type(file_name: String) -> bool {
        file_name.ends_with(".MC")
    }

    fn check_file_exists(file_name: String) -> bool {
        fs::exists(file_name).is_ok()
    }

    fn check_file_already_handled(&self, file_name: String) -> bool {
        self.token_cache.iter().any(|header| header.file_name == file_name)
    }

    fn load_new_file(&mut self, dependency: Include) {
        let header = FileHeader::new(dependency.file);

        self.token_cache.push(header);
    }

    pub fn handle_new_file(&mut self, header: FileHeader) {
        for dependency in header.modules_needed {
            if !self.load_dependency(dependency.clone()) {
                println!("a");
                continue;
            }
        }
    }

    pub fn load_dependency(&mut self, dependency: Include) -> bool {
        let file_name = dependency.file.clone();
        if !Self::check_file_type(file_name.clone()) {
            report!(
                file_name.clone(),
                crate::error_handler::error_codes::ErrorCodes::FileNotMonkeyC,
                0,
                0,
                "Tried including '{}' which is not a Monkey C file",
                file_name.clone()
            );

            return false;
        }

        if !Self::check_file_exists(file_name.clone()) {
            report!(
                file_name.clone(),
                crate::error_handler::error_codes::ErrorCodes::FileNotFound,
                0,
                0,
                "Tried including '{}' which doesn't exist",
                file_name.clone()
            );
            return false;
        }

        if self.check_file_already_handled(file_name.clone()) {
            return false;
        }

        self.load_new_file(dependency.clone());

        true
    }

    pub fn next_unprocessed_file(&mut self) -> Option<String> {
        if self.index < self.token_cache.len() {
            let file_name = self.token_cache[self.index].file_name.clone();
            self.index += 1;
            Some(file_name)
        } else {
            None
        }
    }
}
