use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct LexicalError<'a> {
    pub name: &'a str,
    pub source: LexicalErrorSource,
    pub msg: String
}


impl<'a> LexicalError<'a> {
    pub fn new(kind: &'a str, symbol: char, token: &'a str, index: usize, src: String) -> Self {
        let msg = match kind {
            "UNEXPECTED_END" => format!("Unexpected end {symbol:?} of token {token:?} at position {index:?}"),
            _ => format!("Unknown symbol {symbol:?} for {token:?} at position {index:?}")
        };

        LexicalError {
            name: "Lexical error",
            source: LexicalErrorSource::new(src),
            msg: msg
        }
    }
}


impl<'a> fmt::Display for LexicalError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl<'a> Error for LexicalError<'a> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}


#[derive(Debug, PartialEq)]
pub struct LexicalErrorSource {
    src: String
}

impl LexicalErrorSource {
    pub fn new(src: String) -> Self {
        LexicalErrorSource {
            src: src
        }
    }
}

impl fmt::Display for LexicalErrorSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.src)
    }
}

impl Error for LexicalErrorSource {}