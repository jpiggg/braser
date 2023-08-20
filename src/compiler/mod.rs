use std::str::FromStr;
use wasm_bindgen::prelude::*;
use js_sys;
use crate::shared::Node;

pub fn to_object(pairs: &Vec<Node>) -> js_sys::Object {
    let object = js_sys::Object::new();

    for pair in pairs.iter() {
        let str: js_sys::JsString = js_sys::JsString::from(pair.value.as_str()).into();
       let key = str.substr(1, (str.length() - 2).try_into().unwrap()).into();
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
            let str = js_sys::JsString::from(node.value.as_str());
            str.substr(1, (str.length() - 2).try_into().unwrap()).into()
        },
        "number" => {
            js_sys::Number::from_str(node.value.as_str()).unwrap().into()
        },
        "bigint" => {
            js_sys::BigInt::from_str(&node.value.as_str()).unwrap().into()
        },
        "infinity" => {
            if node.value.as_str() == "-" {
                js_sys::Number::NEGATIVE_INFINITY.into()
            } else {
                js_sys::Number::POSITIVE_INFINITY.into()
            }
        },
        "null" => {
            JsValue::null()
        },
        "undefined" => {
            JsValue::undefined()
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
        // "function" => {},
        _ => {
            panic!("There is no trasformers for specified node kind {:?}", node.kind.as_str());
        }
    };

res
}