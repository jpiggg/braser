use wasm_bindgen::{prelude::*, convert::FromWasmAbi};
use js_sys;
use crate::shared::Node;

// struct Foo;
// struct Bar;

// enum A {
//     AF(Foo),
//     AB(Bar),
//     AS(&'static str)
// }

// fn foobar(key: usize) -> A {
//     match key {
//         1 => A::AF(Foo),
//         2 => A::AB(Bar),
//         _ => A::AS("Something Else")
//     }
// }

#[wasm_bindgen]
pub struct Foo {
    hello: String
}

// Javascript Array https://rustwasm.github.io/wasm-bindgen/reference/types/boxed-jsvalue-slice.html

pub enum Result {
    Array(Box<[JsValue]>),
    String(String),
    F32(f32)
}

#[wasm_bindgen]
pub fn run<'a>(tree: Node<'a>) -> JsValue {
    let a = Foo{hello: "world"};
    let mut obj: js_sys::Object = js_sys::Object::from(a);
    // Node.children[0].kind === type of result;
}