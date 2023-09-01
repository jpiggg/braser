use pretty_assertions::assert_eq;
use crate::decode::lexer::run as lexer;
use crate::shared::tokens::Token;

#[test]
fn test_flatten_object() {
    let src: &str = r#"a${3$"foo####\" bar\"":3$"test",3$"hi":4$100500,3$"date":7$2023-08-01T14:32:01.624Z,3$"myFn":9$[name=my_fn] function my_fn(a, b) {
        return a + b;
    }$,3$"undefined": 0$,3$"nan":6$,3$"null":1$}"#;
    let tokens = lexer(src);
    let expected = vec![
        Token { name: String::from("OS"), value: String::from("" )},
        Token { name: String::from("ST"), value: String::from("\"foo####\\\" bar\\\"\"" )},
        Token { name: String::from("KT"), value: String::from("" )},
        Token { name: String::from("ST"), value: String::from("\"test\"" )},
        Token { name: String::from("LT"), value: String::from("" )},
        Token { name: String::from("ST"), value: String::from("\"hi\"" )},
        Token { name: String::from("KT"), value: String::from("" )},
        Token { name: String::from("NU"), value: String::from("100500" )},
        Token { name: String::from("LT"), value: String::from("" )},
        Token { name: String::from("ST"), value: String::from("\"date\"" )},
        Token { name: String::from("KT"), value: String::from("" )},
        Token { name: String::from("DT"), value: String::from("2023-08-01T14:32:01.624Z" )},
        Token { name: String::from("LT"), value: String::from("" )},
        Token { name: String::from("ST"), value: String::from("\"myFn\"" )},
        Token { name: String::from("KT"), value: String::from("" )},
        Token { name: String::from("FU"), value: String::from("[name=my_fn] function my_fn(a, b) {\n        return a + b;\n    }" )},
        Token { name: String::from("LT"), value: String::from("") }, 
        Token { name: String::from("ST"), value: String::from("\"undefined\"" )},
        Token { name: String::from("KT"), value: String::from("" )},
        Token { name: String::from("UN"), value: String::from("" )},
        Token { name: String::from("LT"), value: String::from("" )},
        Token { name: String::from("ST"), value: String::from("\"nan\"" )},
        Token { name: String::from("KT"), value: String::from("" )},
        Token { name: String::from("NA"), value: String::from("" )},
        Token { name: String::from("LT"), value: String::from("" )},
        Token { name: String::from("ST"), value: String::from("\"null\"" )},
        Token { name: String::from("KT"), value: String::from("" )},
        Token { name: String::from("NL"), value: String::from("" )},
        Token { name: String::from("OE"), value: String::from("") }
    ];

    for (i, token) in tokens.iter().enumerate() {
       let expected_token = &expected[i];

       assert_eq!(*token, *expected_token);
    }
}
#[test]
fn test_nested_object() {
    let src: &str = r#"a${3$"foo":a${3$"hello":3$"world"}}"#;
    let tokens = lexer(src);
    let expected = vec![
        Token{ name: String::from("OS"), value: String::from("")},
        Token{ name: String::from("ST"), value: String::from("\"foo\"")},
        Token{ name: String::from("KT"), value: String::from("")},
        Token{ name: String::from("OS"), value: String::from("")},
        Token{ name: String::from("ST"), value: String::from( "\"hello\"")},
        Token{ name: String::from("KT"), value: String::from( "")},
        Token{ name: String::from("ST"), value: String::from( "\"world\"")},
        Token{ name: String::from("OE"), value: String::from( "")},
        Token{ name: String::from("OE"), value: String::from( "")}
    ];

    for (i, token) in tokens.iter().enumerate() {
        let expected_token = &expected[i];
 
        assert_eq!(*token, *expected_token);
     }
}
#[test]
fn test_empty_object() {
    let src: &str = r#"a${}"#;
    let tokens = lexer(src);
    let expected = vec![Token{name: String::from("OS"), value: String::from("")}, Token{name: String::from("OE"), value: String::from("")}];

    for (i, token) in tokens.iter().enumerate() {
        let expected_token = &expected[i];
 
        assert_eq!(*token, *expected_token);
     }
}
#[test]
fn test_flatten_array() {
    let src: &str = r#"b$[3$"foo", 3$"bar"]"#;
    let tokens = lexer(src);
    let expected = vec![
        Token{name: String::from("AS"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"foo\"")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"bar\"")},
        Token{name: String::from("AE"), value: String::from("")}
    ];

    for (i, token) in tokens.iter().enumerate() {
        let expected_token = &expected[i];
 
        assert_eq!(*token, *expected_token);
     }
}
#[test]
fn test_nested_array() {
    let src: &str = r#"b$[3$"foo", 3$"bar", b$[3$"hello", 3$"world"]]"#;
    let tokens = lexer(src);
    let expected = vec![
        Token{name: String::from("AS"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"foo\"")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"bar\"")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("AS"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"hello\"")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"world\"")},
        Token{name: String::from("AE"), value: String::from("")},
        Token{name: String::from("AE"), value: String::from("")}
    ];

    for (i, token) in tokens.iter().enumerate() {
        let expected_token = &expected[i];
 
        assert_eq!(*token, *expected_token);
     }
}
#[test]
fn test_empty_array() {
    let src: &str = r#"b$[]"#;
    let tokens = lexer(src);
    let expected = vec![Token{name: String::from("AS"), value: String::from("")}, Token{name: String::from("AE"), value: String::from("")}];

    for (i, token) in tokens.iter().enumerate() {
        let expected_token = &expected[i];
 
        assert_eq!(*token, *expected_token);
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
        Token{name: String::from("AS"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"foo####\\\"## bar\\\"\\\\\\\"\"")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"foo\\\n    \\\\\" '\\\"' bar\\\"\\\"\"")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"foo##############################################################\\\" bar\\\"\\\"# ######\"")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"foo####\\\" bar\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\ \\\"\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\ \"")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"hello\"")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\",\"")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\":\"")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\".\"")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"\"")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\",.\\\\ \"")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"/;\\\\\\\"'\"")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"?()\"")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"-!@#$4\"")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"3$\", \\\"…:[ ]{}‘\"")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"`~^*&%>_<+=e|©●≈\n    ```\n        hello world\n    ```\n    \"")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"\n    \"")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"\\\"\\\"\"")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"⟨ ⟩¤⌀÷℮« »¡¿¶®§℗™⁀℠±‰№× lol\"")},
        Token{name: String::from("AE"), value: String::from("")}
        
    ];

    for (i, token) in tokens.iter().enumerate() {
        let expected_token = &expected[i];
 
        assert_eq!(*token, *expected_token);
    }
}
#[test]
fn test_numbers() {
    let src: &str = r#"b$[4$4.,4$5.5, 4$123, 4$123.0, 4$.5, 4$3.2e6, 4$314e-2, 4$0.0314E+2, 4$7.]"#;
    let tokens = lexer(src);
    let expected = vec![
        Token{name: String::from("AS"), value: String::from("")},
        Token{name: String::from("NU"), value: String::from("4.")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("NU"), value: String::from("5.5")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("NU"), value: String::from("123")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("NU"), value: String::from("123.0")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("NU"), value: String::from(".5")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("NU"), value: String::from("3.2e6")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("NU"), value: String::from("314e-2")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("NU"), value: String::from("0.0314E+2")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("NU"), value: String::from("7.")},
        Token{name: String::from("AE"), value: String::from("")}
    ];

    for (i, token) in tokens.iter().enumerate() {
        let expected_token = &expected[i];
 
        assert_eq!(*token, *expected_token);
    }
}
#[test]
fn test_big_int() {
    let src: &str = r#"8$9007199254740991"#;
    let tokens = lexer(src);
    let expected = vec![Token{name: String::from("BI"), value: String::from("9007199254740991")}];

    assert_eq!(tokens.len(), expected.len());
    assert_eq!(expected[0], tokens[0]);
}
#[test]
fn test_infinity() {
    let src: &str = r#"5$-1"#;
    let tokens = lexer(src);
    let expected = vec![Token{name: String::from("IN"), value: String::from("-1")}];

    assert_eq!(tokens.len(), expected.len());
    assert_eq!(expected[0], tokens[0]);
}
#[test]
fn test_nan() {
    let src: &str = r#"6$"#;
    let tokens = lexer(src);
    let expected = vec![Token{name: String::from("NA"), value: String::from("")}];

    assert_eq!(tokens.len(), expected.len());
    assert_eq!(expected[0], tokens[0]);
}
#[test]
fn test_undefined() {
    let src: &str = r#"0$"#;
    let tokens = lexer(src);
    let expected = vec![Token{name: String::from("UN"), value: String::from("")}];

    assert_eq!(tokens.len(), expected.len());
    assert_eq!(expected[0], tokens[0]);
}
#[test]
fn test_null() {
    let src: &str = r#"1$"#;
    let tokens = lexer(src);
    let expected = vec![Token{name: String::from("NL"), value: String::from("")}];

    assert_eq!(tokens.len(), expected.len());
    assert_eq!(expected[0], tokens[0]);
}
#[test]
fn test_boolean_true() {
    let src: &str = r#"2$1"#;
let tokens = lexer(src);
let expected = vec![Token{name: String::from("BO"), value: String::from("1")}];

assert_eq!(tokens.len(), expected.len());
assert_eq!(expected[0], tokens[0]);
}
#[test]
fn test_boolean_false() {
    let src: &str = r#"2$0"#;
    let tokens = lexer(src);
    let expected = vec![Token{name: String::from("BO"), value: String::from("0")}];

    assert_eq!(tokens.len(), expected.len());
    assert_eq!(expected[0], tokens[0]);
}
#[test]
fn test_date() {
    let src: &str = r#"7$2023-08-01T14:32:01.624Z"#;
    let tokens = lexer(src);
    let expected = vec![Token{name: String::from("DT"), value: String::from("2023-08-01T14:32:01.624Z")}];
    assert_eq!(tokens.len(), expected.len());
    assert_eq!(expected[0], tokens[0]);
}

