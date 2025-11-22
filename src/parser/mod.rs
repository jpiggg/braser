use pest::Parser;

use pest_derive;
use from_pest;
use pest_ast;
use pest;

#[derive(pest_derive::Parser)]
#[grammar = "grammar/eson.pest"]
pub struct ESonParser;

mod ast {
    use super::Rule;
    use pest::Span;

    fn span_into_str(span: Span) -> &str {
        span.as_str()
    }

    #[derive(Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::Boolean))]
    pub struct Boolean {}

    #[derive(Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::String))]
    pub struct String {}

    #[derive(Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::Number))]
    pub struct Number {}

    #[derive(Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::Infinity))]
    pub struct Infinity {}

    #[derive(Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::BigInt))]
    pub struct BigInt {}

    #[derive(Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::value))]
    pub enum Value {
        Object(Object),
        Array(Array),
        String(String),
        Number(Number),
        Boolean(Boolean),
        Null,
        // #[pest_ast(outer(with(span_into_str), with(str::to_string)))]
        // _undefined(Undefined),
        NaN,
        Infinity(Infinity),
        BigInt(BigInt)
    }


    #[derive(Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::pair))]
    pub struct Pair {
        #[pest_ast(outer(with(span_into_str), with(str::to_string)))]
        pub key: std::string::String,
        pub value: Value
    }

    #[derive(Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::Array))]
    pub struct Array {
        pub value: Vec<Value>,
    }


    #[derive(Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::Object))]
    pub struct Object {
        pub pair: Vec<Pair>,
    }

    #[derive(Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::ESon))]
    pub struct ESon {
        pub Object: Object,
        _eoi: EOI,
    }

    #[derive(Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::EOI))]
    struct EOI;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use crate::parser::ast::ESon;
    use from_pest::FromPest;
    use pest::Parser;
    use std::fs;

    let source = String::from_utf8(fs::read("./src/examples/data.eson")?)?;
    let mut parse_tree = ESonParser::parse(Rule::ESon, &source)?;
    println!("parse tree = {parse_tree:#?}");
    let syntax_tree: ESon = ESon::from_pest(&mut parse_tree).expect("infallible");
    println!("syntax tree = {syntax_tree:#?}");
    println!();

    Ok(())
}

#[test]
fn csv_example_runs() {
    main().unwrap()
}