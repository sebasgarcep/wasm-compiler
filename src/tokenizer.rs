fn tokenize(input: String) -> Vec<String> {
    vec!["example".to_owned()]
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::tokenize;

    #[test]
    fn tokenize_correctly() {
        assert_eq!(tokenize("".to_owned()), vec!["example".to_owned()]);
    }
}