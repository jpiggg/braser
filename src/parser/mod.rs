use pest::Parser;

use pest_derive;
use from_pest;
use pest;

#[derive(pest_derive::Parser)]
#[grammar = "grammar/eson.pest"]
pub struct ESonParser;

pub mod ast {
    use super::Rule;
    use pest::Span;

    fn span_into_str(span: Span) -> &str {
        println!("Spaaaaan --------> ${:?}", span);
        span.as_str()
    }

    #[derive(Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::Boolean))]
    pub struct Boolean {
        #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))]
        value: bool
    }

    #[derive(Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::String))]
    pub struct String {
        #[pest_ast(outer(with(span_into_str), with(str::to_string)))]
        value: std::string::String
    }

    #[derive(Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::Number))]
    pub struct Number {
        #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))]
        value: f64
    }

    #[derive(Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::Infinity))]
    pub struct Infinity {}

    #[derive(Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::undefined))]
    pub struct Undefined {}

    #[derive(Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::NaN))]
    pub struct NaN {}


    #[derive(Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::null))]
    pub struct Null {}

    #[derive(Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::BigInt))]
    pub struct BigInt {}

    #[derive(Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::value))]
    pub enum Value {
        Object(Box<Object>),
        Array(Box<Array>),
        String(String),
        Number(Number),
        Boolean(Boolean),
        Null(Null),
        Undefined(Undefined),
        NaN(NaN),
        Infinity(Infinity),
        BigInt(BigInt)
    }

     #[derive(Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::COMMENT))]
    pub struct Comment {
        #[pest_ast(outer(with(span_into_str), with(str::to_string)))]
        value: std::string::String
    }

    #[derive(Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::key))]
    pub struct Key {
        #[pest_ast(outer(with(span_into_str), with(str::to_string)))]
        value: std::string::String
    }

    //@TODO: научиться парсить комментарии, если перед ними идет запятая

    #[derive(Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::pair))]
    pub struct Pair {
        pub key: Option<Key>,
        pub value: Option<Value>,
        comment: Option<Comment>
    }

    #[derive(Debug, pest_ast::FromPest)]

    #[pest_ast(rule(Rule::Array))]
    pub struct Array {
        #[pest_ast(default(Vec::new()))]
        pub value: Vec<Value>,
    }


    #[derive(Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::Object))]
    pub struct Object {
        pub pair: Vec<Pair>
    }

    #[derive(Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::ESon))]
    pub struct ESon {
        pub object: Object,
        _eoi: EOI,
    }

    #[derive(Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::EOI))]
    struct EOI;
}
