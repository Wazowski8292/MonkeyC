use crate::variable_types::InlineAssembly;
use crate::parser::Block;

fn push_word(asm: &mut InlineAssembly, word: &str) {
    match asm.table.last_mut() {
        Some(last) => {
            if !last.is_empty() {
                last.push(' ');
            }
            last.push_str(word);
        }
        None => asm.table.push(word.to_string()),
    }
}

pub(super) fn add_inline_asm(asm: &mut InlineAssembly, block: Block) {
    match block {
        Block::Word(word) => push_word(asm, &word.word),
        Block::Line(line) => {
            asm.table.push(String::new());
            for word in line {
                push_word(asm, &word.word)
            }
        }
        Block::Multiple(multiple) => {
            for line in multiple {
                asm.table.push(String::new());
                for word in line {
                    push_word(asm, &word.word);
                }
            }
        }
        Block::Collection(collection) => {
            for block in collection {
                add_inline_asm(asm, block);
            }
        }
        _ => {}
    }
}


