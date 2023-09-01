use wasm_bindgen_test::wasm_bindgen_test_configure;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use js_sys;
use stringify::decode::*;

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
#[wasm_bindgen_test]
fn nested_object_all_types() {
    let src: &str = r#"a${3$"foo":a${3$"hello":3$"world", 3$"number": 4$12345}}"#;

    let string = js_sys::JsString::from(src);
    let res = decode(string);
    let obj: JsValue = js_sys::Reflect::get(&res, &js_sys::JsString::from("foo")).unwrap();
    let result_string = js_sys::Reflect::get(&obj, &js_sys::JsString::from("hello")).unwrap();
    let result_number: JsValue = js_sys::Reflect::get(&obj, &js_sys::JsString::from("number")).unwrap();
    let expected_string: JsValue = js_sys::JsString::from("world").into();
    let expected_number: JsValue = js_sys::Number::from(12345).into();

    assert_eq!(result_string, expected_string);
    assert_eq!(result_number, expected_number);
}
