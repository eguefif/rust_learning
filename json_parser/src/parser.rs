use crate::parser::json::{Object, JsonType, Json};
use crate::parser::num::Num;
use crate::token::tokenizer::{Token, Tokenizer};
use crate::error::JsonError;

pub mod num;
pub mod json;

// TODO: handle error in malformatted string (should I check the whole string ahead ?)
// TODO: parse array

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: Tokenizer<'a>) -> Parser<'a> {
        Self { tokenizer }
    }

    pub fn parse_tokens(&mut self) -> Result<Json, JsonError> {
        if let Some(token) = self.tokenizer.next() {
            match token {
                Token::OpenCurlybracket => {
                    let data = self.parse_object()?;
                    return Ok(Json { data });
                },
                Token::OpenBracket => {todo!()},
                _ => return Err(JsonError::UnexpectedToken(token)),
            }
            
        };
        Err(JsonError::EmptyInput)
    }

    fn parse_object(&mut self) -> Result<Object, JsonError> {
        let mut data: Vec<(String, JsonType)> = Vec::new();

        loop {
            let key_value = self.get_key_value_pair()?;
            data.push(key_value);
            if self.expect_coma_or_end_object()? {
                break
            }
        }
        Ok(Object { data })
    }

    fn get_key_value_pair(&mut self) -> Result<(String, JsonType), JsonError> {
        let next_token = self.tokenizer.next();
        let key = match next_token {
            Some(key) => self.get_key(key)?,
            None => return Err(JsonError::UnexpectedEndOfJson)
        };

        self.expect_colon()?;

        let next_token = self.tokenizer.next();
        let value = match next_token {
            Some(value) => self.get_value(value)?,
            None => return Err(JsonError::UnexpectedEndOfJson),
        };

        Ok((key, value))
    }

    fn get_key(&mut self, token: Token) -> Result<String, JsonError> {
        if let Token::Str(key) = token {
            return Ok(key)
        }
        Err(JsonError::KeyError(token))
    }

    fn expect_colon(&mut self) -> Result<(), JsonError> {
        let token = self.tokenizer.next();
        if let Some(token) = token {
            if token == Token::Colon {
                return Ok(());
            }
            return Err(JsonError::CollonError(token));
        }
        Err(JsonError::UnexpectedEndOfJson)
    }

    fn get_value(&mut self, token: Token) -> Result<JsonType, JsonError> {
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
            Token::OpenCurlybracket => {
                let nested_object = self.parse_object()?;
                return Ok(JsonType::Object(Box::new(nested_object)))
            }
            _ => {}
        }
        Err(JsonError::ValueError(token))
    }

    /// Expects either a comma or closing curly bracket after a key-value pair.
    ///
    /// Returns:
    /// - `Ok(true)` if the object ends (closing curly bracket encountered)
    /// - `Ok(false)` if there are more key-value pairs (comma encountered)
    /// - `Err` if an invalid token is encountered or if the JSON ends unexpectedly
    fn expect_coma_or_end_object(&mut self) -> Result<bool, JsonError> {
        if let Some(next_token) = self.tokenizer.next() {
            match next_token {
                Token::Comma => {
                    if self.tokenizer.is_next_token_closing_curly_bracket() {
                        return Err(JsonError::InvalidComaEndObjectError);
                    }
                    return Ok(false);
                }
                Token::CloseCurlybracket => return Ok(true),
                _ => return Err(JsonError::EndObjectError(next_token))
            }
        }
        Err(JsonError::UnexpectedEndOfJson)
    }
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
        let mut parser = Parser::new(tokenizer);
        let json = parser.parse_tokens().unwrap();
        assert_eq!(json["key1"], JsonType::Str("value1".to_string()));
        assert_eq!(json["key2"], JsonType::Num(Num::Integer(5)));
        assert_eq!(json["key3"], JsonType::Num(Num::Float(15.13)));
        assert_eq!(json["key4"], JsonType::Bool(false));
        assert_eq!(json["key5"], JsonType::Bool(true));
    }

    #[test]
    fn it_should_parse_nested_object_json() {
        let json = r#"
{ 
    "key1": "value1",
    "key2": {
        "key21": "Hello",
        "key22": "World"
    }
}"#;
        let tokenizer = Tokenizer::new(json);
        let mut parser = Parser::new(tokenizer);
        let json = parser.parse_tokens().unwrap();
        assert_eq!(json["key1"], JsonType::Str("value1".to_string()));
        let nested_object = &json["key2"];
        if let JsonType::Object(nested_object) = nested_object {
            println!("obj: {:?}", nested_object);
            assert_eq!(nested_object["key21"], JsonType::Str("Hello".to_string()));
            assert_eq!(nested_object["key22"], JsonType::Str("World".to_string()));
        } else {
            panic!("json[\"key2\"] should be an object");
        }
    }

    #[test]
    fn it_should_return_an_error_when_not_starting_from_curbly_bracket() {
        let json = "\"a\"{";
        let tokenizer = Tokenizer::new(json);
        let mut parser = Parser::new(tokenizer);
        let json = parser.parse_tokens();
        if let Err(error) = json {
            assert_eq!(error, JsonError::UnexpectedToken(Token::Str("a".to_string())));
        } else {
            panic!("Expect error")
        }
    }
}
