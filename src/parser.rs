use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Block {
    Line(String),
    Multiple(Vec<String>),
}

pub fn parse_text(file_path: &String){
    let file = File::open(file_path).expect("Failed to open the file");
        
    let reader = BufReader::new(file);

    let mut result: Vec<Vec<Block>> = vec![];

    let mut current_multiple_items: Vec<String> = vec![];
    let mut current_collection: Vec<Block> = vec![];

    for line in reader.lines().enumerate() {
        let line_content = line.1.expect("Failed to read line");
        let mut current_line: String = Default::default();
        let mut first_char = false;

        for letters in line_content.chars() {
            match letters {
                '{' => {
                    if !current_multiple_items.is_empty() { 
                        current_collection.push(Block::Multiple(current_multiple_items.clone()));
                        current_multiple_items.clear();
                    }
                }
                '}' => {
                    if !current_multiple_items.is_empty() { 
                        current_collection.push(Block::Multiple(current_multiple_items.clone()));
                        current_multiple_items.clear();
                    }
                }
                ';' => { current_multiple_items.push(current_line.clone()); current_line.clear() }
                '\t' => {}
                ' ' => {
                    if first_char {
                        current_line.push(letters);
                    }
                }
                _ => { current_line.push(letters); first_char = true }
            }
        }

        if !current_line.is_empty() {
            current_multiple_items.push(current_line);
        }
    }

    if !current_multiple_items.is_empty() {
        current_collection.push(Block::Multiple(current_multiple_items));
    }

    if !current_collection.is_empty() {
        result.push(current_collection);
    }

    println!("{:?}", result);
}
