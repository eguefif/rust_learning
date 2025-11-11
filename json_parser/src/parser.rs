use crate::{Object, JsonType, Json};
use crate::parser::num::Num;
use crate::token::tokenizer::{Token, Tokenizer};
use crate::error::JsonError;

pub mod num;

// TODO: handle error in malformatted string (should I check the whole string ahead ?)
// TODO: find how to handle error with object that end with ,
// TODO: parse json with multiple nested json
// TODO: parse array

pub fn parse_tokens(mut tokenizer: Tokenizer) -> Result<Json, JsonError> {
    if let Some(token) = tokenizer.next() {
        match token {
            Token::OpenCurlybracket => {
                let data = parse_object(&mut tokenizer)?;
                return Ok(Json { data });
            },
            Token::OpenBracket => {todo!()},
            _ => return Err(JsonError::UnexpectedToken(token)),
        }
        
    };
    Err(JsonError::EmptyInput)
}

fn parse_object(tokenizer: &mut Tokenizer) -> Result<Object, JsonError> {
    let mut data: Object = Vec::new();

    loop {
        let key_value= get_key_value(tokenizer)?;
        data.push(key_value);
        if is_object_ending(tokenizer)? {
            break
        }
    }
    Ok(data)
}

fn get_key_value(tokenizer: &mut Tokenizer) -> Result<(String, JsonType), JsonError> {
    let next_token = tokenizer.next();
    let key = match next_token {
        Some(key) => get_key(key)?,
        None => return Err(JsonError::UnexpectedEndOfJson)
    };

    expect_colon(tokenizer)?;

    let next_token = tokenizer.next();
    let value = match next_token {
        Some(value) => get_value(value)?,
        None => return Err(JsonError::UnexpectedEndOfJson),
    };

    Ok((key, value))
}

fn get_key(token: Token) -> Result<String, JsonError> {
    if let Token::Str(key) = token {
        return Ok(key)
    }
    Err(JsonError::KeyError(token))
}

fn expect_colon(tokenizer: &mut Tokenizer) -> Result<(), JsonError> {
    let token = tokenizer.next();
    if let Some(token) = token {
        if token == Token::Colon {
            return Ok(());
        }
        return Err(JsonError::CollonError(token));
    }
    Err(JsonError::UnexpectedEndOfJson)
}

fn is_object_ending(tokenizer: &mut Tokenizer) -> Result<bool, JsonError> {
    if let Some(next_token) = tokenizer.next() {
        match next_token {
            Token::Comma => return Ok(false),
            Token::CloseCurlybracket => return Ok(true),
            _ => return Err(JsonError::EndObjectError(next_token))
        }
    }
    Err(JsonError::UnexpectedEndOfJson)
}

fn get_value(token: Token) -> Result<JsonType, JsonError> {
    match token {
        Token::Str(value) => return Ok(JsonType::Str(value)),
        Token::Int(value) => {
            let num = Num::Integer(value);
            return Ok(JsonType::Num(num));
        },
        Token::Float(value) => {
            let num = Num::Float(value);
            return Ok(JsonType::Num(num));
        },
        Token::Bool(value) => return Ok(JsonType::Bool(value)),
        _ => {}
    }
    Err(JsonError::ValueError(token))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::tokenizer::Tokenizer;

    #[test]
    fn it_should_parse_one_depth_json() {
        let json = r#"
{ 
    "key1": "value1",
    "key2": 5,
    "key3": 15.13,
    "key4": false,
    "key5": true
}"#;
    let tokenizer = Tokenizer::new(json);
    let json = parse_tokens(tokenizer).unwrap();
    assert_eq!(json["key1"], JsonType::Str("value1".to_string()));
    assert_eq!(json["key2"], JsonType::Num(Num::Integer(5)));
    assert_eq!(json["key3"], JsonType::Num(Num::Float(15.13)));
    assert_eq!(json["key4"], JsonType::Bool(false));
    assert_eq!(json["key5"], JsonType::Bool(true));
    }

    #[test]
    fn it_should_return_an_error_when_not_starting_from_curbly_bracket() {
        let json = "\"a\"{";
        let tokenizer = Tokenizer::new(json);
        let json = parse_tokens(tokenizer);
        if let Err(error) = json {
            assert_eq!(error, JsonError::UnexpectedToken(Token::Str("a".to_string())));
        } else {
            panic!("Expect error")
        }
    }
}
