use pretty_assertions::assert_eq;
use wasm_bindgen::prelude::*;
use crate::compiler::run as compiler;
use crate::shared::Node;

#[test]
fn test_flatten_object() {
    let nodes_tree = Node{
        kind: String::from("root"),
        value: String::from(""),
        children: vec![
            Node {
                kind: String::from("object"),
                value: String::from(""),
                children: vec![
                    Node {
                        kind: String::from("pair"),
                        value: String::from("\"foo####\\\" bar\\\"\""),
                        children: vec![
                            Node {
                                kind: String::from("string"),
                                value: String::from("\"test\""),
                                children: vec![],
                            },
                        ],
                    },
                    Node {
                        kind: String::from("pair"),
                        value: String::from("\"hi\""),
                        children: vec![
                            Node {
                                kind: String::from("number"),
                                value: String::from("100500"),
                                children: vec![],
                            },
                        ],
                    },
                    Node {
                        kind: String::from("pair"),
                        value: String::from("\"date\""),
                        children: vec![
                            Node {
                                kind: String::from("date"),
                                value: String::from("2023-08-01T14:32:01.624Z"),
                                children: vec![],
                            },
                        ],
                    },
                    Node {
                        kind: String::from("pair"),
                        value: String::from("\"myFn\""),
                        children: vec![
                            Node {
                                kind: String::from("function"),
                                value: String::from("function my_fn(a, b) {\n        return a + b;\n    }"),
                                children: vec![],
                            },
                        ],
                    },
                    Node {
                        kind: String::from("pair"),
                        value: String::from("\"undefined\""),
                        children: vec![
                            Node {
                                kind: String::from("undefined"),
                                value: String::from(""),
                                children: vec![],
                            },
                        ],
                    },
                    Node {
                        kind: String::from("pair"),
                        value: String::from("\"nan\""),
                        children: vec![
                            Node {
                                kind: String::from("nan"),
                                value: String::from(""),
                                children: vec![],
                            },
                        ],
                    },
                    Node {
                        kind: String::from("pair"),
                        value: String::from("\"null\""),
                        children: vec![
                            Node {
                                kind: String::from("null"),
                                value: String::from(""),
                                children: vec![],
                            },
                        ],
                    },
                ],
            },
    ]
    };
    let res = compiler(&vec![nodes_tree]);

    assert_eq!(res.first().unwrap(), &JsValue::null());
}