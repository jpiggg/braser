use wasm_bindgen_test::wasm_bindgen_test_configure;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use js_sys;
use std::fs;
// use eson::_old_decode::*;
wasm_bindgen_test_configure!();

use eson::parse::*;

#[wasm_bindgen]
extern "C" {
    type Buffer;
}

#[wasm_bindgen(module = "fs")]
extern "C" {
    #[wasm_bindgen(js_name = readFileSync, catch)]
    fn read_file(path: &str) -> Result<Buffer, JsValue>;
}

#[wasm_bindgen_test]
fn flat_object() {
    // let src: &str = r#"a${3$"foo":a${3$"hello":3$"world", 3$"number": 4$12345}}"#;
    
    let source_code = read_file("/Users/jpig/dev/parser/examples/data.eson").unwrap().obj;
    // let source = String::from_utf8(source_code).unwrap();

    let string = js_sys::JsString::from(js_sys::ArrayBuffer::from(source_code).to_js_string());
    let res = parse(string);
    // let obj: JsValue = js_sys::Reflect::get(&res, &js_sys::JsString::from("foo")).unwrap();
    let result_string = js_sys::Reflect::get(&res, &js_sys::JsString::from("value")).unwrap();
    let result_number: JsValue = js_sys::Reflect::get(&res, &js_sys::JsString::from("name")).unwrap();
    let expected_string: JsValue = js_sys::JsString::from("baz").into();
    let expected_number: JsValue = js_sys::Number::from(13).into();

    assert_eq!(result_string, expected_string);
    assert_eq!(result_number, expected_number);
}
