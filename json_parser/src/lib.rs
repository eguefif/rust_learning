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
//! use json_parser::{from_string, Json, JsonType};
//! use json_parser::types::Num;
//!
//! let json_str = r#"{"name": "Alice", "age": 30}"#;
//! let json: Json = from_string(json_str).unwrap();
//!
//! // Access values using string indexing
//! assert_eq!(json["name"], JsonType::Str("Alice".to_string()));
//! ```

use std::ops::Index;
use crate::token::tokenizer::Tokenizer;
use crate::parser::Parser;
use crate::error::JsonError;
use crate::types::Num;

pub mod token;
pub mod parser;
pub mod error;
pub mod types;

pub use types::Object;

#[derive(Debug, PartialEq)]
pub enum JsonType {
    Str(String),
    Num(Num),
    Bool(bool),
    Object(Box<Object>),
    Array(Vec<JsonType>)
}

impl Index<&str> for JsonType {
    type Output = JsonType;

    fn index<'a, 'b>(&'a self, index: &'b str) -> &'a Self::Output {
        if let JsonType::Object(obj) = &self {
            return &obj[index]
        }
        panic!();
    }
}

impl Index<usize> for JsonType {
    type Output = JsonType;

    fn index<'a, 'b>(&'a self, index: usize) -> &'a Self::Output {
        if let JsonType::Array(obj) = &self {
            return &obj[index]
        }
        panic!();
    }
}

/// A parsed JSON document that can be indexed by string keys or numeric indices
///
/// # Examples
///
/// ```
/// use json_parser::{from_string, Json, JsonType};
/// use json_parser::types::Num;
///
/// let json_str = r#"{"name": "Alice", "age": 30}"#;
/// let json: Json = from_string(json_str).unwrap();
///
/// // Access values using string indexing
/// assert_eq!(json["name"], JsonType::Str("Alice".to_string()));
///
/// // Nested objects
/// let json_str = r#"{"user": {"name": "Bob"}}"#;
/// let json: Json = from_string(json_str).unwrap();
/// if let JsonType::Object(user) = &json["user"] {
///     assert_eq!(user["name"], JsonType::Str("Bob".to_string()));
/// }
///
/// // Arrays
/// let json_str = r#"{"items": [1, 2, 3]}"#;
/// let json: Json = from_string(json_str).unwrap();
/// if let JsonType::Array(items) = &json["items"] {
///     assert_eq!(items[0], JsonType::Num(Num::Integer(1)));
/// }
/// ```
#[derive(Debug)]
pub struct Json {
    pub(crate) data: JsonType,
}

impl Index<&str> for Json {
    type Output = JsonType;

    fn index<'a, 'b>(&'a self, index: &'b str) -> &'a Self::Output {
        if let JsonType::Object(obj) = &self.data {
            return &obj[index]
        }
        panic!();
    }
}

impl Index<usize> for Json {
    type Output = JsonType;

    fn index<'a, 'b>(&'a self, index: usize) -> &'a Self::Output {
        if let JsonType::Array(obj) = &self.data {
            return &obj[index]
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
/// # Arguments
///
/// * `json_string` - A string slice containing valid JSON
///
/// # Returns
///
/// * `Ok(T)` - Successfully parsed and deserialized data
/// * `Err(JsonError)` - Parsing or deserialization failed
///
/// # Examples
///
/// ## Parsing into the `Json` type for dynamic access
///
/// ```
/// use json_parser::{from_string, Json, JsonType};
///
/// let json: Json = from_string(r#"{"name": "Alice", "age": 30}"#).unwrap();
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
///     type Item = Person;
///     fn deserialize(data: JsonType) -> Result<Self::Item, JsonError> {
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

impl Deserialize for Json {
    fn deserialize(data: JsonType) -> Result<Self, JsonError> {
        Ok(Json {
            data
        })
    }
}


/// Trait for types that can be deserialized from JSON data
///
/// Implement this trait to enable parsing JSON strings directly into your custom types
/// using the [`from_string`] function. The trait requires an associated type `Item` which
/// represents the output type of deserialization.
///
/// # Examples
///
/// ## Basic implementation for a simple struct
///
/// ```
/// use json_parser::{Deserialize, JsonType, error::JsonError};
///
/// struct Person {
///     name: String,
/// }
///
/// impl Deserialize for Person {
///     type Item = Person;
///
///     fn deserialize(data: JsonType) -> Result<Self::Item, JsonError> {
///         // Extract the "name" field from the JSON object
///         if let JsonType::Str(name) = &data["name"] {
///             return Ok(Person {
///                 name: name.to_string()
///             });
///         }
///         Err(JsonError::DeserializationError(
///             "Cannot find field 'name' in JSON".to_string()
///         ))
///     }
/// }
/// ```
///
/// ## Using with `from_string`
///
/// ```
/// use json_parser::{from_string, Deserialize, JsonType, error::JsonError};
///
/// # struct Person {
/// #     name: String,
/// # }
/// #
/// # impl Deserialize for Person {
/// #     type Item = Person;
/// #     fn deserialize(data: JsonType) -> Result<Self::Item, JsonError> {
/// #         if let JsonType::Str(name) = &data["name"] {
/// #             return Ok(Person { name: name.to_string() });
/// #         }
/// #         Err(JsonError::DeserializationError("Cannot find field 'name'".to_string()))
/// #     }
/// # }
///
/// let json = r#"{"name": "Alice"}"#;
/// let person: Person = from_string(json).unwrap();
/// assert_eq!(person.name, "Alice");
/// ```
pub trait Deserialize {
    fn deserialize(json: JsonType) -> Result<Self, JsonError> where Self: Sized;
}
