use crate::token::tokenizer::Tokenizer;
use crate::parser::Parser;
use crate::error::JsonError;

pub mod token;
pub mod parser;
pub mod error;

pub use parser::json::{Json, JsonType, Object};

pub fn from_string(json_string: &str) -> Result<Json, JsonError> {
    let tokenizer = Tokenizer::new(json_string);
    let mut parser = Parser::new(tokenizer);
    parser.parse_tokens()
}
