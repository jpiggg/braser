use wasm_bindgen_test::wasm_bindgen_test_configure;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use js_sys;

use eson::parse::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn eson_parse_browser() {
    let source_code = "{
        name: 13,
        value: 'baz',
        isDefined: false,
        isNotDefined: true
        // This is a response from some API
        data: undefined,
        valid_until: NaN,
        valid_from: 1763225669356
        /*
            All cool guys use buffer like this:
            [1, 2, 3] // Yes, it is a comment inside another one!
        */
        buffer: [16, 21, 51, 0, 0, 0, 0],
        src: ['a', 'b', 'c'],
        source: 'abc'
    }";
    
    let string = js_sys::JsString::from(source_code);
    let res = parse(string);
    let result_string = js_sys::Reflect::get(&res, &js_sys::JsString::from("value")).unwrap();
    let result_number: JsValue = js_sys::Reflect::get(&res, &js_sys::JsString::from("name")).unwrap();
    let expected_string: JsValue = js_sys::JsString::from("baz").into();
    let expected_number: JsValue = js_sys::Number::from(13).into();

    assert_eq!(result_string, expected_string);
    assert_eq!(result_number, expected_number);
}