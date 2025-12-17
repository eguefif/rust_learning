use json_parser::from_string;
use json_parser::{Deserialize, JsonType, error::JsonError};

// TODO: create a type and impl the Deserialize
// Add a trait
// Think about two Deserialize impl: one from a string, one from a Json return by from_string
// It will be use for nested object in Deserialize
//
struct Person {
    name: String,
}

impl Deserialize for Person {
    fn deserialize(data: JsonType) -> Result<Person, JsonError> {
        if let JsonType::Str(name) = &data["name"] {
            return Ok(Self {
                name: name.to_string(),
            });
        }
        Err(JsonError::DeserializationError(
            "Cannot find field string name in Json".to_string(),
        ))
    }
}

fn main() {
    let json = r#"
{ 
    "name": "Hello World"
}"#;
    let j: Person = from_string(json).unwrap();

    println!("Hello: {}", j.name);

    let json_str = r#"{"name": "Alice", "age": 30}"#;
    let json: JsonType = from_string(json_str).unwrap();
    println!("{:?}", json["age"]);
}
