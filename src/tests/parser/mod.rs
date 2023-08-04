use pretty_assertions::assert_eq;
use crate::parser::run as parser;
use crate::shared::{tokens::Token, Node};

#[test]
fn test_flatten_object() {
    let tokens = vec![
        Token { name: "OS", value: "" },
        Token { name: "ST", value: "\"foo####\\\" bar\\\"\"" },
        Token { name: "KT", value: "" },
        Token { name: "ST", value: "\"test\"" },
        Token { name: "LT", value: "" },
        Token { name: "ST", value: "\"hi\"" },
        Token { name: "KT", value: "" },
        Token { name: "NU", value: "100500" },
        Token { name: "LT", value: "" },
        Token { name: "ST", value: "\"date\"" },
        Token { name: "KT", value: "" },
        Token { name: "DT", value: "2023-08-01T14:32:01.624Z" },
        Token { name: "LT", value: "" },
        Token { name: "ST", value: "\"myFn\"" },
        Token { name: "KT", value: "" },
        Token { name: "FU", value: "function my_fn(a, b) {\n        return a + b;\n    }" },
        Token { name: "LT", value: "" }, 
        Token { name: "ST", value: "\"undefined\"" },
        Token { name: "KT", value: "" },
        Token { name: "UN", value: "" },
        Token { name: "LT", value: "" },
        Token { name: "ST", value: "\"nan\"" },
        Token { name: "KT", value: "" },
        Token { name: "NA", value: "" },
        Token { name: "LT", value: "" },
        Token { name: "ST", value: "\"null\"" },
        Token { name: "KT", value: "" },
        Token { name: "NL", value: "" },
        Token { name: "OE", value: "" }
    ];

    let result = parser(Node {
        kind: "root",
        value: "",
        children: vec![]
    }, &tokens);
    let expected = Node{
        kind: "root",
        value: "",
        children: vec![
            Node {
                kind: "object",
                value: "",
                children: vec![
                    Node {
                        kind: "pair",
                        value: "\"foo####\\\" bar\\\"\"",
                        children: vec![
                            Node {
                                kind: "string",
                                value: "\"test\"",
                                children: vec![],
                            },
                        ],
                    },
                    Node {
                        kind: "pair",
                        value: "\"hi\"",
                        children: vec![
                            Node {
                                kind: "number",
                                value: "100500",
                                children: vec![],
                            },
                        ],
                    },
                    Node {
                        kind: "pair",
                        value: "\"date\"",
                        children: vec![
                            Node {
                                kind: "date",
                                value: "2023-08-01T14:32:01.624Z",
                                children: vec![],
                            },
                        ],
                    },
                    Node {
                        kind: "pair",
                        value: "\"myFn\"",
                        children: vec![
                            Node {
                                kind: "function",
                                value: "function my_fn(a, b) {\n        return a + b;\n    }",
                                children: vec![],
                            },
                        ],
                    },
                    Node {
                        kind: "pair",
                        value: "\"undefined\"",
                        children: vec![
                            Node {
                                kind: "undefined",
                                value: "",
                                children: vec![],
                            },
                        ],
                    },
                    Node {
                        kind: "pair",
                        value: "\"nan\"",
                        children: vec![
                            Node {
                                kind: "nan",
                                value: "",
                                children: vec![],
                            },
                        ],
                    },
                    Node {
                        kind: "pair",
                        value: "\"null\"",
                        children: vec![
                            Node {
                                kind: "null",
                                value: "",
                                children: vec![],
                            },
                        ],
                    },
                ],
            },
    ]
    };

    assert_eq!(result, expected);
}

#[test]
fn nested_object() {
    let tokens = vec![
        Token{ name: "OS", value: ""},
        Token{ name: "ST", value: "\"foo\""},
        Token{ name: "KT", value: ""},
        Token{ name: "OS", value: ""},
        Token{ name: "ST", value:  "\"hello\""},
        Token{ name: "KT", value:  ""},
        Token{ name: "ST", value:  "\"world\""},
        Token{ name: "OE", value:  ""},
        Token{name: "LT", value: ""},
        Token{name: "ST", value: "\"second\""},
        Token{name: "KT", value: ""},
        Token{name: "NU", value: "12345"},
        Token{ name: "OE", value:  ""}
    ];

    let result = parser(Node {
        kind: "root",
        value: "",
        children: vec![]
    }, &tokens);

    let expected = Node {
        kind: "root",
        value: "",
        children: vec![
            Node {
                kind: "object",
                value: "",
                children: vec![
                    Node{
                        kind: "pair",
                        value: "\"foo\"",
                        children: vec![
                            Node {
                                kind: "object",
                                value: "",
                                children: vec![
                                    Node {
                                        kind: "pair",
                                        value: "\"hello\"",
                                        children: vec![
                                            Node{
                                                kind: "string",
                                                value: "\"world\"",
                                                children: vec![]
                                            }
                                        ]
                                    }
                                ]
                            }
                        ]
                    },
                    Node{
                        kind: "pair",
                        value: "\"second\"",
                        children: vec![
                            Node {
                                kind: "number",
                                value: "12345",
                                children: vec![]
                            }
                        ]
                    }
                ]
            }
        ]
    };

    assert_eq!(result, expected);
}

