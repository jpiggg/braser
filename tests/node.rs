use wasm_bindgen_test::wasm_bindgen_test_configure;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use js_sys;

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
fn eson_parse_nodejs() {
    use eson::parse::*;

    let cwd = process_cwd().as_string().unwrap().to_owned();
    let source_code = read_file(&(cwd + "/examples/data.eson")).unwrap().obj;
    
    let string = js_sys::JsString::from(js_sys::ArrayBuffer::from(source_code).to_js_string());
    let res = parse(string);
    let result_string = js_sys::Reflect::get(&res, &js_sys::JsString::from("value")).unwrap();
    let result_number: JsValue = js_sys::Reflect::get(&res, &js_sys::JsString::from("name")).unwrap();
    let expected_string: JsValue = js_sys::JsString::from("ba\\'z").into();
    let expected_number: JsValue = js_sys::Number::from(13).into();

    assert_eq!(result_string, expected_string);
    assert_eq!(result_number, expected_number);
}

// #[wasm_bindgen_test]
// fn eson_parse_all_types() {
//     use eson::parse::*;

//     let eson_source = r#"{
//         str: "hello",
//         num: 42,
//         bool_true: true,
//         bool_false: false,
//         null_val: null,
//         undefined_val: undefined,
//         nan_val: NaN,
//         infinity_val: Infinity,
//         bigint_val: 123456789n,
//         arr: [1, "two", true],
//         nested: {
//             inner: "value"
//         }
//     }"#;

//     let string = js_sys::JsString::from(eson_source);
//     let res = parse(string);

//     // Test String
//     let str_val = js_sys::Reflect::get(&res, &js_sys::JsString::from("str")).unwrap();
//     let expected: JsValue = js_sys::JsString::from("hello").into();
//     assert_eq!(str_val, expected);

//     // Test Number
//     let num_val = js_sys::Reflect::get(&res, &js_sys::JsString::from("num")).unwrap();
//     let expected: JsValue = js_sys::Number::from(42).into();
//     assert_eq!(num_val, expected);

//     // Test Boolean true
//     let bool_true_val = js_sys::Reflect::get(&res, &js_sys::JsString::from("bool_true")).unwrap();
//     assert_eq!(bool_true_val, JsValue::from(true));

//     // Test Boolean false
//     let bool_false_val = js_sys::Reflect::get(&res, &js_sys::JsString::from("bool_false")).unwrap();
//     assert_eq!(bool_false_val, JsValue::from(false));

//     // Test Null
//     let null_val = js_sys::Reflect::get(&res, &js_sys::JsString::from("null_val")).unwrap();
//     assert_eq!(null_val, JsValue::null());

//     // Test Undefined
//     let undefined_val = js_sys::Reflect::get(&res, &js_sys::JsString::from("undefined_val")).unwrap();
//     assert_eq!(undefined_val, JsValue::undefined());

//     // Test NaN
//     let nan_val = js_sys::Reflect::get(&res, &js_sys::JsString::from("nan_val")).unwrap();
//     assert!(js_sys::Number::is_nan(&nan_val));

//     // Test Infinity
//     let infinity_val = js_sys::Reflect::get(&res, &js_sys::JsString::from("infinity_val")).unwrap();
//     let expected: JsValue = js_sys::Number::POSITIVE_INFINITY.into();
//     assert_eq!(infinity_val, expected);

//     // Test BigInt
//     let bigint_val = js_sys::Reflect::get(&res, &js_sys::JsString::from("bigint_val")).unwrap();
//     assert!(bigint_val.is_bigint());

//     // Test Array
//     let arr = js_sys::Reflect::get(&res, &js_sys::JsString::from("arr")).unwrap();
//     assert!(arr.is_array());
//     let arr_obj = js_sys::Array::from(&arr);
//     let first_expected_value: JsValue = js_sys::Number::from(1).into();
//     let second_expected_value: JsValue = js_sys::JsString::from("two").into();

//     assert_eq!(arr_obj.get(0), first_expected_value);
//     assert_eq!(arr_obj.get(1), second_expected_value);
//     assert_eq!(arr_obj.get(2), JsValue::from(true));

//     // Test Nested Object
//     let nested = js_sys::Reflect::get(&res, &js_sys::JsString::from("nested")).unwrap();
//     let inner = js_sys::Reflect::get(&nested, &js_sys::JsString::from("inner")).unwrap();

//     let expected: JsValue = js_sys::JsString::from("value").into();
//     assert_eq!(inner, expected);
// }

