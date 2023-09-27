use pretty_assertions::assert_eq;
use crate::decode::parser::run as parser;
use crate::shared::{tokens::Token, Node};

#[test]
fn test_flatten_object() {
    let tokens = vec![
        Token { name: String::from("OS"), value: String::from("") , start: 0},
        Token { name: String::from("ST"), value: String::from("\"foo####\\\" bar\\\"\"") , start: 0},
        Token { name: String::from("KT"), value: String::from("") , start: 0},
        Token { name: String::from("ST"), value: String::from("\"test\"") , start: 0},
        Token { name: String::from("LT"), value: String::from("") , start: 0},
        Token { name: String::from("ST"), value: String::from("\"hi\"") , start: 0},
        Token { name: String::from("KT"), value: String::from("") , start: 0},
        Token { name: String::from("NU"), value: String::from("100500") , start: 0},
        Token { name: String::from("LT"), value: String::from("") , start: 0},
        Token { name: String::from("ST"), value: String::from("\"date\"") , start: 0},
        Token { name: String::from("KT"), value: String::from("") , start: 0},
        Token { name: String::from("DT"), value: String::from("2023-08-01T14:32:01.624Z") , start: 0},
        Token { name: String::from("LT"), value: String::from("") , start: 0},
        Token { name: String::from("ST"), value: String::from("\"myFn\"") , start: 0},
        Token { name: String::from("KT"), value: String::from("") , start: 0},
        Token { name: String::from("FU"), value: String::from("[name=my_fn] function my_fn(a, b) {\n        return a + b;\n    }") , start: 0},
        Token { name: String::from("LT"), value: String::from("") , start: 0}, 
        Token { name: String::from("ST"), value: String::from("\"undefined\"") , start: 0},
        Token { name: String::from("KT"), value: String::from("") , start: 0},
        Token { name: String::from("UN"), value: String::from("") , start: 0},
        Token { name: String::from("LT"), value: String::from("") , start: 0},
        Token { name: String::from("ST"), value: String::from("\"nan\"") , start: 0},
        Token { name: String::from("KT"), value: String::from("") , start: 0},
        Token { name: String::from("NA"), value: String::from("") , start: 0},
        Token { name: String::from("LT"), value: String::from("") , start: 0},
        Token { name: String::from("ST"), value: String::from("\"null\"") , start: 0},
        Token { name: String::from("KT"), value: String::from("") , start: 0},
        Token { name: String::from("NL"), value: String::from("") , start: 0},
        Token { name: String::from("OE"), value: String::from(""), start: 0 }
    ];

    let result = parser(Node {
        kind: String::from("root"),
        value: String::from(""),
        children: vec![]
    }, &tokens).unwrap();
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
        Token{ name: String::from("OS"), value: String::from(""), start: 0},
        Token{ name: String::from("ST"), value: String::from("\"foo\""), start: 0},
        Token{ name: String::from("KT"), value: String::from(""), start: 0},
        Token{ name: String::from("OS"), value: String::from(""), start: 0},
        Token{ name: String::from("ST"), value: String::from("\"hello\""), start: 0},
        Token{ name: String::from("KT"), value: String::from(""), start: 0},
        Token{ name: String::from("ST"), value: String::from("\"world\""), start: 0},
        Token{ name: String::from("OE"), value: String::from(""), start: 0},
        Token{ name: String::from("LT"), value: String::from(""), start: 0},
        Token{ name: String::from("ST"), value: String::from("\"second\""), start: 0},
        Token{ name: String::from("KT"), value: String::from(""), start: 0},
        Token{ name: String::from("NU"), value: String::from("12345"), start: 0},
        Token{ name: String::from("OE"), value: String::from(""), start: 0}
    ];

    let result = parser(Node {
        kind: String::from("root"),
        value: String::from(""),
        children: vec![]
    }, &tokens).unwrap();

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
        Token{name: String::from("OS"), value: String::from(""), start: 0},
        Token{name: String::from("OE"), value: String::from(""), start: 0}
    ];

    let result = parser(Node {
        kind: String::from("root"),
        value: String::from(""),
        children: vec![]
    }, &tokens).unwrap();

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
        Token{name: String::from("AS"), value: String::from(""), start: 0},
        Token{name: String::from("ST"), value: String::from("\"foo####\\\" bar\\\"\""), start: 0},
        Token{name: String::from("LT"), value: String::from(""), start: 0},
        Token{name: String::from("OS"), value: String::from(""), start: 0},
        Token{name: String::from("ST"), value: String::from("\"hi\""), start: 0},
        Token{name: String::from("KT"), value: String::from(""), start: 0},
        Token{name: String::from("DT"), value: String::from("2023-08-01T14:32:01.624Z"), start: 0},
        Token{name: String::from("LT"), value: String::from(""), start: 0},
        Token{name: String::from("ST"), value: String::from("\"test\""), start: 0},
        Token{name: String::from("KT"), value: String::from(""), start: 0},
        Token{name: String::from("NU"), value: String::from("123"), start: 0},
        Token{name: String::from("OE"), value: String::from(""), start: 0},
        Token{name: String::from("LT"), value: String::from(""), start: 0},
        Token{name: String::from("NL"), value: String::from(""), start: 0},
        Token{name: String::from("AE"), value: String::from(""), start: 0},
    ];

    let result = parser(Node {
        kind: String::from("root"),
        value: String::from(""),
        children: vec![]
    }, &tokens).unwrap();

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
        Token{name: String::from("AS"), value: String::from(""), start: 0},
        Token{name: String::from("UN"), value: String::from(""), start: 0},
        Token{name: String::from("LT"), value: String::from(""), start: 0},
        Token{name: String::from("AS"), value: String::from(""), start: 0},
        Token{name: String::from("OS"), value: String::from(""), start: 0},
        Token{name: String::from("ST"), value: String::from("\"hello\""), start: 0},
        Token{name: String::from("KT"), value: String::from(""), start: 0},
        Token{name: String::from("AS"), value: String::from(""), start: 0},
        Token{name: String::from("BI"), value: String::from("9007199254740991n"), start: 0},
        Token{name: String::from("AE"), value: String::from(""), start: 0},
        Token{name: String::from("OE"), value: String::from(""), start: 0},
        Token{name: String::from("AE"), value: String::from(""), start: 0},
        Token{name: String::from("AE"), value: String::from(""), start: 0}
    ];

    let result = parser(Node {
        kind: String::from("root"),
        value: String::from(""),
        children: vec![]
    }, &tokens).unwrap();

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
        Token{name: String::from("AS"), value: String::from(""), start: 0},
        Token{name: String::from("AE"), value: String::from(""), start: 0}
    ];

    let result = parser(Node {
        kind: String::from("root"),
        value: String::from(""),
        children: vec![]
    }, &tokens).unwrap();

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
        Token{name: String::from("AS"), value: String::from(""), start: 0},
        Token{name: String::from("AS"), value: String::from(""), start: 0},
        Token{name: String::from("AE"), value: String::from(""), start: 0},
        Token{name: String::from("AE"), value: String::from(""), start: 0}
    ];

    let result = parser(Node {
        kind: String::from("root"),
        value: String::from(""),
        children: vec![]
    }, &tokens).unwrap();

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