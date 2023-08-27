pub mod lexer;
pub mod parser;
pub mod compiler;

pub mod tests;

use self::lexer::run as lexer;
use self::parser::run as parser;
use self::compiler::run as compiler;

use js_sys::JsString;
use wasm_bindgen::prelude::*;

use crate::shared::Node;

#[wasm_bindgen]
pub fn decode(source: JsString) -> wasm_bindgen::JsValue {
    let lexems = lexer(&source.as_string().unwrap());

    let nodes_tree = parser(Node {
        kind: String::from("root"),
        value: String::from(""),
        children: vec![]
    }, &lexems);

    let result: wasm_bindgen::JsValue = compiler(nodes_tree.children.first().unwrap());

    result
}