#[wasm_bindgen_test]
fn eson_stringify_nodejs() {
    use eson::stringify::*;

    let obj = js_sys::Object::new().into();

    js_sys::Reflect::set(&obj, &js_sys::JsString::from("foo"), &js_sys::JsString::from("bar")).unwrap();

    let res = stringify(&obj);

    assert_eq!(res, js_sys::JsString::from("{foo:\"bar\"}"));
}

#[wasm_bindgen_test]
fn eson_stringify_all_types() {
    use eson::stringify::*;

    let obj = js_sys::Object::new().into();

    // String
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("str"), &js_sys::JsString::from("hello")).unwrap();

    // String with escaped quotes
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("str_escaped"), &js_sys::JsString::from("hello\"world'test")).unwrap();

    // Number (positive)
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("num_42"), &js_sys::Number::from(42)).unwrap();

    // Number (negative)
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("num_negative"), &js_sys::Number::from(-42)).unwrap();

    // Number (float)
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("num_float"), &js_sys::Number::from(3.14159)).unwrap();

    // Number (negative float)
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("num_negative_float"), &js_sys::Number::from(-3.14159)).unwrap();

    // Boolean true
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("bool_true"), &JsValue::from(true)).unwrap();

    // Boolean false
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("bool_false"), &JsValue::from(false)).unwrap();

    // Null
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("null_val"), &JsValue::null()).unwrap();

    // NaN
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("nan_val"), &JsValue::from(f64::NAN)).unwrap();

    // Infinity (positive)
    let val: JsValue = js_sys::Number::POSITIVE_INFINITY.into();
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("infinity_val"), &val).unwrap();

    // Infinity (negative)
    let neg_infinity: JsValue = js_sys::Number::NEGATIVE_INFINITY.into();
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("infinity_negative"), &neg_infinity).unwrap();

    // BigInt (positive)
    let bigint_pos = JsValue::bigint_from_str("123456789");
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("bigint_pos"), &bigint_pos).unwrap();

    // BigInt (negative)
    let bigint_neg = JsValue::bigint_from_str("-987654321");
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("bigint_neg"), &bigint_neg).unwrap();

    // Array with mixed types
    let arr = js_sys::Array::new();
    arr.push(&js_sys::Number::from(1));
    arr.push(&js_sys::JsString::from("two"));
    arr.push(&JsValue::from(true));
    arr.push(&js_sys::Number::from(-5.5));
    arr.push(&JsValue::null());
    let bigint_in_arr = JsValue::bigint_from_str("999");
    arr.push(&bigint_in_arr);
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("arr"), &arr).unwrap();

    // Nested Object
    let nested = js_sys::Object::new().into();
    js_sys::Reflect::set(&nested, &js_sys::JsString::from("inner"), &js_sys::JsString::from("value")).unwrap();
    js_sys::Reflect::set(&nested, &js_sys::JsString::from("nested_num"), &js_sys::Number::from(-99.99)).unwrap();
    
    // Empty array in nested object
    let empty_arr = js_sys::Array::new();
    js_sys::Reflect::set(&nested, &js_sys::JsString::from("empty_arr"), &empty_arr).unwrap();

    // Empty object in nested object
    let empty_obj = js_sys::Object::new().into();
    js_sys::Reflect::set(&nested, &js_sys::JsString::from("empty_obj"), &empty_obj).unwrap();
    
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("nested"), &nested).unwrap();

    let res = stringify(&obj);

    console_log!(" -----> res {:#?}", res);

    // Verify the stringified output contains expected patterns
    let res_str = res.as_string().unwrap();
    assert!(res_str.contains("str:\"hello\""));
    assert!(res_str.contains("str_escaped:\"hello\\\"world'test\"") || res_str.contains("str_escaped:\"hello\"world'test\""));
    assert!(res_str.contains("num_42:42"));
    assert!(res_str.contains("num_negative:-42"));
    assert!(res_str.contains("num_float:3.14159"));
    assert!(res_str.contains("num_negative_float:-3.14159"));
    assert!(res_str.contains("bool_true:true"));
    assert!(res_str.contains("bool_false:false"));
    assert!(res_str.contains("null_val:null"));
    assert!(res_str.contains("nan_val:NaN"));
    assert!(res_str.contains("infinity_val:Infinity"));
    assert!(res_str.contains("infinity_negative:-Infinity"));
    assert!(res_str.contains("bigint_pos:123456789n"));
    assert!(res_str.contains("bigint_neg:-987654321n"));
    assert!(res_str.contains("arr:[1,\"two\",true,-5.5,null,999n]"));
    assert!(res_str.contains("nested:{inner:\"value\",nested_num:-99.99,empty_arr:[],empty_obj:{}}"));
}