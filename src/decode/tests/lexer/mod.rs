use pretty_assertions::assert_eq;
use crate::decode::lexer::run as lexer;
use crate::shared::tokens::Token;

#[test]
fn test_flatten_object() {
    let src: &str = r#"a${3$"foo####\" bar\"":3$"test",3$"hi":4$100500,3$"date":7$2023-08-01T14:32:01.624Z,3$"myFn":9$[name=my_fn] function my_fn(a, b) {
        return a + b;
    }$,3$"undefined": 0$,3$"nan":6$,3$"null":1$}"#;
    let tokens = lexer(src).unwrap();
    let expected = vec![
        Token { name: String::from("OS"), value: String::from("" ), start: 0},
        Token { name: String::from("ST"), value: String::from("\"foo####\\\" bar\\\"\"" ), start: 3},
        Token { name: String::from("KT"), value: String::from("" ), start: 22},
        Token { name: String::from("ST"), value: String::from("\"test\"" ), start: 23},
        Token { name: String::from("LT"), value: String::from("" ), start: 31},
        Token { name: String::from("ST"), value: String::from("\"hi\"" ), start: 32},
        Token { name: String::from("KT"), value: String::from("" ), start: 38},
        Token { name: String::from("NU"), value: String::from("100500" ), start: 39},
        Token { name: String::from("LT"), value: String::from("" ), start: 47},
        Token { name: String::from("ST"), value: String::from("\"date\"" ), start: 48},
        Token { name: String::from("KT"), value: String::from("" ), start: 56},
        Token { name: String::from("DT"), value: String::from("2023-08-01T14:32:01.624Z" ), start: 57},
        Token { name: String::from("LT"), value: String::from("" ), start: 83},
        Token { name: String::from("ST"), value: String::from("\"myFn\"" ), start: 84},
        Token { name: String::from("KT"), value: String::from("" ), start: 92},
        Token { name: String::from("FU"), value: String::from("[name=my_fn] function my_fn(a, b) {\n        return a + b;\n    }" ), start: 93},
        Token { name: String::from("LT"), value: String::from(""), start: 159}, 
        Token { name: String::from("ST"), value: String::from("\"undefined\"" ), start: 160},
        Token { name: String::from("KT"), value: String::from("" ), start: 173},
        Token { name: String::from("UN"), value: String::from("" ), start: 175},
        Token { name: String::from("LT"), value: String::from("" ), start: 177},
        Token { name: String::from("ST"), value: String::from("\"nan\"" ), start: 178},
        Token { name: String::from("KT"), value: String::from("" ), start: 185},
        Token { name: String::from("NA"), value: String::from("" ), start: 186},
        Token { name: String::from("LT"), value: String::from("" ), start: 188},
        Token { name: String::from("ST"), value: String::from("\"null\"" ), start: 189},
        Token { name: String::from("KT"), value: String::from("" ), start: 197},
        Token { name: String::from("NL"), value: String::from("" ), start: 198},
        Token { name: String::from("OE"), value: String::from(""), start: 200 }
    ];

    for (i, token) in tokens.iter().enumerate() {
       let expected_token = &expected[i];

       assert_eq!(*expected_token, *token);
    }
}
#[test]
fn test_nested_object() {
    let src: &str = r#"a${3$"foo":a${3$"hello":3$"world"}}"#;
    let tokens = lexer(src).unwrap();
    let expected = vec![
        Token{ name: String::from("OS"), value: String::from(""), start: 0},
        Token{ name: String::from("ST"), value: String::from("\"foo\""), start: 3},
        Token{ name: String::from("KT"), value: String::from(""), start: 10},
        Token{ name: String::from("OS"), value: String::from(""), start: 11},
        Token{ name: String::from("ST"), value: String::from( "\"hello\""), start: 14},
        Token{ name: String::from("KT"), value: String::from( ""), start: 23},
        Token{ name: String::from("ST"), value: String::from( "\"world\""), start: 24},
        Token{ name: String::from("OE"), value: String::from( ""), start: 33},
        Token{ name: String::from("OE"), value: String::from( ""), start: 34}
    ];

    for (i, token) in tokens.iter().enumerate() {
        let expected_token = &expected[i];
 
        assert_eq!(*expected_token, *token);
     }
}
#[test]
fn test_empty_object() {
    let src: &str = r#"a${}"#;
    let tokens = lexer(src).unwrap();
    let expected = vec![
            Token{name: String::from("OS"), value: String::from(""), start: 0},
            Token{name: String::from("OE"), value: String::from(""), start: 3}
        ];

    for (i, token) in tokens.iter().enumerate() {
        let expected_token = &expected[i];
 
        assert_eq!(*expected_token, *token);
     }
}
#[test]
fn test_flatten_array() {
    let src: &str = r#"b$[3$"foo", 3$"bar"]"#;
    let tokens = lexer(src).unwrap();
    let expected = vec![
        Token{name: String::from("AS"), value: String::from(""), start: 0},
        Token{name: String::from("ST"), value: String::from("\"foo\""), start: 3},
        Token{name: String::from("LT"), value: String::from(""), start: 10},
        Token{name: String::from("ST"), value: String::from("\"bar\""), start: 12},
        Token{name: String::from("AE"), value: String::from(""), start: 19}
    ];

    for (i, token) in tokens.iter().enumerate() {
        let expected_token = &expected[i];
 
        assert_eq!(*expected_token, *token);
     }
}
#[test]
fn test_nested_array() {
    let src: &str = r#"b$[3$"foo", 3$"bar", b$[3$"hello", 3$"world"]]"#;
    let tokens = lexer(src).unwrap();
    let expected = vec![
        Token{name: String::from("AS"), value: String::from(""), start: 0},
        Token{name: String::from("ST"), value: String::from("\"foo\""), start: 3},
        Token{name: String::from("LT"), value: String::from(""), start: 10},
        Token{name: String::from("ST"), value: String::from("\"bar\""), start: 12},
        Token{name: String::from("LT"), value: String::from(""), start: 19},
        Token{name: String::from("AS"), value: String::from(""), start: 21},
        Token{name: String::from("ST"), value: String::from("\"hello\""), start: 24},
        Token{name: String::from("LT"), value: String::from(""), start: 33},
        Token{name: String::from("ST"), value: String::from("\"world\""), start: 35},
        Token{name: String::from("AE"), value: String::from(""), start: 44},
        Token{name: String::from("AE"), value: String::from(""), start: 45}
    ];

    for (i, token) in tokens.iter().enumerate() {
        let expected_token = &expected[i];
 
        assert_eq!(*expected_token, *token);
     }
}
#[test]
fn test_empty_array() {
    let src: &str = r#"b$[]"#;
    let tokens = lexer(src).unwrap();
    let expected = vec![
        Token{name: String::from("AS"), value: String::from(""), start: 0},
        Token{name: String::from("AE"), value: String::from(""), start: 3}
        ];

    for (i, token) in tokens.iter().enumerate() {
        let expected_token = &expected[i];
 
        assert_eq!(*expected_token, *token);
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
    let tokens = lexer(src).unwrap();
    let expected = vec![
        Token{name: String::from("AS"), value: String::from(""), start: 0},
        Token{name: String::from("ST"), value: String::from("\"foo####\\\"## bar\\\"\\\\\\\"\""), start: 3},
        Token{name: String::from("LT"), value: String::from(""), start: 28},
        Token{name: String::from("ST"), value: String::from("\"foo\\\n    \\\\\" '\\\"' bar\\\"\\\"\""), start: 30},
        Token{name: String::from("LT"), value: String::from(""), start: 59},
        Token{name: String::from("ST"), value: String::from("\"foo##############################################################\\\" bar\\\"\\\"# ######\""), start: 61},
        Token{name: String::from("LT"), value: String::from(""), start: 148},
        Token{name: String::from("ST"), value: String::from("\"foo####\\\" bar\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\ \\\"\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\ \""), start: 150},
        Token{name: String::from("LT"), value: String::from(""), start: 224},
        Token{name: String::from("ST"), value: String::from("\"hello\""), start: 230},
        Token{name: String::from("LT"), value: String::from(""), start: 239},
        Token{name: String::from("ST"), value: String::from("\",\""), start: 245},
        Token{name: String::from("LT"), value: String::from(""), start: 250},
        Token{name: String::from("ST"), value: String::from("\":\""), start: 252},
        Token{name: String::from("LT"), value: String::from(""), start: 257},
        Token{name: String::from("ST"), value: String::from("\".\""), start: 259},
        Token{name: String::from("LT"), value: String::from(""), start: 264},
        Token{name: String::from("ST"), value: String::from("\"\""), start: 266},
        Token{name: String::from("LT"), value: String::from(""), start: 270},
        Token{name: String::from("ST"), value: String::from("\",.\\\\ \""), start: 272},
        Token{name: String::from("LT"), value: String::from(""), start: 281},
        Token{name: String::from("ST"), value: String::from("\"/;\\\\\\\"'\""), start: 283},
        Token{name: String::from("LT"), value: String::from(""), start: 294},
        Token{name: String::from("ST"), value: String::from("\"?()\""), start: 296},
        Token{name: String::from("LT"), value: String::from(""), start: 303},
        Token{name: String::from("ST"), value: String::from("\"-!@#$4\""), start: 305},
        Token{name: String::from("LT"), value: String::from(""), start: 315},
        Token{name: String::from("ST"), value: String::from("\"3$\", \\\"…:[ ]{}‘\""), start: 317},
        Token{name: String::from("LT"), value: String::from(""), start: 336},
        Token{name: String::from("ST"), value: String::from("\"`~^*&%>_<+=e|©●≈\n    ```\n        hello world\n    ```\n    \""), start: 338},
        Token{name: String::from("LT"), value: String::from(""), start: 399},
        Token{name: String::from("ST"), value: String::from("\"\n    \""), start: 401},
        Token{name: String::from("LT"), value: String::from(""), start: 410},
        Token{name: String::from("ST"), value: String::from("\"\\\"\\\"\""), start: 416},
        Token{name: String::from("LT"), value: String::from(""), start: 424},
        Token{name: String::from("ST"), value: String::from("\"⟨ ⟩¤⌀÷℮« »¡¿¶®§℗™⁀℠±‰№× lol\""), start: 426},
        Token{name: String::from("AE"), value: String::from(""), start: 457}
        
    ];

    for (i, token) in tokens.iter().enumerate() {
        let expected_token = &expected[i];
 
        assert_eq!(*expected_token, *token);
    }
}
#[test]
fn test_numbers() {
    let src: &str = r#"b$[4$4.,4$5.5, 4$123, 4$123.0, 4$.5, 4$3.2e6, 4$314e-2, 4$0.0314E+2, 4$7.]"#;
    let tokens = lexer(src).unwrap();
    let expected = vec![
        Token{name: String::from("AS"), value: String::from(""), start: 0},
        Token{name: String::from("NU"), value: String::from("4."), start: 3},
        Token{name: String::from("LT"), value: String::from(""), start: 7},
        Token{name: String::from("NU"), value: String::from("5.5"), start: 8},
        Token{name: String::from("LT"), value: String::from(""), start: 13},
        Token{name: String::from("NU"), value: String::from("123"), start: 15},
        Token{name: String::from("LT"), value: String::from(""), start: 20},
        Token{name: String::from("NU"), value: String::from("123.0"), start: 22},
        Token{name: String::from("LT"), value: String::from(""), start: 29},
        Token{name: String::from("NU"), value: String::from(".5"), start: 31},
        Token{name: String::from("LT"), value: String::from(""), start: 35},
        Token{name: String::from("NU"), value: String::from("3.2e6"), start: 37},
        Token{name: String::from("LT"), value: String::from(""), start: 44},
        Token{name: String::from("NU"), value: String::from("314e-2"), start: 46},
        Token{name: String::from("LT"), value: String::from(""), start: 54},
        Token{name: String::from("NU"), value: String::from("0.0314E+2"), start: 56},
        Token{name: String::from("LT"), value: String::from(""), start: 67},
        Token{name: String::from("NU"), value: String::from("7."), start: 69},
        Token{name: String::from("AE"), value: String::from(""), start: 73}
    ];

    for (i, token) in tokens.iter().enumerate() {
        let expected_token = &expected[i];
 
        assert_eq!(*expected_token, *token);
    }
}
#[test]
fn test_big_int() {
    let src: &str = r#"8$9007199254740991"#;
    let tokens = lexer(src).unwrap();
    let expected = vec![
        Token{name: String::from("BI"), value: String::from("9007199254740991"), start: 0}
    ];

    assert_eq!(tokens.len(), expected.len());
    assert_eq!(expected[0], tokens[0]);
}
#[test]
fn test_infinity() {
    let src: &str = r#"5$-1"#;
    let tokens = lexer(src).unwrap();
    let expected = vec![
        Token{name: String::from("IN"), value: String::from("-1"), start: 0}
        ];

    assert_eq!(tokens.len(), expected.len());
    assert_eq!(expected[0], tokens[0]);
}
#[test]
fn test_nan() {
    let src: &str = r#"6$"#;
    let tokens = lexer(src).unwrap();
    let expected = vec![Token{name: String::from("NA"), value: String::from(""), start: 0}];

    assert_eq!(tokens.len(), expected.len());
    assert_eq!(expected[0], tokens[0]);
}
#[test]
fn test_undefined() {
    let src: &str = r#"0$"#;
    let tokens = lexer(src).unwrap();
    let expected = vec![Token{name: String::from("UN"), value: String::from(""), start: 0}];

    assert_eq!(tokens.len(), expected.len());
    assert_eq!(expected[0], tokens[0]);
}
#[test]
fn test_null() {
    let src: &str = r#"1$"#;
    let tokens = lexer(src).unwrap();
    let expected = vec![Token{name: String::from("NL"), value: String::from(""), start: 0}];

    assert_eq!(tokens.len(), expected.len());
    assert_eq!(expected[0], tokens[0]);
}
#[test]
fn test_boolean_true() {
    let src: &str = r#"2$1"#;
let tokens = lexer(src).unwrap();
let expected = vec![Token{name: String::from("BO"), value: String::from("1"), start: 0}];

assert_eq!(tokens.len(), expected.len());
assert_eq!(expected[0], tokens[0]);
}
#[test]
fn test_boolean_false() {
    let src: &str = r#"2$0"#;
    let tokens = lexer(src).unwrap();
    let expected = vec![Token{name: String::from("BO"), value: String::from("0"), start: 0}];

    assert_eq!(tokens.len(), expected.len());
    assert_eq!(expected[0], tokens[0]);
}
#[test]
fn test_date() {
    let src: &str = r#"7$2023-08-01T14:32:01.624Z"#;
    let tokens = lexer(src).unwrap();
    let expected = vec![Token{name: String::from("DT"), value: String::from("2023-08-01T14:32:01.624Z"), start: 0}];
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
    let tokens = lexer(src).unwrap();
    let expected = vec![
        Token{name: String::from("OS"), value: String::from(""), start: 0},
        Token{name: String::from("ST"), value: String::from("\"foo\""), start: 3},
        Token{name: String::from("KT"), value: String::from(""), start: 10},
        Token{name: String::from("FU"), value: String::from("[name=test] function test() { return URL + '/movies'; }"), start: 11},
        Token{name: String::from("LT"), value: String::from(""), start: 69},
        Token{name: String::from("ST"), value: String::from("\"bar\""), start: 70},
        Token{name: String::from("KT"), value: String::from(""), start: 77},
        Token{name: String::from("FU"), value: String::from("[name=test] function test(fn) {\n        if (typeof fn === \"function\") {\n            return true;\n        }\n        return false;\n    }"), start: 78},
        Token{name: String::from("LT"), value: String::from(""), start: 215},
        Token{name: String::from("ST"), value: String::from("\"myFn\"" ), start: 216},
        Token{name: String::from("KT"), value: String::from(""), start: 224},
        Token{name: String::from("FU"), value: String::from("[name=my_fn] function my_fn(a, b) {\n        return a + b;\n    }"), start: 225},
        Token{name: String::from("LT"), value: String::from(""), start: 291},
        Token{name: String::from("ST"), value: String::from("\"closure\""), start: 292},
        Token{name: String::from("KT"), value: String::from(""), start: 303},
        Token{name: String::from("FU"), value: String::from("[name=closure] function (isLoading) {\n        var foo = \"bar\";\n        return function (testParam) {\n            console.log(isLoading, foo, testParam);\n        };\n    }"), start: 304}, 
        Token{name: String::from("OE"), value: String::from(""), start: 476}
    ];

    for (i, token) in tokens.iter().enumerate() {
        let expected_token = &expected[i];
 
        assert_eq!(*expected_token, *token);
     }
}

#[test]

fn test_error_unexpected_end_of () {
    let src: &str = r#"3$"hello"#;
    let err = lexer(src).unwrap_err();

    assert_eq!(r#"Unexpected end 'o' of token "ST" at position 7"#, err.msg);
    
}