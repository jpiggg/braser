// use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/eson.pest"]
pub struct ESONParser;