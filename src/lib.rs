use std::{iter::Peekable, str::Chars};

/*struct Lexer {
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
        vec
    }
}
*/

struct Lexer {
    tokens: Vec<String>,
}

impl Lexer {
    pub fn new() -> Self {
        Self { tokens: vec![] }
    }
    pub fn lex(&mut self, input: String) {
        let mut it = input.chars().peekable();
        self.value(&mut it);
        //let tokens = self.tokens;
        //self.tokens = vec![];
    }

    pub fn value(&mut self, it: &mut Peekable<Chars>) {
        while let Some(&c) = it.peek() {
            match c {
                '0'..='9' => self.number(it),
                _ => todo!("more matches coming"),
            }
        }
    }

    fn number(&mut self, it: &mut Peekable<Chars>) {
        let mut num_str = String::new();
        while let Some(&c) = it.peek() {
            match c {
                '0'..='9' => num_str.push(it.next().expect("this is a bug")),
                _ => break,
            }
        }
        self.tokens.push(num_str);
    }
}

// Challenge: change this to a slice
// CharIndices can help with unicode issues.
pub fn lex(input: String) -> Vec<String> {
    let mut lexer = Lexer::new();
    lexer.lex(input);
    lexer.tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pass_in_string() {
        let input = "24".to_string();
        let tokens = lex(input);
        assert_eq!(vec!["24".to_string()], tokens);
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
