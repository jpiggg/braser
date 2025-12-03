fn main() -> Result<(), Box<dyn std::error::Error>> {
    use eson::parser::ast::ESon;
    use from_pest::FromPest;
    use pest::Parser;
    use std::fs;

    let source = String::from_utf8(fs::read("./examples/data.eson")?)?;
    let mut parse_tree = eson::parser::ESonParser::parse(eson::parser::Rule::ESon, &source)?;
    println!("parse tree = {parse_tree:#?}");
    let syntax_tree: ESon = ESon::from_pest(&mut parse_tree).expect("infallible");
    println!("syntax tree = {syntax_tree:#?}");
    println!();

     Ok(())
}

#[test]
fn eson_example_run() {
    main().unwrap()
}