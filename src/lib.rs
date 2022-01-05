use std::{iter::Peekable, str::Chars};

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
    }

    pub fn value(&mut self, it: &mut Peekable<Chars>) {
        while let Some(&c) = it.peek() {
            match c {
                '0'..='9' => self.number(it),
                '\t' | ' ' | '\n' | '\r' => {
                    it.next(); // skip whitespace
                }
                ':' | ',' | '[' | ']' | '{' | '}' => self.single_char(it),
                't' | 'f' | 'n' => self.alpha_literal(it),
                '"' => self.string(it),
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

    // true, false
    fn alpha_literal(&mut self, it: &mut Peekable<Chars>) {
        let mut str = String::new();
        while let Some(&c) = it.peek() {
            match c {
                'a'..='z' => str.push(it.next().unwrap()),
                _ => break,
            }
        }
        self.tokens.push(str);
    }

    fn single_char(&mut self, it: &mut Peekable<Chars>) {
        self.tokens.push(it.next().unwrap().to_string());
    }

    fn string(&mut self, it: &mut Peekable<Chars>) {
        assert!(it.next() == Some('"'));
        let mut str = String::new();
        let mut prior_was_backslash = false;
        while let Some(c) = it.next() {
            if prior_was_backslash {
                str.push(c);
                prior_was_backslash = false;
                continue;
            }
            match c {
                '\\' => {
                    prior_was_backslash = true;
                }
                '"' => {
                    self.tokens.push(str);
                    return;
                }
                _ => str.push(c),
            }
        }
        panic!("unterminated string")
    }
}

// Challenge: change this to a slice
// CharIndices can help with unicode issues.
pub fn lex(input: String) -> Vec<String> {
    let mut lexer = Lexer::new();
    lexer.lex(input);
    lexer.tokens
}

pub fn lex_slice(input: &str) -> Vec<String> {
    lex(input.to_string())
}

//struct JsonValue
#[derive(PartialEq, Debug)]
pub enum Value {
    Number,
    String,
    Boolean(bool),
    Null,
}

pub fn parse(tokens: Vec<String>) -> Value {
    let t = tokens.first().unwrap().as_str();
    match t {
        "true" => Value::Boolean(true),
        "false" => Value::Boolean(false),
        _ => todo!(""),
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    // https://stackoverflow.com/a/38183903
    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn booleans() {
        assert_eq!(Value::Boolean(true), parse(vec_of_strings!["true"]))
    }
}

#[cfg(test)]
mod lexer_tests {
    use super::*;
    #[test]
    fn just_a_number() {
        let input = "24".to_string();
        let tokens = lex(input);
        assert_eq!(vec!["24".to_string()], tokens);
    }

    // https://stackoverflow.com/a/38183903
    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn lexing() {
        assert_eq!(
            lex_slice("[5  , false , {:}]"),
            vec_of_strings!["[", "5", ",", "false", ",", "{", ":", "}", "]"]
        );
        assert_eq!(
            lex_slice("{\"foo\": \"bar\"}"),
            vec_of_strings!["{", "foo", ":", "bar", "}"]
        )
    }

    #[test]
    #[should_panic]
    fn unterminated_string() {
        lex_slice("\"unclosed");
    }
}
