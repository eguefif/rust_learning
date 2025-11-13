use std::iter::Iterator;
use std::iter::Peekable;
use std::str::Chars;

pub use super::token::Token;

/// Tokenizes JSON input into a stream of tokens
#[derive(Debug)]
pub struct Tokenizer<'a> {
    json: Peekable<Chars<'a>>
}

impl<'a> Tokenizer<'a> {
    pub fn new(json: &'a str) -> Tokenizer<'a> {
        Tokenizer { json: json.chars().peekable() }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    /// Returns the next token from the JSON input, skipping whitespace
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(token) = self.json.next() {
            match token {
                '{' => return Some(Token::OpenCurlybracket),
                '}' => return Some(Token::CloseCurlybracket),
                '[' => return Some(Token::OpenBracket),
                ']' => return Some(Token::CloseBracket),
                ',' => return Some(Token::Comma),
                ':' => return Some(Token::Colon),
                ' ' => continue,
                '\t' => continue,
                '\n' => continue,
                _ => return Some(self.parse_complex_token(token))
            }
        }
        None
    }
}

impl Tokenizer<'_> {
    /// Checks if the next non-whitespace token is a closing curly bracket
    pub fn is_next_token_closing_curly_bracket(&mut self) -> bool {
        while let Some(peek) = self.json.peek() {
            match peek {
                ' ' | '\t' | '\n' => {
                    self.json.next();
                    continue;
                },
                '}' => return true,
                _ => return false,
            }
        }
        false
    }

    fn parse_complex_token(&mut self, token: char) -> Token {
        if token == '"' {
            return self.parse_string();
        } else if token.is_numeric() {
            return self.parse_numeric(token);
        } else if token == 'f' || token == 't' {
            return self.parse_bool(token)
        } else {
            panic!("Error: unknown char: |{}|", token);
        }
    }

    fn parse_string(&mut self) -> Token {
        let mut string_token = String::new();
        while let Some(next_char) = self.json.next() {
            if next_char == '"' {
                return Token::Str(string_token);
            }
            string_token.push(next_char)
        }
        panic!("Error: string does not end with \" token"); 
    }

    fn parse_numeric(&mut self, token: char) -> Token {
        let mut numeric_string = String::new();
        numeric_string.push(token);
        let mut is_float = false;
        while let Some(peek_char) = self.json.peek() {
            if peek_char.is_numeric() {
                numeric_string.push(*peek_char);
            }  else if *peek_char == '.' {
                numeric_string.push(*peek_char);
                is_float = true;
            }
            else {
                break;
            }
            self.json.next();
        }

        if is_float == true {
            let number = numeric_string.parse::<f64>().expect("Error: string is not a number");
            return Token::Float(number);
        } else {
            let number = numeric_string.parse::<i64>().expect("Error: string is not a number");
            return Token::Int(number);
        }
    }

