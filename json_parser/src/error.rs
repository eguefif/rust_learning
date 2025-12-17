use crate::token::tokenizer::Token;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum JsonError {
    EmptyInput,
    UnexpectedToken(Token),
    KeyError(Token),
    ValueError(Token),
    CollonError(Token),
    ComaError(Token),
    EndObjectError(Token),
    InvalidComaEndObjectError,
    UnexpectedEndOfJson,
    DeserializationError(String),
}

impl Error for JsonError {}

impl Display for JsonError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let msg = match self {
            JsonError::DeserializationError(msg) => {
                format!("Deserialization into struct error: {}.", msg)
            }
            JsonError::InvalidComaEndObjectError => format!("An object must not end with a coma."),
            JsonError::UnexpectedEndOfJson => format!("Unexpected end of json string."),
            JsonError::EmptyInput => format!("Empty input string"),
            JsonError::UnexpectedToken(got) => format!("Unexpected token, got: {}", got),
            JsonError::KeyError(token) => format!("Key error, got: {}", token),
            JsonError::ValueError(token) => format!("Value error, got: {}", token),
            JsonError::CollonError(token) => format!("Expected collon ':' but got: {}", token),
            JsonError::ComaError(token) => format!("Expected coma ',' but got: {}", token),
            JsonError::EndObjectError(token) => {
                format!("Object does not end properly, got: {}", token)
            }
        };
        write!(f, "Error: {}", msg)
    }
}
