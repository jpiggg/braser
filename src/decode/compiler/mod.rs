use std::str::FromStr;
use wasm_bindgen::prelude::*;
use js_sys;
use crate::shared::Node;

pub fn to_object(pairs: &Vec<Node>) -> js_sys::Object {
    let object = js_sys::Object::new();

    for pair in pairs.iter() {
        let str = pair.value.as_str();
        let key: js_sys::JsString = js_sys::JsString::from(&str[1..str.len() - 1]).into();
        let _ = js_sys::Reflect::set(&object, &key, &run(pair.children.first().unwrap()));
    }

    object
}

pub fn to_array(items: &Vec<Node>) -> js_sys::Array {
    let result = js_sys::Array::new();

    for item in items.iter() {
        result.push(&run(item));
    }

    result
}

pub fn run(node: &Node) -> JsValue {
    let res: JsValue = match node.kind.as_str() {
        "object" => {
            to_object(&node.children).into()
        },
        "array" => {
            to_array(&node.children).into()
        },
        "string" => {
            let str = node.value.as_str();
            js_sys::JsString::from(&str[1..str.len() - 1]).into()
        },
        "number" => {
            let val: f64 = node.value.as_str().parse().unwrap();
            js_sys::Number::from(val).into()
        },
        "bigint" => {
            js_sys::BigInt::from_str(&node.value.as_str()).unwrap().into()
        },
        "infinity" => {
            if node.value.as_str() == "-1" {
                js_sys::Number::NEGATIVE_INFINITY.into()
            } else {
                js_sys::Number::POSITIVE_INFINITY.into()
            }
        },
        "null" => {
            JsValue::null()
        },
        "nan" => {
            JsValue::from(f64::NAN)
        }
        "date" => {
            js_sys::Date::new(&js_sys::JsString::from(node.value.as_str())).into()
        },
        "boolean" => {
            JsValue::as_bool(&js_sys::JsString::from(node.value.as_str())).into()
        },
        "function" => {
            js_sys::eval(node.value.as_str()).unwrap()
        },
        _ => {
            JsValue::undefined()
        }
    };

res
}