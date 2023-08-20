use std::str::FromStr;

use wasm_bindgen::prelude::*;
use js_sys;
use crate::shared::Node;

pub enum Result {
    Array(Box<[JsValue]>),
    String(String),
    F32(f32)
}

pub fn to_object(pairs: &Vec<Node>) -> js_sys::Object {
    let object = js_sys::Object::new();

    for pair in pairs.iter() {
        let key: js_sys::JsString = js_sys::JsString::from(pair.value.as_str()).into();
        let _ = js_sys::Reflect::set(&object, &key, &compile(pair.children.first().unwrap()));
    }

    object
}

pub fn to_array(items: &Vec<Node>) -> js_sys::Array {
    let result = js_sys::Array::new();

    for item in items.iter() {
        result.push(&compile(item));
    }

    result
}

pub fn compile(node: &Node) -> JsValue {
    let result = js_sys::Array::new();
    for child in node.children.iter() {
        let res: JsValue = match child.kind.as_str() {
            "object" => {
                to_object(&child.children).into()
            },
            "array" => {
                to_array(&child.children).into()
            },
            "string" => {
                js_sys::JsString::from(child.value.as_str()).into()
            },
            "number" => {
                js_sys::Number::from_str(child.value.as_str()).unwrap().into()
            },
            "bigint" => {
                js_sys::BigInt::from_str(&child.value.as_str()).unwrap().into()
            },
            "infinity" => {
                if child.value.as_str() == "-" {
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
                js_sys::Date::new(&js_sys::JsString::from(child.value.as_str())).into()
            },
            "boolean" => {
                JsValue::as_bool(&js_sys::JsString::from(child.value.as_str())).into()
            },
            // "function" => {},
            _ => {
                panic!("There is should be a node kind")
            }
        };

        result.push(&res);
    }

    result.into()

}

pub fn run(nodes: &Vec<Node>) -> Vec<JsValue> {
    let mut res: Vec<JsValue> = vec![];

    for node in nodes.iter() {
        res.push(compile(node));
    }

    res

}