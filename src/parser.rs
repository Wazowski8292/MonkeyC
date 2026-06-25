use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
pub enum Block {
    Word(String),
    Line(Vec<String>),
    Multiple(Vec<Vec<String>>),
    Collection(Vec<Block>),
}

pub fn parse_text(file_path: &String) -> Result<Vec<Block>, String> {
    let file = File::open(file_path).expect("Failed to open the file");
    let reader = BufReader::new(file);

    let mut stack: Vec<Vec<Block>> = vec![vec![]];
    let mut current_multiple_items: Vec<Vec<String>> = vec![];
    let mut current_line: Vec<String> = vec![];
    let mut current_word: String = Default::default();

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
                        stack.last_mut().unwrap().push(Block::Collection(finished));
                    }
                    skip = false;
                }
                ';' => {
                    if !current_word.is_empty() {
                        current_line.push(current_word.clone());
                        current_word.clear();
                    }
                    current_multiple_items.push(current_line.clone());
                    current_line.clear();
                    first_char = false;
                    skip = false;
                }
                '\t' | ' ' => {
                    if first_char {
                        current_line.push(current_word.clone());
                        current_word.clear();
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
                    current_word.push(letters); 
                    first_char = true;
                    skip = false;
                }
            }
        }

        if !current_word.is_empty() {
            current_line.push(current_word.clone());
            current_word.clear();
        }

        if !current_line.is_empty() {
            current_multiple_items.push(current_line.clone());
            current_line.clear();
        }
    }

    if !current_multiple_items.is_empty() {
        panic!("You have to finish the last line");
    }

    let current_collection = stack.pop().unwrap();
    if !current_collection.is_empty() {
        println!("{:#?}", current_collection);
    }
    Ok(current_collection)
}

fn add_last_block(first_char: &mut bool, current_word: &mut String, current_line: &mut Vec<String>, 
    current_multiple_items: &mut Vec<Vec<String>>, stack: &mut Vec<Vec<Block>>, line: usize, chars: usize, new_line: bool) -> Result<(), String>{
        
    if !current_word.is_empty() {
        if new_line {
            current_line.push(current_word.to_string());
            current_word.clear();
            *first_char = false;
        } else {
            return Err(format!("You haven't finished the line. Line: {}; Char: {}", line, chars));
        }
    }
    
    if !current_line.is_empty() {
        current_multiple_items.push(current_line.clone());
        current_line.clear();
        *first_char = false;
    }
    
    if !current_multiple_items.is_empty() { 
        stack.last_mut().unwrap().push(Block::Multiple(current_multiple_items.clone()));
        current_multiple_items.clear();
    }   

    Ok(())
}