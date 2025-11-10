use crate::token::tokenizer::Tokenizer;

pub mod token;

pub fn from_string(json_string: &str) {
    let mut tokenizer = Tokenizer::new(json_string);
    tokenizer.next();
}
