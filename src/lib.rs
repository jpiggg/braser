pub mod lexer;
pub mod parser;
pub mod compiler;
pub mod shared;
pub mod tests;

use crate::lexer::run as lexer;
use crate::parser::run as parser;
use crate::compiler::run as compiler;

use crate::shared::Node;

pub fn decode(source: &str) -> Vec<wasm_bindgen::JsValue> {
    let lexems = lexer(source);

    let nodes_tree = parser(Node {
        kind: String::from("root"),
        value: String::from(""),
        children: vec![]
    }, &lexems);

    let result: Vec<wasm_bindgen::JsValue> = compiler(&vec![nodes_tree]);

    result
}