use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ParsingError<'a> {
    pub name: &'a str,
    pub msg: String
}


impl<'a> ParsingError<'a> {
    pub fn new(kind: &'a str) -> Self {
        let msg = match kind {
            "INVALID_KEY_PAIR" => format!("Invalid object key pair"),
            "INVALID_OBJECT_KEY" => format!("Invalid object key"),
            _ => format!("Unknown parsing error")
        };

        ParsingError {
            name: "Lexical error",
            msg: msg
        }
    }
}

impl<'a> fmt::Display for ParsingError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl<'a> Error for ParsingError<'a> {}
