use std::{collections::HashMap, iter::Peekable};

pub mod lex;

#[derive(PartialEq, Debug)]
pub enum Value {
    Number(f64),
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
        "null" => Value::Null,
        "{" => object(it),
        "[" => array(it),
        s if t.starts_with("\"") => Value::String(string(s)),
        _ if ('0'..'9').contains(&t.chars().next().unwrap()) => number(t),
        s => {
            dbg!(s);
            todo!("");
        }
    }
}

fn number(t: String) -> Value {
    Value::Number(t.parse::<f64>().unwrap())
}

fn array(it: &mut Peekable<std::vec::IntoIter<String>>) -> Value {
    let mut arr = Vec::new();
    let mut just_saw_comma = false;
    loop {
        let tok = it.peek().unwrap();
        match tok.as_str() {
            "]" => {
                assert!(!just_saw_comma, "Array has trailing comma");
                it.next();
                return Value::Array(arr);
            }
            "," => {
                assert!(!just_saw_comma, "Array has duplicate comma");
                assert!(arr.len() > 0, "Array has leading comma");
                just_saw_comma = true;
                it.next();
            } // todo do comma assertions
            _ => {
                assert!(
                    just_saw_comma || arr.len() == 0,
                    "Array items not separated by comma"
                );
                just_saw_comma = false;
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
}

fn string(t: &str) -> String {
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
            parse(vec_of_strings!["[", "true", ",", "false", "]"])
        );

        assert_eq!(
            Value::Array(vec![
                Value::Boolean(true),
                Value::Object(HashMap::new()),
                Value::Boolean(false)
            ]),
            parse(vec_of_strings![
                "[", "true", ",", "{", "}", ",", "false", "]"
            ])
        );

        assert_eq!(
            Value::Array(vec![Value::Number(5.), Value::Number(27.), Value::Null]),
            parse(vec_of_strings!["[", "5", ",", "27", ",", "null", "]"])
        );
    }

    #[test]
    #[should_panic]
    fn array_leading_comma() {
        parse(vec_of_strings!["[", ",", "false", "]"]);
    }

    #[test]
    #[should_panic]
    fn array_trailing_comma() {
        parse(vec_of_strings!["[", "false", ",", "]"]);
    }

    #[test]
    #[should_panic]
    fn array_duplicate_comma() {
        parse(vec_of_strings!["[", "true", ",", ",", "false", "]"]);
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
