#![allow(
    dead_code
)]

use pest_derive;

#[derive(pest_derive::Parser)]
#[grammar = "src/parse/grammar/eson.pest"]
pub struct ESonParser;

pub mod ast {
    use super::Rule;
    use pest::Span;

    fn span_into_str(span: Span) -> &str {
        span.as_str()
    }

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::Boolean))]
    pub struct Boolean {
        #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))]
        pub value: bool
    }

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::String))]
    pub struct String<'pest> {
        #[pest_ast(outer(with(span_into_str)))]
        pub value: &'pest str
    }
    

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::Number))]
    pub struct Number {
        #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))]
        pub value: f64
    }

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::Infinity))]
    pub struct Infinity {}

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::undefined))]
    pub struct Undefined {}

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::NaN))]
    pub struct NaN {}

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::null))]
    pub struct Null {}

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::BigInt))]
    pub struct BigInt<'pest> {
        #[pest_ast(outer(with(span_into_str)))]
        pub value: &'pest str
    }

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::value))]
    pub enum Value<'pest> {
        Object(Box<Object<'pest>>),
        Array(Box<Array<'pest>>),
        String(String<'pest>),
        Number(Number),
        Boolean(Boolean),
        Null(Null),
        Undefined(Undefined),
        NaN(NaN),
        Infinity(Infinity),
        BigInt(BigInt<'pest>)
    }

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::COMMENT))]
    pub struct Comment<'pest> {
        #[pest_ast(outer(with(span_into_str)))]
        pub value: &'pest str
    }

    // #[derive(PartialEq, Debug, pest_ast::FromPest)]
    // #[pest_ast(rule(Rule::key_variant))]
    // pub struct Key<'pest> {
    //     #[pest_ast(outer(with(span_into_str)))]
    //     pub value: &'pest str
    // }

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::key_variant))]
    pub enum Key<'pest> {
        KeySimple(String<'pest>),
        KeyComplex(String<'pest>),
    }

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::key_simple))]
    pub struct KeySimple<'pest> {
        #[pest_ast(outer(with(span_into_str)))]
        pub value: &'pest str
    }

     #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::key_complex))]
    pub struct KeyComplex<'pest> {
        #[pest_ast(outer(with(span_into_str)))]
        pub value: &'pest str
    }

    //@TODO: научиться парсить комментарии, если перед ними идет запятая

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::pair))]
    pub struct Pair<'pest> {
        pub key: Key<'pest>,
        pub value: Value<'pest>,
        pub comment: Option<Comment<'pest>>
    }

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::Array))]
    pub struct Array<'pest> {
        #[pest_ast(default(Vec::new()))]
        pub value: Vec<Value<'pest>>,
    }

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::Object))]
    pub struct Object<'pest> {
        pub pair: Vec<Pair<'pest>>
    }


    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::ESon))]
    pub struct ESon<'pest> {
        pub object: Object<'pest>,
        pub _eoi: EOI,
    }

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::EOI))]
    pub struct EOI;
}


#[cfg(test)]
mod tests {
    use from_pest::FromPest;
    use pest::Parser;
    use crate::parse::parser::ast;
    use pretty_assertions::{assert_eq};

    #[test]
    fn test_flat_object() {
        let source = String::from_utf8(std::fs::read("./examples/data.eson").unwrap()).unwrap();
        let mut parse_tree = crate::parse::parser::ESonParser::parse(crate::parse::parser::Rule::ESon, &source).unwrap();
        let syntax_tree: ast::ESon = ast::ESon::from_pest(&mut parse_tree).expect("infallible");

        let expected = ast::ESon {
            object: ast::Object {
                pair: vec![
                    ast::Pair {
                        key: ast::Key::KeySimple(ast::String {value: "name"}),
                        value: ast::Value::Number(ast::Number { value: 13.0 }),
                        comment: None
                    },
                    ast::Pair {
                        key: ast::Key::KeySimple(ast::String {value: "value"}),
                        value: ast::Value::String(ast::String { value: "ba\\'z" }),
                        comment: None
                    },
                    ast::Pair {
                         key: ast::Key::KeySimple(ast::String {value: "isDefined"}),
                         value: ast::Value::Boolean(ast::Boolean { value: false }),
                         comment: None
                    },
                    ast::Pair {
                        key: ast::Key::KeySimple(ast::String {value: "isNotDefined"}),
                        value: ast::Value::Boolean(ast::Boolean { value: true }),
                        comment: Some(ast::Comment { value: "// This is a response from some API"})
                    },
                    ast::Pair {
                        key: ast::Key::KeySimple(ast::String {value: "data"}),
                        value: ast::Value::Undefined(ast::Undefined {}),
                        comment: None
                    },
                    ast::Pair {
                        key: ast::Key::KeyComplex(ast::String {value: "valid_until"}),
                        value: ast::Value::NaN(ast::NaN {}),
                        comment: None
                    },
                    ast::Pair {
                        key: ast::Key::KeyComplex(ast::String {value: "valid_from"}),
                        value: ast::Value::Number(ast::Number { value: 1763225669356.0 }),
                        comment: Some(ast::Comment { value : "/*\n    All cool guys use buffer like this:\n    [1, 2, 3] // Yes, it is a comment inside another one!\n  */"})
                    },
                    ast::Pair {
                        key: ast::Key::KeySimple(ast::String {value: "buffer"}),
                        value: ast::Value::Array(Box::new(ast::Array {
                            value: vec![
                                ast::Value::Number(ast::Number { value: 16.0 }),
                                ast::Value::Number(ast::Number { value: 21.0 }),
                                ast::Value::Number(ast::Number { value: 51.0 }),
                                ast::Value::Number(ast::Number { value: 0.0 }),
                                ast::Value::Number(ast::Number { value: 0.0 }),
                                ast::Value::Number(ast::Number { value: 0.0 }),
                                ast::Value::Number(ast::Number { value: 0.0 })
                            ]
                        })),
                        comment: None
                    },
                    ast::Pair {
                        key: ast::Key::KeySimple(ast::String {value: "src"}),
                        value: ast::Value::Array(Box::new(ast::Array {
                            value: vec![
                                ast::Value::String(ast::String { value: "a" }),
                                ast::Value::String(ast::String { value: "b" }),
                                ast::Value::String(ast::String { value: "c" }),
                            ]
                        })),
                        comment: None
                    },
                    ast::Pair {
                        key: ast::Key::KeyComplex(ast::String {value: "source42"}),
                        value: ast::Value::String(ast::String { value: "abc" }),
                        comment: None
                    }
                ]
            },
            _eoi: ast::EOI
        };

        assert_eq!(&syntax_tree, &expected);
    }
}