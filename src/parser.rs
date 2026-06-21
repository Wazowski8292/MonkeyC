use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Block {
    Line(String),
    Multiple(Vec<String>),
    Colection(Vec<Block>),
}

pub fn parse_text(file_path: &String){
    let file = File::open(file_path).expect("Failed to open the file");
    let reader = BufReader::new(file);

    let mut stack: Vec<Vec<Block>> = vec![vec![]];
    let mut current_multiple_items: Vec<String> = vec![];
    let mut current_line: String = Default::default();

    let mut first_char = false;

    for line in reader.lines().enumerate() {
        let line_content = line.1.expect("Failed to read line");
        first_char = false;

        for letters in line_content.chars() {
            match letters {
                '{' => {
                    add_last_block(&mut first_char, &mut current_line, &mut current_multiple_items, &mut stack);

                    stack.push(vec![]);
                }
                '}' => {
                    add_last_block(&mut first_char, &mut current_line, &mut current_multiple_items, &mut stack);

                    if stack.len() > 1 {
                        let finished = stack.pop().unwrap();
                        stack.last_mut().unwrap().push(Block::Colection(finished));
                    }
                }
                ';' => { current_multiple_items.push(current_line.clone()); current_line.clear() }
                '\t' | ' ' => {
                    if first_char {
                        current_line.push(letters);
                    }
                }
                _ => { current_line.push(letters); first_char = true }
            }
        }

        // Maybe take this out. The dif is if it is mandatory to add ; at th eend or not
        if !current_line.is_empty() {
            current_multiple_items.push(current_line.clone());
            current_line.clear();
        }
    }

    if !current_multiple_items.is_empty() {
        stack.last_mut().unwrap().push(Block::Multiple(current_multiple_items));
    }

    let current_collection = stack.pop().unwrap();
    if !current_collection.is_empty() {
        println!("{:#?}", current_collection);
    }
}

fn add_last_block(first_char: &mut bool, current_line: &mut String, current_multiple_items: &mut Vec<String>, stack: &mut Vec<Vec<Block>>) {
    if !current_line.is_empty() {
        current_multiple_items.push(current_line.clone());
        current_line.clear();
        *first_char = false;
    }
    
    if !current_multiple_items.is_empty() { 
        stack.last_mut().unwrap().push(Block::Multiple(current_multiple_items.clone()));
        current_multiple_items.clear();
    }   
}