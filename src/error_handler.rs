use std::cell::RefCell;

#[derive(Clone)]
pub struct Diagnostic {
    pub file_name: String,
    pub line: usize,
    pub col: usize,
    pub msg: String,
}

pub struct ErrorHandler {
    source: Vec<String>,
    warnings: Vec<Diagnostic>,
    errors: Vec<Diagnostic>,
}

impl ErrorHandler {
    pub fn new() -> Self {
        Self {
            source: vec![],
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn set_source(&mut self, src: Vec<String>) {
        self.source = src;
    }

    pub fn add_warning(&mut self, line: usize, col: usize, msg: String, file_name: String) {
        self.warnings.push(Diagnostic {
            file_name,
            line,
            col,
            msg,
        });
    }

    pub fn add_error(&mut self, line: usize, col: usize, msg: String, file_name: String) {
        self.errors.push(Diagnostic {
            file_name,
            line,
            col,
            msg,
        });
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn print_diagnostics(&self, diagnostics: Vec<Diagnostic>, diagnostic_type: String) {
        for diagnostic in diagnostics.iter() {
            println!("\n[{}]: {}", diagnostic_type, diagnostic.msg);
            println!("--> {} line:{}, char pos :{}", diagnostic.file_name, diagnostic.line.to_string(), diagnostic.col.to_string());
            println!();

            self.general_info_print(diagnostic.clone());

            println!();
        }
    }

    fn general_info_print(&self, diagnostic: Diagnostic) {
        if diagnostic.line == 0 {
            // No source location — nothing to display.
            return;
        }

        let error_line = (diagnostic.line - 1) as usize;
        let total_lines = self.source.len();

        let mut start = if error_line > 2 { error_line - 2 } else { 1 };
        let mut end = start + 4;

        if end > total_lines {
            end = total_lines;
            start = if end > 4 { end - 4 } else { 1 };
        }

        let width = (end + 1).to_string().len();

        for i in start..=end {
            let line_content = &self.source[i - 1];
            println!("{:>width$} | {}", i + 1, line_content, width = width);

            if i == error_line {
                let (word_start, word_len) = Self::word_span_at(line_content, diagnostic.col as usize);
                let gutter_len = width + 3;
                let leading_spaces = " ".repeat(gutter_len + word_start.saturating_sub(1));
                let squiggles = "~".repeat(word_len.max(1));
                println!("{}{}", leading_spaces, squiggles);
            }
        }
    }

    fn word_span_at(line: &str, pos: usize) -> (usize, usize) {
        let chars: Vec<char> = line.chars().collect();
        if chars.is_empty() {
            return (pos, 1);
        }

        let idx = pos.saturating_sub(1).min(chars.len() - 1);
        let is_word_char = |c: char| c.is_alphanumeric() || c == '_';

        let mut start = idx;
        while start > 0 && is_word_char(chars[start - 1]) {
            start -= 1;
        }

        let mut end = idx;
        while end < chars.len() && is_word_char(chars[end]) {
            end += 1;
        }

        if end == start {
            end = start + 1;
        }

        (start + 1, end - start)
    }

    pub fn print_all_warnings(&self) {
        self.print_diagnostics(self.warnings.clone(), "Warning".to_string());
    }

    pub fn print_all_errors(&self) {
        let len = self.errors.len();
        self.print_diagnostics(self.errors.clone(), "Warning".to_string());

        println!(
            "There {} {} compiler error{}. Please fix the compiler error{} before compiling.",
            { if len == 1 { "is" } else { "are" } },
            len,
            { if len == 1 { "" } else { "s" } },
            { if len == 1 { "" } else { "s" } }
        );
    }
}

thread_local! {
    static HANDLER: RefCell<ErrorHandler> = RefCell::new(ErrorHandler::new());
}

pub fn with_handler<R>(f: impl FnOnce(&mut ErrorHandler) -> R) -> R {
    HANDLER.with(|h| f(&mut h.borrow_mut()))
}

#[macro_export]
macro_rules! report {
    ($file_name:expr, $line:expr, $col:expr, $($arg:tt)*) => {
        $crate::error_handler::with_handler(|h| {
            h.add_error($line as usize, $col as usize, format!($($arg)*), $file_name)
        })
    };
}

#[macro_export]
macro_rules! warn {
    ($file_name:expr, $line:expr, $col:expr, $($arg:tt)*) => {
        $crate::error_handler::with_handler(|h| {
            h.add_warning($line as usize, $col as usize, format!($($arg)*), $file_name)
        })
    };
}
