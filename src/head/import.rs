use crate::report;
use crate::semantic_analyzer::TableTypes;
use std::fs;

struct FileHeader {
    file_name: String,
    public_table: Vec<TableTypes>,
}

impl FileHeader {
    pub fn new(file_name: String) -> Self {
        Self {
            file_name,
            public_table: vec![],
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

    fn load_new_file(&mut self, file_name: String) {
        let header = FileHeader::new(file_name);

        self.token_cache.push(header);
    }

    pub fn handle_new_file(&mut self, file_name: String) {
        if !Self::check_file_type(file_name.clone()) {
            report!(
                file_name.clone(),
                crate::error_handler::error_codes::ErrorCodes::FileNotMonkeyC,
                0,
                0,
                "Tried including '{}' which is not a Monkey C file",
                file_name.clone()
            );
            return;
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
            return;
        }

        if self.check_file_already_handled(file_name.clone()) {
            return;
        }

        self.load_new_file(file_name);
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
