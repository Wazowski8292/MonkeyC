use crate::report;
use crate::semantic_analyzer::TableTypes;
use crate::warn;
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

struct FileLoader {
    token_cache: Vec<FileHeader>,
}

impl FileLoader {
    pub fn new() -> Self {
        Self {
            token_cache: vec![],
        }
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
        if !Self::check_file_exists(file_name.clone()) {
            report!(
                file_name.clone(),
                crate::error_handler::error_codes::ErrorCodes::FileNotFound,
                0,
                0,
                "Tried including '{}' which doesn't exist",
                file_name.clone()
            );
        }

        if self.check_file_already_handled(file_name.clone()) {
            warn!(
                file_name.clone(),
                crate::error_handler::warning_codes::WarningCodes::IncludedFileMultipleTimes,
                0,
                0,
                "Included {} multiple times",
                file_name.clone()
            );
        }

        self.load_new_file(file_name);
    }
}
