
pub mod parser;
pub mod compiler;

use self::parser::ast::ESon;
use self::compiler::compile;
use from_pest::FromPest;
use pest::Parser;
use std::fs;

use js_sys::JsString;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn parse(source: JsString) -> JsValue {
    // let source = String::from_utf8(fs::read("./examples/data.eson").unwrap()).unwrap();
    let raw_source= &source.as_string().unwrap();
    let mut parse_tree = self::parser::ESonParser::parse(self::parser::Rule::ESon, &raw_source).unwrap();
    println!("parse tree = {parse_tree:#?}");
    let syntax_tree: ESon = ESon::from_pest(&mut parse_tree).expect("infallible");
    println!("syntax tree = {syntax_tree:#?}");
    
    let result: wasm_bindgen::JsValue = compile(&syntax_tree);
    
    result
}