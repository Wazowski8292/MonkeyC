use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct Word {
    pub word: String,
    pub line: Option<usize>,
    pub char_num: Option<usize>,
}

impl Word {
    pub fn new() -> Self {
        Self {
            word: String::default(),
            line: None,
            char_num: None,
        }
    }

    fn clear(&mut self) {
        self.word.clear();
        self.line = None;
        self.char_num = None;
    }
}

#[derive(Debug, Clone)]
pub enum Block {
    Word(Word),
    Line(Vec<Word>),
    Multiple(Vec<Vec<Word>>),
    Collection(Vec<Block>),
    Parameter(Vec<Block>),
}

#[derive(PartialEq)]
enum SkipType {
    Line,
    Block,
    None,
}

struct Parser {
    file_name: String,
    file_source: Vec<String>,
    stack: Vec<Vec<Block>>,
    current_multiple_items: Vec<Vec<Word>>,
    current_line: Vec<Word>,
    current_word: Word,
    skip_block: SkipType,
    skip_pending: bool,
}

impl Parser {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_name: file_path.to_string(),
            file_source: vec![],
            stack: vec![vec![]],
            current_multiple_items: vec![],
            current_line: vec![],
            current_word: Word::new(),
            skip_block: SkipType::None,
            skip_pending: false,
        }
    }

    fn start_parsing(&mut self) -> Result<(), String> {
        let file = File::open(&self.file_name).map_err(|e| format!("Failed to open '{}': {}", self.file_name, e))?;
        let reader = BufReader::new(file);

        for (index, line) in reader.lines().enumerate() {
            let line = line.map_err(|e| format!("Failed to read line {}: {}", index + 1, e))?;
            self.parse_line(&line, index + 1);
        }
        Ok(())
    }

    fn parse_line(&mut self, line: &str, line_position: usize) {
        self.file_source.push(line.to_string());
        if self.skip_block == SkipType::Line {
            self.skip_block = SkipType::None;
        }
        self.skip_pending = false;

        for (index, letter) in line.chars().enumerate() {
            self.parse_char(letter, line_position, index + 1);

            if self.skip_block == SkipType::Line {
                break;
            }
        }

        self.finish_word();
        self.finish_line();
    }

    fn parse_char(&mut self, letter: char, line_position: usize, char_position: usize) {
        if self.skip_block == SkipType::Block {
            self.parse_skipped_char(letter);
            return;
        }

        let skip_pending = self.skip_pending;

        match letter {
            '(' if skip_pending => self.start_skip_block(),
            '{' | '(' => self.open_block(),
            '}' | ')' => self.close_block(letter == ')'),
            ';' => self.end_statement(),
            '#' => self.handle_skip(skip_pending),
            ' ' | '\t' => self.finish_word(),
            _ => self.push_to_word(letter, line_position, char_position),
        }
    }

    fn open_block(&mut self) {
        self.flush_pending_items();
        self.stack.push(vec![]);
    }

    fn close_block(&mut self, is_parameter: bool) {
        self.flush_pending_items();

        if self.stack.len() > 1 {
            let finished = self.stack.pop().unwrap();
            let block = if is_parameter { Block::Parameter(finished) } else { Block::Collection(finished) };
            self.stack.last_mut().unwrap().push(block);
        }
    }

    fn end_statement(&mut self) {
        self.finish_word();
        self.finish_line();
    }

    fn handle_skip(&mut self, was_pending: bool) {
        if was_pending {
            self.skip_block = SkipType::Line;
        } else {
            self.skip_pending = true;
        }
    }

    fn start_skip_block(&mut self) {
        self.finish_word();
        self.skip_block = SkipType::Block;
    }

    fn parse_skipped_char(&mut self, letter: char) {
        match letter {
            ')' => self.skip_pending = true,
            '#' if self.skip_pending => {
                self.skip_block = SkipType::None;
                self.skip_pending = false;
            }
            _ => self.skip_pending = false,
        }
    }

    fn push_to_word(&mut self, letter: char, line_position: usize, char_position: usize) {
        self.current_word.word.push(letter);
        self.current_word.line.get_or_insert(line_position);
        self.current_word.char_num.get_or_insert(char_position);
    }

    fn finish_word(&mut self) {
        if !self.current_word.word.is_empty() {
            self.current_line.push(self.current_word.clone());
            self.current_word.clear();
        }
    }

    fn finish_line(&mut self) {
        if !self.current_line.is_empty() {
            self.current_multiple_items.push(std::mem::take(&mut self.current_line));
        }
    }

    fn finish_multiple(&mut self) {
        if !self.current_multiple_items.is_empty() {
            let items = std::mem::take(&mut self.current_multiple_items);
            self.stack.last_mut().unwrap().push(Block::Multiple(items));
        }
    }

    fn flush_pending_items(&mut self) {
        self.finish_word();
        self.finish_line();
        self.finish_multiple();
    }

    fn finish(mut self, debug: bool) -> Result<(Vec<Block>, Vec<String>), String> {
        if !self.current_multiple_items.is_empty() {
            return Err("You have to finish the last line".to_string());
        }
        if self.stack.len() > 1 {
            return Err("A block was opened but never closed".to_string());
        }
        if self.skip_block == SkipType::Block {
            return Err("A skip block was opened but never closed".to_string());
        }

        let output = self.stack.pop().unwrap();
        if debug && !output.is_empty() {
            println!("{:#?}", output);
        }
        Ok((output, self.file_source))
    }
}

pub fn parse_text(file_path: &str, debug: bool) -> Result<(Vec<Block>, Vec<String>), String> {
    let mut parser = Parser::new(file_path);
    parser.start_parsing()?;
    parser.finish(debug)
}
