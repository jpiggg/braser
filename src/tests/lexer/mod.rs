use pretty_assertions::assert_eq;
use crate::lexer::run as lexer;
use crate::shared::tokens::Token;

#[test]
fn test_flatten_object() {
    let src: &str = r#"a${3$"foo####\" bar\"":3$"test",3$"hi":4$100500,3$"date":7$2023-08-01T14:32:01.624Z,3$"myFn":9$function my_fn(a, b) {
        return a + b;
    }$,3$"undefined":0$,3$"nan":6$,3$"null":1$}"#;
    let tokens = lexer(src);
    let expected = vec![
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

    for (i, token) in tokens.iter().enumerate() {
       let expected_token = expected[i];

       assert_eq!(*token, expected_token);
    }
}
#[test]
fn test_nested_object() {
    let src: &str = r#"a${3$"foo":a${3$"hello":3$"world"}}"#;
    let tokens = lexer(src);
    let expected = vec![
        Token{ name: "OS", value: ""},
        Token{ name: "ST", value: "\"foo\""},
        Token{ name: "KT", value: ""},
        Token{ name: "OS", value: ""},
        Token{ name: "ST", value:  "\"hello\""},
        Token{ name: "KT", value:  ""},
        Token{ name: "ST", value:  "\"world\""},
        Token{ name: "OE", value:  ""},
        Token{ name: "OE", value:  ""}
    ];

    for (i, token) in tokens.iter().enumerate() {
        let expected_token = expected[i];
 
        assert_eq!(*token, expected_token);
     }
}
#[test]
fn test_empty_object() {
    let src: &str = r#"a${}"#;
    let tokens = lexer(src);
    let expected = vec![Token{name: "OS", value: ""}, Token{name: "OE", value: ""}];

    for (i, token) in tokens.iter().enumerate() {
        let expected_token = expected[i];
 
        assert_eq!(*token, expected_token);
     }
}
#[test]
fn test_flatten_array() {
    let src: &str = r#"b$[3$"foo", 3$"bar"]"#;
    let tokens = lexer(src);
    let expected = vec![
        Token{name: "AS", value: ""},
        Token{name: "ST", value: "\"foo\""},
        Token{name: "LT", value: ""},
        Token{name: "ST", value: "\"bar\""},
        Token{name: "AE", value: ""}
    ];

    for (i, token) in tokens.iter().enumerate() {
        let expected_token = expected[i];
 
        assert_eq!(*token, expected_token);
     }
}
#[test]
fn test_nested_array() {
    let src: &str = r#"b$[3$"foo", 3$"bar", b$[3$"hello", 3$"world"]]"#;
    let tokens = lexer(src);
    let expected = vec![
        Token{name: "AS", value: ""},
        Token{name: "ST", value: "\"foo\""},
        Token{name: "LT", value: ""},
        Token{name: "ST", value: "\"bar\""},
        Token{name: "LT", value: ""},
        Token{name: "AS", value: ""},
        Token{name: "ST", value: "\"hello\""},
        Token{name: "LT", value: ""},
        Token{name: "ST", value: "\"world\""},
        Token{name: "AE", value: ""},
        Token{name: "AE", value: ""}
    ];

    for (i, token) in tokens.iter().enumerate() {
        let expected_token = expected[i];
 
        assert_eq!(*token, expected_token);
     }
}
#[test]
fn test_empty_array() {
    let src: &str = r#"b$[]"#;
    let tokens = lexer(src);
    let expected = vec![Token{name: "AS", value: ""}, Token{name: "AE", value: ""}];

    for (i, token) in tokens.iter().enumerate() {
        let expected_token = expected[i];
 
        assert_eq!(*token, expected_token);
     }
}
#[test]
fn test_strings() {
    let src: &str = r###"b$[3$"foo####\"## bar\"\\\"", 3$"foo\
    \\" '\"' bar\"\"", 3$"foo##############################################################\" bar\"\"# ######", 3$"foo####\" bar\\\\\\\\\\\\\\\\\\\\\\\\\\\ \"\\\\\\\\\\\\\\\\\\\\\ "\\\"",
    3$"hello",
    3$",", 3$":", 3$".", 3$"", 3$",.\\ ", 3$"/;\\\"'", 3$"?()", 3$"-!@#$4", 3$"3$", \"…:[ ]{}‘", 3$"`~^*&%>_<+=e|©●≈
    ```
        hello world
    ```
    ", 3$"
    ",
    3$"\"\"", 3$"⟨ ⟩¤⌀÷℮« »¡¿¶®§℗™⁀℠±‰№× lol"]"###;
    let tokens = lexer(src);
    let expected = vec![
        Token{name: "AS", value: ""},
        Token{name: "ST", value: "\"foo####\\\"## bar\\\"\\\\\\\"\""},
        Token{name: "LT", value: ""},
        Token{name: "ST", value: "\"foo\\\n    \\\\\" '\\\"' bar\\\"\\\"\""},
        Token{name: "LT", value: ""},
        Token{name: "ST", value: "\"foo##############################################################\\\" bar\\\"\\\"# ######\""},
        Token{name: "LT", value: ""},
        Token{name: "ST", value: "\"foo####\\\" bar\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\ \\\"\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\ \""},
        Token{name: "LT", value: ""},
        Token{name: "ST", value: "\"hello\""},
        Token{name: "LT", value: ""},
        Token{name: "ST", value: "\",\""},
        Token{name: "LT", value: ""},
        Token{name: "ST", value: "\":\""},
        Token{name: "LT", value: ""},
        Token{name: "ST", value: "\".\""},
        Token{name: "LT", value: ""},
        Token{name: "ST", value: "\"\""},
        Token{name: "LT", value: ""},
        Token{name: "ST", value: "\",.\\\\ \""},
        Token{name: "LT", value: ""},
        Token{name: "ST", value: "\"/;\\\\\\\"'\""},
        Token{name: "LT", value: ""},
        Token{name: "ST", value: "\"?()\""},
        Token{name: "LT", value: ""},
        Token{name: "ST", value: "\"-!@#$4\""},
        Token{name: "LT", value: ""},
        Token{name: "ST", value: "\"3$\", \\\"…:[ ]{}‘\""},
        Token{name: "LT", value: ""},
        Token{name: "ST", value: "\"`~^*&%>_<+=e|©●≈\n    ```\n        hello world\n    ```\n    \""},
        Token{name: "LT", value: ""},
        Token{name: "ST", value: "\"\n    \""},
        Token{name: "LT", value: ""},
        Token{name: "ST", value: "\"\\\"\\\"\""},
        Token{name: "LT", value: ""},
        Token{name: "ST", value: "\"⟨ ⟩¤⌀÷℮« »¡¿¶®§℗™⁀℠±‰№× lol\""},
        Token{name: "AE", value: ""}
        
    ];

    for (i, token) in tokens.iter().enumerate() {
        let expected_token = expected[i];
 
        assert_eq!(*token, expected_token);
    }
}
#[test]
fn test_numbers() {
    let src: &str = r#"b$[4$4.,4$5.5, 4$123, 4$123.0, 4$.5, 4$3.2e6, 4$314e-2, 4$0.0314E+2, 4$7.]"#;
    let tokens = lexer(src);
    let expected = vec![
        Token{name: "AS", value: ""},
        Token{name: "NU", value: "4."},
        Token{name: "LT", value: ""},
        Token{name: "NU", value: "5.5"},
        Token{name: "LT", value: ""},
        Token{name: "NU", value: "123"},
        Token{name: "LT", value: ""},
        Token{name: "NU", value: "123.0"},
        Token{name: "LT", value: ""},
        Token{name: "NU", value: ".5"},
        Token{name: "LT", value: ""},
        Token{name: "NU", value: "3.2e6"},
        Token{name: "LT", value: ""},
        Token{name: "NU", value: "314e-2"},
        Token{name: "LT", value: ""},
        Token{name: "NU", value: "0.0314E+2"},
        Token{name: "LT", value: ""},
        Token{name: "NU", value: "7."},
        Token{name: "AE", value: ""}
    ];

    for (i, token) in tokens.iter().enumerate() {
        let expected_token = expected[i];
 
        assert_eq!(*token, expected_token);
    }
}
#[test]
fn test_big_int() {
    let src: &str = r#"8$9007199254740991"#;
    let tokens = lexer(src);
    let expected = vec![Token{name: "BI", value: "9007199254740991"}];

    assert_eq!(tokens.len(), expected.len());
    assert_eq!(expected[0], tokens[0]);
}
#[test]
fn test_infinity() {
    let src: &str = r#"5$-1"#;
    let tokens = lexer(src);
    let expected = vec![Token{name: "IN", value: "-1"}];

    assert_eq!(tokens.len(), expected.len());
    assert_eq!(expected[0], tokens[0]);
}
#[test]
fn test_nan() {
    let src: &str = r#"6$"#;
    let tokens = lexer(src);
    let expected = vec![Token{name: "NA", value: ""}];

    assert_eq!(tokens.len(), expected.len());
    assert_eq!(expected[0], tokens[0]);
}
#[test]
fn test_undefined() {
    let src: &str = r#"0$"#;
    let tokens = lexer(src);
    let expected = vec![Token{name: "UN", value: ""}];

    assert_eq!(tokens.len(), expected.len());
    assert_eq!(expected[0], tokens[0]);
}
#[test]
fn test_null() {
    let src: &str = r#"1$"#;
    let tokens = lexer(src);
    let expected = vec![Token{name: "NL", value: ""}];

    assert_eq!(tokens.len(), expected.len());
    assert_eq!(expected[0], tokens[0]);
}
#[test]
fn test_boolean_true() {
    let src: &str = r#"2$1"#;
let tokens = lexer(src);
let expected = vec![Token{name: "BO", value: "1"}];

assert_eq!(tokens.len(), expected.len());
assert_eq!(expected[0], tokens[0]);
}
#[test]
fn test_boolean_false() {
    let src: &str = r#"2$0"#;
    let tokens = lexer(src);
    let expected = vec![Token{name: "BO", value: "0"}];

    assert_eq!(tokens.len(), expected.len());
    assert_eq!(expected[0], tokens[0]);
}
#[test]
fn test_date() {
    let src: &str = r#"7$2023-08-01T14:32:01.624Z"#;
    let tokens = lexer(src);
    let expected = vec![Token{name: "DT", value: "2023-08-01T14:32:01.624Z"}];
    assert_eq!(tokens.len(), expected.len());
    assert_eq!(expected[0], tokens[0]);
}

