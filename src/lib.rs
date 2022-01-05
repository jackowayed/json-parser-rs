use std::{collections::HashMap, iter::Peekable, str::Chars};

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
    String(String),
    Boolean(bool),
    Null,
    Object(HashMap<String, Value>),
}

pub fn parse(tokens: Vec<String>) -> Value {
    //let t = tokens.first().unwrap().as_str();
    let mut it = tokens.into_iter();
    value(it)
}

pub fn value(mut it: std::vec::IntoIter<String>) -> Value {
    let t = it.next().unwrap();
    match t.as_str() {
        "true" => Value::Boolean(true),
        "false" => Value::Boolean(false),
        "{" => object(it),
        s if t.starts_with("\"") => Value::String(string(s)),
        _ => todo!(""),
    }
}

fn object(mut it: std::vec::IntoIter<String>) -> Value {
    let mut map = HashMap::new();
    let t = it.next().unwrap();
    match t.as_str() {
        "}" => return Value::Object(map),
        "," => (), // todo: currently allowing comma before first pair
        _ => {
            let (key, value) = pair(it);
            map.insert(key, value);
        }
    }
    return Value::Object(map);
    // string
    // :
    // value
    // [, string: value]*
    // }
    todo!()
}

fn pair(mut it: std::vec::IntoIter<String>) -> (String, Value) {
    let key = string(it.next().unwrap().as_str());
    assert!(it.next().unwrap().as_str() == ":");
    let value = value(it);
    return (key, value);
}

fn string(t: &str) -> String {
    dbg!(t);
    t[1..t.len() - 1].to_string()
}

#[cfg(test)]
mod parser_tests {
    use std::hash::Hash;

    use super::*;

    // https://stackoverflow.com/a/38183903
    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn booleans() {
        assert_eq!(Value::Boolean(true), parse(vec_of_strings!["true"]))
    }

    #[test]
    fn string() {
        assert_eq!(
            Value::String("foo bar".to_string()),
            parse(vec!["\"foo bar\"".to_string()])
        )
    }

    #[test]
    fn object() {
        let empty_map = HashMap::new();

        let empty_obj = parse(vec_of_strings!["{", "}"]);
        assert_eq!(empty_obj, Value::Object(empty_map));

        let mut singleton_map = HashMap::new();
        singleton_map.insert("foo".to_string(), Value::String("bar".to_string()));
        assert_eq!(
            Value::Object(singleton_map),
            parse(vec_of_strings!["{", "\"foo\"", ":", "\"bar\"", "}"])
        );
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
