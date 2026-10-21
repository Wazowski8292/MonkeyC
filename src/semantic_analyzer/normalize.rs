use crate::parser::Word;

const INCREMENT_OPERATORS: [&str; 2] = ["++", "--"];
const PATH_SEPARATOR: &str = "::";

pub(super) fn normalize_line(line: &[Word]) -> Option<Vec<Word>> {
    let mut normalized = Vec::new();
    let mut index = 0;

    while index < line.len() {
        let word = line[index].clone();

        if word.word.is_empty() {
            return None;
        }

        if !expand_limiters(line, &mut word.clone(), &mut index) {
            continue;
        }

        expand_signs(&mut normalized, word);
    }

    Some(normalized)
}

fn expand_limiters(line: &[Word], word: &mut Word, index: &mut usize) -> bool {
    if word.word.len() > 1 && word.word.ends_with(',') && !word.word.starts_with('"') {
        word.word.pop();
    }

    if word.word == "," {
        *index += 1;
        return false;
    }

    if *index + 3 < line.len() && line[*index + 1].word == "[" && line[*index + 3].word == "]" {
        word.word = format!("{}[{}]", word.word, line[*index + 2].word);
        *index += 4;
    } else if *index + 1 < line.len() && line[*index + 1].word.starts_with('[') && line[*index + 1].word.ends_with(']') {
        word.word = format!("{}{}", word.word, line[*index + 1].word);
        *index += 2;
    } else {
        *index += 1;
    }

    true
}

fn expand_signs(normalized: &mut Vec<Word>, word: Word) {
    let mut was_modified = false;

    for operator in INCREMENT_OPERATORS {
        if word.word.len() > 2 && word.word.ends_with(operator) {
            let mut variable = word.clone();
            variable.word = word.word[..word.word.len() - 2].to_string();

            let mut operator_word = word.clone();
            operator_word.word = operator.to_string();

            normalized.push(variable);
            normalized.push(operator_word);

            was_modified = true;
        }
    }

    if word.word.contains(PATH_SEPARATOR) {
        split_by(normalized, &word, PATH_SEPARATOR);
        was_modified = true;
    }

    if !was_modified {
        normalized.push(word);
    }
}

fn split_by(normalized: &mut Vec<Word>, word: &Word, separator: &str) {
    let parts: Vec<&str> = word.word.split(separator).collect();
    let last = parts.len() - 1;
    let start = word.char_num.unwrap_or(0);
    let mut offset = 0;

    for (i, part) in parts.iter().enumerate() {
        normalized.push(Word {
            word: part.to_string(),
            line: word.line,
            char_num: Some(start + offset),
        });
        offset += part.len();

        if i < last {
            normalized.push(Word {
                word: separator.to_string(),
                line: word.line,
                char_num: Some(start + offset),
            });
            offset += separator.len();
        }
    }
}
