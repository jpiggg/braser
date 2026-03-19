use wasm_bindgen_test::wasm_bindgen_test_configure;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use js_sys;

use eson::parse::*;

wasm_bindgen_test_configure!();

#[wasm_bindgen]
extern "C" {
    type Buffer;
}

#[wasm_bindgen(module = "fs")]
extern "C" {
    #[wasm_bindgen(js_name = readFileSync, catch)]
    fn read_file(path: &str) -> Result<Buffer, JsValue>;
}


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = process, js_name = cwd)]
    fn process_cwd() -> JsValue;
}

#[wasm_bindgen_test]
fn flat_object_nodejs() {
    let cwd = process_cwd().as_string().unwrap().to_owned();
    let source_code = read_file(&(cwd + "/examples/data.eson")).unwrap().obj;
    
    let string = js_sys::JsString::from(js_sys::ArrayBuffer::from(source_code).to_js_string());
    let res = parse(string);
    let result_string = js_sys::Reflect::get(&res, &js_sys::JsString::from("value")).unwrap();
    let result_number: JsValue = js_sys::Reflect::get(&res, &js_sys::JsString::from("name")).unwrap();
    let expected_string: JsValue = js_sys::JsString::from("baz").into();
    let expected_number: JsValue = js_sys::Number::from(13).into();

    assert_eq!(result_string, expected_string);
    assert_eq!(result_number, expected_number);
}