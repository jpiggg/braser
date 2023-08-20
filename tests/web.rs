use wasm_bindgen_test::wasm_bindgen_test_configure;
use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;
use stringify::decode;

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
#[wasm_bindgen_test]
fn test() {
    let src: &str = r#"a${3$"foo":a${3$"hello":3$"world"}}"#;
    let res = decode(src);

    assert_eq!(res.first().unwrap(), &JsValue::null());
}