    fn parse_bool(&mut self, token: char) -> Token {
        let mut boolean_str = token.to_string();
        let bool_letters = vec!['a', 'l', 's', 'e', 'r', 'u'];
        while let Some(next_char) = self.json.peek() {
            if bool_letters.contains(&next_char) {
                boolean_str.push(*next_char);
                self.json.next();
            } else {
                break;
            }
        }
        if boolean_str == "true" {
            return Token::Bool(true);
        } else if boolean_str == "false" {
            return Token:: Bool(false)
        }
        Token::Str(boolean_str)
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_should_return_next_basic_token() {
        let json = r#" { } [ ]  ,"#;
        let mut tokenizer = Tokenizer::new(json);

        assert_eq!(Token::OpenCurlybracket, tokenizer.next().unwrap());
        assert_eq!(Token::CloseCurlybracket, tokenizer.next().unwrap());
        assert_eq!(Token::OpenBracket, tokenizer.next().unwrap());
        assert_eq!(Token::CloseBracket, tokenizer.next().unwrap());
        assert_eq!(Token::Comma, tokenizer.next().unwrap());
        assert_eq!(None, tokenizer.next());
    }

    #[test]
    fn it_should_return_next_token() {
        let json = r#"
{ 
    "key1": "value1",
    "key2": 5,
    "key3": 1.1,
    "key4": 15.13,
    "key5": false,
    "key6": true
}"#;
        let mut tokenizer = Tokenizer::new(json);

        assert_eq!(Token::OpenCurlybracket, tokenizer.next().unwrap());

        assert_eq!(Token::Str("key1".to_string()), tokenizer.next().unwrap());
        assert_eq!(Token::Colon, tokenizer.next().unwrap());
        assert_eq!(Token::Str("value1".to_string()), tokenizer.next().unwrap());
        assert_eq!(Token::Comma, tokenizer.next().unwrap());

        assert_eq!(Token::Str("key2".to_string()), tokenizer.next().unwrap());
        assert_eq!(Token::Colon, tokenizer.next().unwrap());
        assert_eq!(Token::Int(5), tokenizer.next().unwrap());
        assert_eq!(Token::Comma, tokenizer.next().unwrap());

        assert_eq!(Token::Str("key3".to_string()), tokenizer.next().unwrap());
        assert_eq!(Token::Colon, tokenizer.next().unwrap());
        assert_eq!(Token::Float(1.1), tokenizer.next().unwrap());
        assert_eq!(Token::Comma, tokenizer.next().unwrap());

        assert_eq!(Token::Str("key4".to_string()), tokenizer.next().unwrap());
        assert_eq!(Token::Colon, tokenizer.next().unwrap());
        assert_eq!(Token::Float(15.13), tokenizer.next().unwrap());
        assert_eq!(Token::Comma, tokenizer.next().unwrap());

        assert_eq!(Token::Str("key5".to_string()), tokenizer.next().unwrap());
        assert_eq!(Token::Colon, tokenizer.next().unwrap());
        assert_eq!(Token::Bool(false), tokenizer.next().unwrap());
        assert_eq!(Token::Comma, tokenizer.next().unwrap());

        assert_eq!(Token::Str("key6".to_string()), tokenizer.next().unwrap());
        assert_eq!(Token::Colon, tokenizer.next().unwrap());
        assert_eq!(Token::Bool(true), tokenizer.next().unwrap());

        assert_eq!(Token::CloseCurlybracket, tokenizer.next().unwrap());

        assert_eq!(None, tokenizer.next());
    }

    #[test]
    fn it_should_tokenize_object() {
        let json = r#"
{ 
    "key1": 5,
    "key2": {
        "key21": 15,
        "key22": false
    }
}"#;
        let mut tokenizer = Tokenizer::new(json);

        assert_eq!(Token::OpenCurlybracket, tokenizer.next().unwrap());

        assert_eq!(Token::Str("key1".to_string()), tokenizer.next().unwrap());
        assert_eq!(Token::Colon, tokenizer.next().unwrap());
        assert_eq!(Token::Int(5), tokenizer.next().unwrap());
        assert_eq!(Token::Comma, tokenizer.next().unwrap());

        assert_eq!(Token::Str("key2".to_string()), tokenizer.next().unwrap());
        assert_eq!(Token::Colon, tokenizer.next().unwrap());

        assert_eq!(Token::OpenCurlybracket, tokenizer.next().unwrap());

        assert_eq!(Token::Str("key21".to_string()), tokenizer.next().unwrap());
        assert_eq!(Token::Colon, tokenizer.next().unwrap());
        assert_eq!(Token::Int(15), tokenizer.next().unwrap());
        assert_eq!(Token::Comma, tokenizer.next().unwrap());

        assert_eq!(Token::Str("key22".to_string()), tokenizer.next().unwrap());
        assert_eq!(Token::Colon, tokenizer.next().unwrap());
        assert_eq!(Token::Bool(false), tokenizer.next().unwrap());


        assert_eq!(Token::CloseCurlybracket, tokenizer.next().unwrap());
        assert_eq!(Token::CloseCurlybracket, tokenizer.next().unwrap());

        assert_eq!(None, tokenizer.next());
    }

    #[test]
    fn it_should_tokenize_array() {
        let json = r#"
{ 
    "key1": 5,
    "key2": [1, 2, 3]
}"#;
        let mut tokenizer = Tokenizer::new(json);

        assert_eq!(Token::OpenCurlybracket, tokenizer.next().unwrap());

        assert_eq!(Token::Str("key1".to_string()), tokenizer.next().unwrap());
        assert_eq!(Token::Colon, tokenizer.next().unwrap());
        assert_eq!(Token::Int(5), tokenizer.next().unwrap());
        assert_eq!(Token::Comma, tokenizer.next().unwrap());

        assert_eq!(Token::Str("key2".to_string()), tokenizer.next().unwrap());
        assert_eq!(Token::Colon, tokenizer.next().unwrap());

        assert_eq!(Token::OpenBracket, tokenizer.next().unwrap());
        assert_eq!(Token::Int(1), tokenizer.next().unwrap());
        assert_eq!(Token::Comma, tokenizer.next().unwrap());
        assert_eq!(Token::Int(2), tokenizer.next().unwrap());
        assert_eq!(Token::Comma, tokenizer.next().unwrap());
        assert_eq!(Token::Int(3), tokenizer.next().unwrap());

        assert_eq!(Token::CloseBracket, tokenizer.next().unwrap());
        assert_eq!(Token::CloseCurlybracket, tokenizer.next().unwrap());

        assert_eq!(None, tokenizer.next());
    }
}
