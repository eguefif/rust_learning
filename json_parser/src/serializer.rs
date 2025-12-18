use crate::JsonType;
use crate::error::JsonError;
use crate::types::Object;

pub fn serialize_json(data: &JsonType) -> Result<String, JsonError> {
    let mut retval = String::new();
    match data {
        JsonType::Str(value) => {
            retval.push('"');
            retval.push_str(value);
            retval.push('"');
        }
        JsonType::Num(value) => {
            retval.push_str(&value.serialize());
        }
        JsonType::Bool(value) => {
            if value == &true {
                retval.push_str("true");
            } else {
                retval.push_str("false");
            }
        }
        JsonType::Array(value) => retval.push_str(&serialize_array(value)?),
        JsonType::Object(value) => retval.push_str(&serialize_object(value)?),
    }
    Ok(retval)
}

fn serialize_array(input: &Vec<JsonType>) -> Result<String, JsonError> {
    let mut retval = String::new();
    let mut peek = input.iter().peekable();
    retval.push('[');
    loop {
        let Some(next_input) = peek.next() else {
            return Err(JsonError::SerializationError(
                "Wrong array format".to_string(),
            ));
        };
        retval.push_str(&serialize_json(next_input)?);
        if let None = peek.peek() {
            break;
        }
        retval.push(',');
    }
    retval.push(']');
    Ok(retval)
}

fn serialize_object(input: &Object) -> Result<String, JsonError> {
    let mut retval = String::new();
    let mut peek = input.data.iter().peekable();
    retval.push('{');
    loop {
        let Some((key, value)) = peek.next() else {
            return Err(JsonError::SerializationError(
                "Wrong object format".to_string(),
            ));
        };
        retval.push('"');
        retval.push_str(&key);
        retval.push('"');
        retval.push_str(":");
        retval.push_str(&serialize_json(value)?);
        if let None = peek.peek() {
            break;
        }
        retval.push(',');
    }
    retval.push('}');
    Ok(retval)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Num, Object};

    #[test]
    fn it_should_serialize_string() {
        let input = JsonType::Str("Hello, World".to_string());
        let result = serialize_json(&input).unwrap();

        assert_eq!("\"Hello, World\"", result);
    }

    #[test]
    fn it_should_serialize_num() {
        let input = JsonType::Num(Num::Integer(54));
        let result = serialize_json(&input).unwrap();

        assert_eq!("54", result);
    }

    #[test]
    fn it_should_serialize_bool() {
        let input = JsonType::Bool(true);
        let result = serialize_json(&input).unwrap();

        assert_eq!("true", result);
    }

    #[test]
    fn it_should_serialize_array() {
        let v = vec![JsonType::Str("hello".to_string()), JsonType::Bool(true)];
        let input = JsonType::Array(v);
        let result = serialize_json(&input).unwrap();

        assert_eq!("[\"hello\",true]", result);
    }

    #[test]
    fn it_should_serialize_object() {
        let v = Object {
            data: vec![
                ("key1".to_string(), JsonType::Str("hello".to_string())),
                ("key2".to_string(), JsonType::Bool(true)),
            ],
        };
        let input = JsonType::Object(Box::new(v));
        let result = serialize_json(&input).unwrap();

        assert_eq!("{\"key1\":\"hello\",\"key2\":true}", result);
    }
}
