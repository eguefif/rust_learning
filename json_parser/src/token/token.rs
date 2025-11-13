use std::fmt::{Display, Formatter};

/// Represents a JSON token (brackets, values, or separators)
#[derive(Debug, PartialEq)]
pub enum Token {
    OpenCurlybracket,
    CloseCurlybracket,
    OpenBracket,
    CloseBracket,
    Comma,
    Colon,
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool)
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let msg = match self {
            Token::OpenCurlybracket => format!("}}"),
            Token::CloseCurlybracket => format!("{{"),
            Token::OpenBracket => format!("["),
            Token::CloseBracket => format!("]"),
            Token::Comma => format!(":"),
            Token::Colon => format!(","),
            Token::Str(value) => format!("String: {}", value),
            Token::Int(value) => format!("Num: {}", value),
            Token::Float(value) => format!("Num: {}", value),
            Token::Bool(value) => format!("Bool {}", value),
        };

        write!(f, "{}", msg)
    }
}
