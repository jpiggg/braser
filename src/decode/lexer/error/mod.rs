use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct LexicalError<'a> {
    pub name: &'a str,
    pub source: &'a str,
    pub msg: String
}


impl<'a> LexicalError<'a> {
    pub fn new(symbol: &'a str, token: &'a str, index: i64, src: &'a str) -> Self {
        LexicalError {
            name: "Lexical error",
            source: src,
            msg: format!("Unknown symbol {symbol:?} for {token:?} at position {index:?}")
        }
    }
}


impl<'a> fmt::Display for LexicalError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl<'a> Error for LexicalError<'a> {}

// impl<'a> Error for LexicalError<'a> {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         Some(&self.source)
//     }
// }