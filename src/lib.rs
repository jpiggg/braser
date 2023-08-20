pub mod lexer;
pub mod parser;
pub mod compiler;
pub mod shared;
pub mod tests;

// use wasm_bindgen::prelude::*;
use crate::lexer::run as lexer;
use crate::parser::run as parser;
use crate::shared::Node;

// #[wasm_bindgen]
pub fn decode(source: &str) -> Node {
    let result = lexer(source);

    let nodes_tree = parser(Node {
        kind: String::from("root"),
        value: String::from(""),
        children: vec![]
    }, &result);

    nodes_tree
}