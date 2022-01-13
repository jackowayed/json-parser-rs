use std::collections::HashMap;

use crate::lex::Token;

#[derive(PartialEq, Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
}

pub fn parse(tokens: Vec<Token>) -> Vec<Value> {
    todo!()
}
