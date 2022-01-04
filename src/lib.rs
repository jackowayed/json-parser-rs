struct Lexer {
    input: Box<String>,
    cursor: u32,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: Box::new(input),
            cursor: 0,
        }
    }

    pub fn lex(&self) -> Vec<String> {
        let vec;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
