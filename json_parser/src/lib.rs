use crate::token::tokenizer::Tokenizer;
use crate::parser::parse_tokens;
use crate::parser::num::Num;
use crate::error::JsonError;

use std::ops::Index;

pub mod token;
pub mod parser;
pub mod error;

#[derive(Debug, PartialEq)]
pub enum JsonType {
    Str(String),
    Num(Num),
    Bool(bool),
}

type Object = Vec<(String, JsonType)>;

#[derive(Debug)]
pub struct Json {
    data: Object,
}

impl Index<&str> for Json {
    type Output = JsonType;

    fn index<'a, 'b>(&'a self, index: &'b str) -> &'a Self::Output {
        for (key, value) in &self.data {
            if key == index {
                return &value;
            }
        }
        panic!("Error: trying to dereference Json object with unknown key");
    }
}

pub fn from_string(json_string: &str) -> Result<Json, JsonError> {
    let tokenizer = Tokenizer::new(json_string);
    parse_tokens(tokenizer)
}
