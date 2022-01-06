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
        // TODO: this function currently drops the quotes,
        // but the parser assumes there will be quotes.
        // Need to change one side or the other before integration.
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

#[derive(PartialEq, Debug)]
pub enum Value {
    Number,
    String(String),
    Boolean(bool),
    Null,
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
}

pub fn parse(tokens: Vec<String>) -> Value {
    //let t = tokens.first().unwrap().as_str();
    let mut it = tokens.into_iter().peekable();
    value(&mut it)
}

pub fn value(it: &mut Peekable<std::vec::IntoIter<String>>) -> Value {
    let t = it.next().unwrap();
    match t.as_str() {
        "true" => Value::Boolean(true),
        "false" => Value::Boolean(false),
        "{" => object(it),
        "[" => array(it),
        s if t.starts_with("\"") => Value::String(string(s)),
        s => {
            dbg!(s);
            todo!("");
        }
    }
}

fn array(it: &mut Peekable<std::vec::IntoIter<String>>) -> Value {
    let mut arr = Vec::new();
    loop {
        let tok = it.peek().unwrap();
        match tok.as_str() {
            "]" => {
                it.next();
                return Value::Array(arr);
            }
            "," => {
                it.next();
            } // todo do comma assertions
            _ => {
                arr.push(value(it));
            }
        }
    }
}

fn object(mut it: &mut Peekable<std::vec::IntoIter<String>>) -> Value {
    let mut map = HashMap::new();
    #[derive(PartialEq, Debug)]
    enum WhatsNext {
        PairOrEnd, // start
        CommaOrEnd,
        Pair,
    }
    let mut state = WhatsNext::PairOrEnd;
    loop {
        let tok = it.next().unwrap();
        match tok.as_str() {
            "}" => {
                assert!(state != WhatsNext::Pair, "Can't end after comma");
                return Value::Object(map);
            }
            "," => {
                assert!(
                    state == WhatsNext::CommaOrEnd,
                    "Comma where it shouldn't be."
                );
                state = WhatsNext::Pair;
            }
            _ => {
                assert!(state != WhatsNext::CommaOrEnd, "Need comma between pairs");
                state = WhatsNext::CommaOrEnd;
                // possible fix use singleton iterator to put t back via chaining.
                let key = string(&tok);
                assert!(it.next().unwrap().as_str() == ":");
                let value = value(&mut it);
                map.insert(key, value);
            }
        }
    }
    //return Value::Object(map);
}

fn string(t: &str) -> String {
    dbg!(t);
    t[1..t.len() - 1].to_string()
}

// https://stackoverflow.com/a/38183903
#[allow(unused_macros)]
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[cfg(test)]
mod parser_tests {
    use super::*;

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
    fn array() {
        assert_eq!(
            Value::Array(vec![Value::Boolean(true), Value::Boolean(false)]),
            parse(vec_of_strings!["[", "true", "false", "]"])
        );
    }

    #[test]
    fn object() {
        let empty_map = HashMap::new();
        let empty_object_value = Value::Object(empty_map);

        let empty_obj = parse(vec_of_strings!["{", "}"]);
        assert_eq!(empty_obj, empty_object_value);

        let mut singleton_map = HashMap::new();
        singleton_map.insert("foo".to_string(), Value::String("bar".to_string()));
        let singleton_object = Value::Object(singleton_map);
        assert_eq!(
            singleton_object,
            // {"foo": "bar"}
            parse(vec_of_strings!["{", "\"foo\"", ":", "\"bar\"", "}"])
        );

        let mut doubleton_map = HashMap::new();
        doubleton_map.insert("foo".to_string(), Value::String("bar".to_string()));
        doubleton_map.insert("baz".to_string(), Value::Boolean(false));
        assert_eq!(
            Value::Object(doubleton_map),
            // {"foo": "bar"}
            parse(vec_of_strings![
                "{", "\"foo\"", ":", "\"bar\"", ",", "\"baz\"", ":", "false", "}"
            ])
        );

        let mut nested_map = HashMap::new();
        nested_map.insert("outer".to_string(), empty_object_value);
        assert_eq!(
            Value::Object(nested_map),
            parse(vec_of_strings!["{", "\"outer\"", ":", "{", "}", "}"])
        );

        let mut nested_singleton_map = HashMap::new();
        nested_singleton_map.insert("outer".to_string(), singleton_object);
        assert_eq!(
            Value::Object(nested_singleton_map),
            parse(vec_of_strings![
                "{",
                "\"outer\"",
                ":",
                "{",
                "\"foo\"",
                ":",
                "\"bar\"",
                "}",
                "}"
            ])
        );
    }

    #[test]
    #[should_panic]
    fn missing_comma() {
        parse(vec_of_strings![
            "{", "\"foo\"", ":", "\"bar\"", "\"baz\"", ":", "false", "}"
        ]);
    }

    #[test]
    #[should_panic]
    fn trailing_comma_on_doubleton() {
        parse(vec_of_strings![
            "{", "\"foo\"", ":", "\"bar\"", ",", "\"baz\"", ":", "false", ",", "}"
        ]);
    }

    #[test]
    #[should_panic]
    fn leading_comma_on_doubleton() {
        parse(vec_of_strings![
            "{", ",", "\"foo\"", ":", "\"bar\"", ",", "\"baz\"", ":", "false", "}"
        ]);
    }

    #[test]
    #[should_panic]
    fn doubled_comma_on_doubleton() {
        parse(vec_of_strings![
            "{", "\"foo\"", ":", "\"bar\"", ",", ",", "\"baz\"", ":", "false", "}"
        ]);
    }
    #[test]
    #[should_panic]
    fn object_with_leading_comma() {
        parse(vec_of_strings!("{", ",", "\"foo\"", ":", "\"bar\"", "}"));
    }

    #[test]
    #[should_panic]
    fn object_with_trailing_comma() {
        parse(vec_of_strings!("{", "\"foo\"", ":", "\"bar\"", ",", "}"));
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