#[test]
fn test_functions() {
    let src: &str = r#"a${3$"foo":9$function () { return URL + '/movies'; }$,3$"bar":9$function test(fn) {
        if (typeof fn === "function") {
            return true;
        }
        return false;
    }$,3$"myFn":9$function my_fn(a, b) {
        return a + b;
    }$,3$"closure":9$function (isLoading) {
        var foo = "bar";
        return function (testParam) {
            console.log(isLoading, foo, testParam);
        };
    }$}"#;
    let tokens = lexer(src);
    let expected = vec![
        Token{name: "OS", value: ""},
        Token{name: "ST", value: "\"foo\""},
        Token{name: "KT", value: ""},
        Token{name: "FU", value: "function () { return URL + '/movies'; }"},
        Token{name: "LT", value: ""},
        Token{name: "ST", value: "\"bar\""},
        Token{name: "KT", value: ""},
        Token{name: "FU", value: "function test(fn) {\n        if (typeof fn === \"function\") {\n            return true;\n        }\n        return false;\n    }"},
        Token{name: "LT", value: ""},
        Token{name: "ST", value: "\"myFn\"" },
        Token{name: "KT", value: ""},
        Token{name: "FU", value: "function my_fn(a, b) {\n        return a + b;\n    }"},
        Token{name: "LT", value: ""},
        Token{name: "ST", value: "\"closure\""},
        Token{name: "KT", value: ""},
        Token{name: "FU", value: "function (isLoading) {\n        var foo = \"bar\";\n        return function (testParam) {\n            console.log(isLoading, foo, testParam);\n        };\n    }"}, 
        Token{name: "OE", value: ""}
    ];

    for (i, token) in tokens.iter().enumerate() {
        let expected_token = expected[i];
 
        assert_eq!(*token, expected_token);
     }
}