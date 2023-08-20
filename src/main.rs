use stringify::lexer::run as lexer;
use stringify::parser::run as parser;
use stringify::shared::Node;

// #[wasm_bindgen]
pub fn main() {
    let str1: &str = r#"a${3$"foo####\" bar\"":3$"test",3$"hi":4$100500,3$"date":7$2023-08-01T14:32:01.624Z,3$"myFn":9$function my_fn(a, b) {
        return a + b;
    }$,3$"undefined":0$,3$"nan":6$,3$"null":1$}"#;

    let result = lexer(str1);
    println!("{str1:?}");
    println!("Hello, lexer! {:?}", result);

    let nodes_tree = parser(Node {
        kind: String::from("root"),
        value: String::from(""),
        children: vec![]
    }, &result);
 
    println!("Hello, parser! {:?}", nodes_tree);
}
