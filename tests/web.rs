use wasm_bindgen_test::wasm_bindgen_test_configure;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use js_sys;
use stringify::decode;

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
#[wasm_bindgen_test]
fn test() {
    let src: &str = r#"a${3$"foo":a${3$"hello":3$"world"}}"#;
    let res = decode(src);
    let obj: JsValue = js_sys::Reflect::get(res.first().unwrap(), &js_sys::JsString::from("\"foo\"")).unwrap();
    let result = js_sys::Reflect::get(&obj, &js_sys::JsString::from("\"hello\"")).unwrap();
    let expected: JsValue = js_sys::JsString::from("\"world\"").into();

    assert_eq!(result, expected);
}
