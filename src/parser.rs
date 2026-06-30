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

pub fn parse_text(file_path: &String) -> Result<Vec<Block>, String> {
    let file = File::open(file_path).expect("Failed to open the file");
    let reader = BufReader::new(file);

    let mut stack: Vec<Vec<Block>> = vec![vec![]];
    let mut current_multiple_items: Vec<Vec<Word>> = vec![];
    let mut current_line: Vec<Word> = vec![];
    let mut current_word: Word = Word::new();

    let mut first_char;

    for (num, line) in reader.lines().enumerate() {
        let line_content = line.expect("Failed to read line");
        first_char = false;
        let num = num + 1;
        let mut char_pos = 0;
        let mut skip = false; 

        for letters in line_content.chars() {
            char_pos += 1;

            match letters {
                '{' | '(' => {
                    let error = add_last_block(&mut first_char, &mut current_word, &mut current_line, &mut current_multiple_items, &mut stack, num, char_pos, letters == '(');
                    match error {
                        Err(msg) => {return Err(msg); },
                        _ => {}
                    }

                    stack.push(vec![]);
                    skip = false;
                }
                '}' | ')' => {
                    let error = add_last_block(&mut first_char, &mut current_word, &mut current_line, &mut current_multiple_items, &mut stack, num, char_pos, letters == ')');
                    match error {
                        Err(msg) => {return Err(msg); },
                        _ => {}
                    }

                    if stack.len() > 1 {
                        let finished = stack.pop().unwrap();
                        if letters == ')' {
                            stack.last_mut().unwrap().push(Block::Parameter(finished));
                        } else {
                            stack.last_mut().unwrap().push(Block::Collection(finished));
                        }
                    }
                    skip = false;
                }
                ';' => {
                    add_last_word(&mut current_word, &mut current_line);
                    add_last_line(&mut current_line, &mut current_multiple_items, &mut first_char);

                    first_char = false;
                    skip = false;
                }
                '\t' | ' ' => {
                    if first_char {
                        add_last_word(&mut current_word, &mut current_line);
                    }
                    skip = false;
                }
                '/' => {
                    if skip {
                        break;
                    }
                    skip = true;
                }
                _ => { 
                    current_word.word.push(letters);
                    if !current_word.line.is_some() {
                        current_word.line = Some(num);
                    }
                    if !current_word.char_num.is_some() {
                        current_word.char_num = Some(char_pos);
                    }

                    first_char = true;
                    skip = false;
                }
            }
        }

        add_last_word(&mut current_word, &mut current_line);
        add_last_line(&mut current_line, &mut current_multiple_items, &mut first_char);
    }

    if !current_multiple_items.is_empty() {
        panic!("You have to finish the last line");
    }

    let current_collection = stack.pop().unwrap();
    if !current_collection.is_empty() {
        //println!("{:#?}", current_collection);
    }
    Ok(current_collection)
}



fn add_last_word(current_word: &mut Word, current_line: &mut Vec<Word>) {
    if !current_word.word.is_empty() {
        current_line.push(current_word.clone());
        current_word.clear();
    }
}

fn add_last_line(current_line: &mut Vec<Word>, current_multiple_items: &mut Vec<Vec<Word>>, first_char: &mut bool) {
    if !current_line.is_empty() {
        current_multiple_items.push(current_line.clone());
        current_line.clear();
        *first_char = false;
    }
}

fn add_last_block(first_char: &mut bool, current_word: &mut Word, current_line: &mut Vec<Word>, 
    current_multiple_items: &mut Vec<Vec<Word>>, stack: &mut Vec<Vec<Block>>, line: usize, chars: usize, new_line: bool) -> Result<(), String>{
        
    if !current_word.word.is_empty() {
        if new_line {
            current_line.push(current_word.clone());
            current_word.clear();
            *first_char = false;
        } else {
            return Err(format!("You haven't finished the line. Line: {}; Char: {}", line, chars));
        }
    }
    
    add_last_line(current_line, current_multiple_items, first_char);
    
    if !current_multiple_items.is_empty() { 
        stack.last_mut().unwrap().push(Block::Multiple(current_multiple_items.clone()));
        current_multiple_items.clear();
    }   

    Ok(())
}