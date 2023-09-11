use pretty_assertions::assert_eq;
use crate::decode::parser::run as parser;
use crate::shared::{tokens::Token, Node};

#[test]
fn test_flatten_object() {
    let tokens = vec![
        Token { name: String::from("OS"), value: String::from("") },
        Token { name: String::from("ST"), value: String::from("\"foo####\\\" bar\\\"\"") },
        Token { name: String::from("KT"), value: String::from("") },
        Token { name: String::from("ST"), value: String::from("\"test\"") },
        Token { name: String::from("LT"), value: String::from("") },
        Token { name: String::from("ST"), value: String::from("\"hi\"") },
        Token { name: String::from("KT"), value: String::from("") },
        Token { name: String::from("NU"), value: String::from("100500") },
        Token { name: String::from("LT"), value: String::from("") },
        Token { name: String::from("ST"), value: String::from("\"date\"") },
        Token { name: String::from("KT"), value: String::from("") },
        Token { name: String::from("DT"), value: String::from("2023-08-01T14:32:01.624Z") },
        Token { name: String::from("LT"), value: String::from("") },
        Token { name: String::from("ST"), value: String::from("\"myFn\"") },
        Token { name: String::from("KT"), value: String::from("") },
        Token { name: String::from("FU"), value: String::from("[name=my_fn] function my_fn(a, b) {\n        return a + b;\n    }") },
        Token { name: String::from("LT"), value: String::from("") }, 
        Token { name: String::from("ST"), value: String::from("\"undefined\"") },
        Token { name: String::from("KT"), value: String::from("") },
        Token { name: String::from("UN"), value: String::from("") },
        Token { name: String::from("LT"), value: String::from("") },
        Token { name: String::from("ST"), value: String::from("\"nan\"") },
        Token { name: String::from("KT"), value: String::from("") },
        Token { name: String::from("NA"), value: String::from("") },
        Token { name: String::from("LT"), value: String::from("") },
        Token { name: String::from("ST"), value: String::from("\"null\"") },
        Token { name: String::from("KT"), value: String::from("") },
        Token { name: String::from("NL"), value: String::from("") },
        Token { name: String::from("OE"), value: String::from("") }
    ];

    let result = parser(Node {
        kind: String::from("root"),
        value: String::from(""),
        children: vec![]
    }, &tokens);
    let expected = Node{
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
                                value: String::from("var my_fn = function my_fn(a, b) {\n        return a + b;\n    }; my_fn"),
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

    assert_eq!(result, expected);
}

#[test]
fn nested_object() {
    let tokens = vec![
        Token{ name: String::from("OS"), value: String::from("")},
        Token{ name: String::from("ST"), value: String::from("\"foo\"")},
        Token{ name: String::from("KT"), value: String::from("")},
        Token{ name: String::from("OS"), value: String::from("")},
        Token{ name: String::from("ST"), value: String::from("\"hello\"")},
        Token{ name: String::from("KT"), value: String::from("")},
        Token{ name: String::from("ST"), value: String::from("\"world\"")},
        Token{ name: String::from("OE"), value: String::from("")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"second\"")},
        Token{name: String::from("KT"), value: String::from("")},
        Token{name: String::from("NU"), value: String::from("12345")},
        Token{ name: String::from("OE"), value: String::from("")}
    ];

    let result = parser(Node {
        kind: String::from("root"),
        value: String::from(""),
        children: vec![]
    }, &tokens);

    let expected = Node {
        kind: String::from("root"),
        value: String::from(""),
        children: vec![
            Node {
                kind: String::from("object"),
                value: String::from(""),
                children: vec![
                    Node{
                        kind: String::from("pair"),
                        value: String::from("\"foo\""),
                        children: vec![
                            Node {
                                kind: String::from("object"),
                                value: String::from(""),
                                children: vec![
                                    Node {
                                        kind: String::from("pair"),
                                        value: String::from("\"hello\""),
                                        children: vec![
                                            Node{
                                                kind: String::from("string"),
                                                value: String::from("\"world\""),
                                                children: vec![]
                                            }
                                        ]
                                    }
                                ]
                            }
                        ]
                    },
                    Node{
                        kind: String::from("pair"),
                        value: String::from("\"second\""),
                        children: vec![
                            Node {
                                kind: String::from("number"),
                                value: String::from("12345"),
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
        Token{name: String::from("OS"), value: String::from("")},
        Token{name: String::from("OE"), value: String::from("")}
    ];

    let result = parser(Node {
        kind: String::from("root"),
        value: String::from(""),
        children: vec![]
    }, &tokens);

    let expected = Node{
        kind: String::from("root"),
        value: String::from(""),
        children: vec![
            Node {
                kind: String::from("object"),
                value: String::from(""),
                children: vec![]
            }
        ]
    };

    assert_eq!(result, expected)
}

#[test]
fn flatten_array() {
    let tokens = vec![
        Token{name: String::from("AS"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"foo####\\\" bar\\\"\"")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("OS"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"hi\"")},
        Token{name: String::from("KT"), value: String::from("")},
        Token{name: String::from("DT"), value: String::from("2023-08-01T14:32:01.624Z")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"test\"")},
        Token{name: String::from("KT"), value: String::from("")},
        Token{name: String::from("NU"), value: String::from("123")},
        Token{name: String::from("OE"), value: String::from("")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("NL"), value: String::from("")},
        Token{name: String::from("AE"), value: String::from("")},
    ];

    let result = parser(Node {
        kind: String::from("root"),
        value: String::from(""),
        children: vec![]
    }, &tokens);

    let expected = Node {
        kind: String::from("root"),
        value: String::from(""),
        children: vec![
            Node {
                kind: String::from("array"),
                value: String::from(""),
                children: vec![
                    Node{
                        kind: String::from("string"),
                        value: String::from("\"foo####\\\" bar\\\"\""),
                        children: vec![]
                    },
                    Node {
                        kind: String::from("object"),
                        value: String::from(""),
                        children: vec![
                            Node {
                                kind: String::from("pair"),
                                value: String::from("\"hi\""),
                                children: vec![
                                    Node {
                                        kind: String::from("date"),
                                        value: String::from("2023-08-01T14:32:01.624Z"),
                                        children: vec![]
                                    }
                                ]
                            },
                            Node {
                                kind: String::from("pair"),
                                value: String::from("\"test\""),
                                children: vec![
                                    Node {
                                        kind: String::from("number"),
                                        value: String::from("123"),
                                        children: vec![]
                                    }
                                ]
                            }
                        ]
                    },
                    Node {
                        kind: String::from("null"),
                        value: String::from(""),
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
        Token{name: String::from("AS"), value: String::from("")},
        Token{name: String::from("UN"), value: String::from("")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("AS"), value: String::from("")},
        Token{name: String::from("OS"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"hello\"")},
        Token{name: String::from("KT"), value: String::from("")},
        Token{name: String::from("AS"), value: String::from("")},
        Token{name: String::from("BI"), value: String::from("9007199254740991n")},
        Token{name: String::from("AE"), value: String::from("")},
        Token{name: String::from("OE"), value: String::from("")},
        Token{name: String::from("AE"), value: String::from("")},
        Token{name: String::from("AE"), value: String::from("")}
    ];

    let result = parser(Node {
        kind: String::from("root"),
        value: String::from(""),
        children: vec![]
    }, &tokens);

    let expected = Node {
        kind: String::from("root"),
        value: String::from(""),
        children: vec![
            Node {
                kind: String::from("array"),
                value: String::from(""),
                children: vec![
                    Node{
                        kind: String::from("undefined"),
                        value: String::from(""),
                        children: vec![]
                    },
                    Node {
                        kind: String::from("array"),
                        value: String::from(""),
                        children: vec![
                            Node {
                                kind: String::from("object"),
                                value: String::from(""),
                                children: vec![
                                    Node {
                                        kind: String::from("pair"),
                                        value: String::from("\"hello\""),
                                        children: vec![
                                            Node {
                                                kind: String::from("array"),
                                                value: String::from(""),
                                                children: vec![
                                                    Node {
                                                        kind: String::from("bigint"),
                                                        value: String::from("9007199254740991n"),
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
        Token{name: String::from("AS"), value: String::from("")},
        Token{name: String::from("AE"), value: String::from("")}
    ];

    let result = parser(Node {
        kind: String::from("root"),
        value: String::from(""),
        children: vec![]
    }, &tokens);

    let expected = Node {
        kind: String::from("root"),
        value: String::from(""),
        children: vec![
            Node {
                kind: String::from("array"),
                value: String::from(""),
                children: vec![]
            }
        ]
    };

    assert_eq!(result, expected);
}
#[test]
fn empty_nested_array() {
    let tokens = vec![
        Token{name: String::from("AS"), value: String::from("")},
        Token{name: String::from("AS"), value: String::from("")},
        Token{name: String::from("AE"), value: String::from("")},
        Token{name: String::from("AE"), value: String::from("")}
    ];

    let result = parser(Node {
        kind: String::from("root"),
        value: String::from(""),
        children: vec![]
    }, &tokens);

    let expected = Node {
        kind: String::from("root"),
        value: String::from(""),
        children: vec![
            Node{
                kind: String::from("array"),
                value: String::from(""),
                children: vec![
                    Node {
                        kind: String::from("array"),
                        value: String::from(""),
                        children: vec![]
                    }
                ]
            }
        ]
    };

    assert_eq!(result, expected);
}