#[test]
fn empty_object() {
    let tokens = vec![
        Token{name: "OS", value: ""},
        Token{name: "OE", value: ""}
    ];

    let result = parser(Node {
        kind: "root",
        value: "",
        children: vec![]
    }, &tokens);

    let expected = Node{
        kind: "root",
        value: "",
        children: vec![
            Node {
                kind: "object",
                value: "",
                children: vec![]
            }
        ]
    };

    assert_eq!(result, expected)
}

#[test]
fn flatten_array() {
    let tokens = vec![
        Token{name: "AS", value: ""},
        Token{name: "ST", value: "\"foo####\\\" bar\\\"\""},
        Token{name: "LT", value: ""},
        Token{name: "OS", value: ""},
        Token{name: "ST", value: "\"hi\""},
        Token{name: "KT", value: ""},
        Token{name: "DT", value: "2023-08-01T14:32:01.624Z"},
        Token{name: "LT", value: ""},
        Token{name: "ST", value: "\"test\""},
        Token{name: "KT", value: ""},
        Token{name: "NU", value: "123"},
        Token{name: "OE", value: ""},
        Token{name: "LT", value: ""},
        Token{name: "NL", value: ""},
        Token{name: "AE", value: ""},
    ];

    let result = parser(Node {
        kind: "root",
        value: "",
        children: vec![]
    }, &tokens);

    let expected = Node {
        kind: "root",
        value: "",
        children: vec![
            Node {
                kind: "array",
                value: "",
                children: vec![
                    Node{
                        kind: "string",
                        value: "\"foo####\\\" bar\\\"\"",
                        children: vec![]
                    },
                    Node {
                        kind: "object",
                        value: "",
                        children: vec![
                            Node {
                                kind: "pair",
                                value: "\"hi\"",
                                children: vec![
                                    Node {
                                        kind: "date",
                                        value: "2023-08-01T14:32:01.624Z",
                                        children: vec![]
                                    }
                                ]
                            },
                            Node {
                                kind: "pair",
                                value: "\"test\"",
                                children: vec![
                                    Node {
                                        kind: "number",
                                        value: "123",
                                        children: vec![]
                                    }
                                ]
                            }
                        ]
                    },
                    Node {
                        kind: "null",
                        value: "",
                        children: vec![]
                    }
                ]
            }
        ]
    };

    assert_eq!(result, expected);
}
#[test]
fn nested_array() {
    let tokens = vec![
        Token{name: "AS", value: ""},
        Token{name: "UN", value: ""},
        Token{name: "LT", value: ""},
        Token{name: "AS", value: ""},
        Token{name: "OS", value: ""},
        Token{name: "ST", value: "\"hello\""},
        Token{name: "KT", value: ""},
        Token{name: "AS", value: ""},
        Token{name: "BI", value: "9007199254740991n"},
        Token{name: "AE", value: ""},
        Token{name: "OE", value: ""},
        Token{name: "AE", value: ""},
        Token{name: "AE", value: ""}
    ];

    let result = parser(Node {
        kind: "root",
        value: "",
        children: vec![]
    }, &tokens);

    let expected = Node {
        kind: "root",
        value: "",
        children: vec![
            Node {
                kind: "array",
                value: "",
                children: vec![
                    Node{
                        kind: "undefined",
                        value: "",
                        children: vec![]
                    },
                    Node {
                        kind: "array",
                        value: "",
                        children: vec![
                            Node {
                                kind: "object",
                                value: "",
                                children: vec![
                                    Node {
                                        kind: "pair",
                                        value: "\"hello\"",
                                        children: vec![
                                            Node {
                                                kind: "array",
                                                value: "",
                                                children: vec![
                                                    Node {
                                                        kind: "bigint",
                                                        value: "9007199254740991n",
                                                        children: vec![]
                                                    }
                                                ]
                                            }
                                        ]
                                    }
                                ]
                            }
                        ]
                    }
                ]
            }
        ]
    };

    assert_eq!(result, expected);
}
#[test]
fn empty_flatten_array() {
    let tokens = vec![
        Token{name: "AS", value: ""},
        Token{name: "AE", value: ""}
    ];

    let result = parser(Node {
        kind: "root",
        value: "",
        children: vec![]
    }, &tokens);

    let expected = Node {
        kind: "root",
        value: "",
        children: vec![
            Node {
                kind: "array",
                value: "",
                children: vec![]
            }
        ]
    };

    assert_eq!(result, expected);
}
#[test]
fn empty_nested_array() {
    let tokens = vec![
        Token{name: "AS", value: ""},
        Token{name: "AS", value: ""},
        Token{name: "AE", value: ""},
        Token{name: "AE", value: ""}
    ];

    let result = parser(Node {
        kind: "root",
        value: "",
        children: vec![]
    }, &tokens);

    let expected = Node {
        kind: "root",
        value: "",
        children: vec![
            Node{
                kind: "array",
                value: "",
                children: vec![
                    Node {
                        kind: "array",
                        value: "",
                        children: vec![]
                    }
                ]
            }
        ]
    };

    assert_eq!(result, expected);
}