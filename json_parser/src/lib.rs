//! JSON parser library
//!
//! This library provides functionality for parsing JSON strings into Rust data structures.
//! The main entry point is the [`from_string`] function, which parses a JSON string
//! and returns a [`Json`] struct that can be indexed using string keys (for objects)
//! or numeric indices (for arrays).
//!
//! # Examples
//!
//! ```
//! use json_parser::{from_string, JsonType};
//! use json_parser::types::Num;
//!
//! let json_str = r#"{"name": "Alice", "age": 30}"#;
//! let json: JsonType = from_string(json_str).unwrap();
//!
//! // Access values using string indexing
//! assert_eq!(json["name"], JsonType::Str("Alice".to_string()));
//! ```

use crate::error::JsonError;
use crate::parser::Parser;
use crate::serializer::serialize_json;
use crate::token::tokenizer::Tokenizer;
use crate::types::Num;
use std::ops::Index;

pub mod error;
pub mod parser;
pub mod serializer;
pub mod token;
pub mod types;

pub use types::Object;

/// A parsed JSON enum that can be indexed by string keys or numeric indices
///
/// # Examples
///
/// ```
/// use json_parser::{from_string, JsonType};
/// use json_parser::types::Num;
///
/// let json_str = r#"{"name": "Alice", "age": 30}"#;
/// let json: JsonType = from_string(json_str).unwrap();
///
/// // Access values using string indexing
/// assert_eq!(json["name"], JsonType::Str("Alice".to_string()));
/// ```

#[derive(Debug, PartialEq, Clone)]
pub enum JsonType {
    Str(String),
    Num(Num),
    Bool(bool),
    Object(Box<Object>),
    Array(Vec<JsonType>),
}

// This impl allow the following use case:
//  let json_str = r#"{"name": "Alice", "age": 30}"#;
//  let json: JsonType = from_string(json_str).unwrap();
impl Deserialize for JsonType {
    fn deserialize(data: JsonType) -> Result<Self, JsonError> {
        Ok(data)
    }
}

impl Serialize for JsonType {
    fn serialize(&self) -> JsonType {
        self.clone()
    }
}

impl Index<&str> for JsonType {
    type Output = JsonType;

    fn index<'a, 'b>(&'a self, index: &'b str) -> &'a Self::Output {
        if let JsonType::Object(obj) = &self {
            return &obj[index];
        }
        panic!();
    }
}

impl Index<usize> for JsonType {
    type Output = JsonType;

    fn index<'a, 'b>(&'a self, index: usize) -> &'a Self::Output {
        if let JsonType::Array(obj) = &self {
            return &obj[index];
        }
        panic!();
    }
}

/// Parses a JSON string into any type that implements the `Deserialize` trait
///
/// This function is generic over types that implement [`Deserialize`], allowing you to
/// parse JSON directly into custom Rust structures or use the provided [`Json`] type
/// for dynamic access.
///
/// # Examples
///
/// ## Parsing into the `Json` type for dynamic access
///
/// ```
/// use json_parser::{from_string, JsonType};
///
/// let json: JsonType = from_string(r#"{"name": "Alice", "age": 30}"#).unwrap();
/// assert_eq!(json["name"], JsonType::Str("Alice".to_string()));
/// ```
///
/// ## Parsing into a custom type
///
/// ```
/// use json_parser::{from_string, Deserialize, JsonType, error::JsonError};
///
/// struct Person {
///     name: String,
/// }
///
/// impl Deserialize for Person {
///     fn deserialize(data: JsonType) -> Result<Person, JsonError> {
///         if let JsonType::Str(name) = &data["name"] {
///             return Ok(Person {
///                 name: name.to_string()
///             });
///         }
///         Err(JsonError::DeserializationError("Cannot find field 'name'".to_string()))
///     }
/// }
///
/// let person: Person = from_string(r#"{"name": "Alice"}"#).unwrap();
/// assert_eq!(person.name, "Alice");
/// ```
pub fn from_string<T: Deserialize>(json_string: &str) -> Result<T, JsonError> {
    let tokenizer = Tokenizer::new(json_string);
    let mut parser = Parser::new(tokenizer);
    let data = parser.parse_tokens()?;
    <T as Deserialize>::deserialize(data)
}

/// Trait for types that can be deserialized from JSON data
///
/// Implement this trait to enable parsing JSON strings directly into your custom types
/// using the [`from_string`] function. The trait requires an associated type `Item` which
/// represents the output type of deserialization.
pub trait Deserialize {
    fn deserialize(json: JsonType) -> Result<Self, JsonError>
    where
        Self: Sized;
}

pub fn to_string<T: Serialize>(input: T) -> Result<String, JsonError> {
    let json_data = input.serialize();
    serialize_json(&json_data)
}

pub trait Serialize {
    fn serialize(self: &Self) -> JsonType;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_serialize_jsontype() {
        let v = Object {
            data: vec![
                ("key1".to_string(), JsonType::Str("hello".to_string())),
                ("key2".to_string(), JsonType::Bool(true)),
            ],
        };
        let input = JsonType::Object(Box::new(v));
        let result: String = to_string(input).unwrap();

        assert_eq!("{\"key1\":\"hello\",\"key2\":true}", result);
    }
}
