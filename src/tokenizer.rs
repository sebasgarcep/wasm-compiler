fn tokenize(input: String) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let mut token_characters: Vec<char> = vec![];

    for character in input.chars() {
        if token_characters.len() > 0 && is_token_break(&character) {
            consume_token(&mut result, &mut token_characters);
        }

        if !should_consume(&character) {
            continue;
        }

        token_characters.push(character.clone());

        if is_special_character(&character) {
            consume_token(&mut result, &mut token_characters);
        }
    }

    result
}

fn consume_token(result: &mut Vec<String>, token_characters: &mut Vec<char>) {
    let token: String = token_characters.iter().collect();
    result.push(token);
    token_characters.clear();
}

fn is_token_break(character: &char) -> bool {
    return character.is_whitespace() || is_special_character(&character);
}

fn is_special_character(character: &char) -> bool {
    return match character {
        | '('
        | ')'
        | '{'
        | '}'
        | '['
        | ']'
        | '+'
        | '-'
        | '*'
        | '/'
        | '^'
        | ':'
        | ';'
        | '"'
        => true,
        _ => false,
    }
}

fn should_consume(character: &char) -> bool {
    return !character.is_whitespace();
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::tokenize;
    use std::fs;

    #[test]
    fn tokenize_basic_program_correctly() {
        let basic_program = fs::read_to_string("samples/basic-bear/main.bear").unwrap();
        assert_eq!(tokenize(basic_program.to_owned()), vec!["function", "main", "(", "args", ":", "Array<String>", ")", "{", "print", "(", "\"", "Hello", "World!", "\"", ")", ";", "}", ";"]);
    }
}