#[test]
fn test_functions() {
    let src: &str = r#"a${3$"foo":9$[name=test] function test() { return URL + '/movies'; }$,3$"bar":9$[name=test] function test(fn) {
        if (typeof fn === "function") {
            return true;
        }
        return false;
    }$,3$"myFn":9$[name=my_fn] function my_fn(a, b) {
        return a + b;
    }$,3$"closure":9$[name=closure] function (isLoading) {
        var foo = "bar";
        return function (testParam) {
            console.log(isLoading, foo, testParam);
        };
    }$}"#;
    let tokens = lexer(src);
    let expected = vec![
        Token{name: String::from("OS"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"foo\"")},
        Token{name: String::from("KT"), value: String::from("")},
        Token{name: String::from("FU"), value: String::from("[name=test] function test() { return URL + '/movies'; }")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"bar\"")},
        Token{name: String::from("KT"), value: String::from("")},
        Token{name: String::from("FU"), value: String::from("[name=test] function test(fn) {\n        if (typeof fn === \"function\") {\n            return true;\n        }\n        return false;\n    }")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"myFn\"" )},
        Token{name: String::from("KT"), value: String::from("")},
        Token{name: String::from("FU"), value: String::from("[name=my_fn] function my_fn(a, b) {\n        return a + b;\n    }")},
        Token{name: String::from("LT"), value: String::from("")},
        Token{name: String::from("ST"), value: String::from("\"closure\"")},
        Token{name: String::from("KT"), value: String::from("")},
        Token{name: String::from("FU"), value: String::from("[name=closure] function (isLoading) {\n        var foo = \"bar\";\n        return function (testParam) {\n            console.log(isLoading, foo, testParam);\n        };\n    }")}, 
        Token{name: String::from("OE"), value: String::from("")}
    ];

    for (i, token) in tokens.iter().enumerate() {
        let expected_token = &expected[i];
 
        assert_eq!(*token, *expected_token);
     }
}