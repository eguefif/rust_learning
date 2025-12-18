use json_parser::{from_string, to_string};
use json_parser::types::Object;
use json_parser::{Deserialize, JsonType, error::JsonError, Serialize};

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

impl Serialize for Person {
    fn serialize(&self) -> JsonType {
        let v = vec![("name".to_string(), JsonType::Str(self.name.clone()))];
        let object = Object { data: v };
        JsonType::Object(Box::new(object))
    }
}

fn main() {
    // Example with a struct that implements Deserialize and Serialize
    let json = r#"
{ 
    "name": "Hello World"
}"#;
    let j: Person = from_string(json).unwrap();

    println!("Hello: {}", j.name);

    let serialized: String = to_string(j).unwrap();
    println!("Peson serialized: {}", serialized);

    // Example with generic data
    let json_str = r#"{"name": "Alice", "age": 30}"#;
    let json: JsonType = from_string(json_str).unwrap();
    println!("{:?}", json["age"]);
    let serialized = to_string(json).unwrap();
    println!("Serialized: {}", serialized);
}
