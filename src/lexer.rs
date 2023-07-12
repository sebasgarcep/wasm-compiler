use std::vec::IntoIter;

struct Tokenizer {
    result: Vec<String>,
    token_characters: Vec<char>,
}

impl Tokenizer {
    fn tokenize(input: String) -> IntoIter<String> {
        let mut tokenizer = Tokenizer {
            result: vec![],
            token_characters: vec![],
        };
        tokenizer.process(input);
        tokenizer.result.into_iter()
    }

    fn consume_token(&mut self) {
        let token: String = self.token_characters.iter().collect();
        self.result.push(token);
        self.token_characters.clear();
    }
    
    fn is_token_break(character: &char) -> bool {
        return character.is_whitespace() || Tokenizer::is_special_character(&character);
    }
    
    // FIXME: THIS TOKENIZER IS NOT CORRECT FOR THE LANGUAGE
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
            | '='
            => true,
            _ => false,
        }
    }
    
    fn should_consume(character: &char) -> bool {
        return !character.is_whitespace();
    }

    fn process(&mut self, input: String) {
        for character in input.chars() {
            if self.token_characters.len() > 0 && Tokenizer::is_token_break(&character) {
                self.consume_token();
            }

            if !Tokenizer::should_consume(&character) {
                continue;
            }

            self.token_characters.push(character.clone());

            if Tokenizer::is_special_character(&character) {
                self.consume_token();
            }
        }

        if !self.token_characters.is_empty() {
            self.consume_token()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{Tokenizer};
    use std::fs;

    #[test]
    fn tokenize_basic_program_correctly() {
        let basic_program = fs::read_to_string("samples/basic-bear/main.bear").unwrap();
        let tokens = vec![
            "function", "main", "(", "args", ":", "Array<String>", ")", "{",
            "print", "(", "\"", "Hello", "World!", "\"", ")", ";",
            "}", ";",
        ];
        assert_eq!(Tokenizer::tokenize(basic_program.to_owned()).collect::<Vec<_>>(), tokens);
    }
}
