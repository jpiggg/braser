use stringify::lexer::run as lexer;
use stringify::parser::run as parser;
use stringify::shared::Node;

fn main() {
    // let str: &str = "a${"3$foo\" bar\"": 3$\"ß%vbjbfjfeb \"\" \'dffdfd%ß\"}";
    // let str0: &str = r#"a${3$"foo\" bar\"":3$"test"}"#;
    let str1: &str = r#"a${3$"foo####\" bar\"":3$"test",3$"hi":4$100500,3$"date":7$2023-08-01T14:32:01.624Z,3$"myFn":9$function my_fn(a, b) {
        return a + b;
    }$,3$"undefined":0$,3$"nan":6$,3$"null":1$}"#;

    // let str2: &str = "a${3$\"hello\"$3: a${3$\"foo\"$3: 3$\"bar\"$3}}";
    // let str3: &str = "a${3$\"hello\": 3$\"world\", 3$\"fo\'bar\'r\": 8$5.56789e+0, 3$\"date\": 7$\'2022-03-02T13:39:36.614Z\',3$\"ariel\": 4$1234567876543212345678,3$\"artem\":1$,3$\"artem2\":0$,3$\'jpig\': 4$01, 3$\'jpig2\': 4$1.}";
    // let result = lexer(str0);
    // println!("{str0:?}");
    // println!("Hello, lexer! {:?}", result);

    // let nodes_tree = parser(Node {
    //     kind: "root",
    //     value: "",
    //     children: vec![]
    // }, &result);

    // println!("Hello, parser! {:?}", nodes_tree);

    let result = lexer(str1);
    println!("{str1:?}");
    println!("Hello, lexer! {:?}", result);

    let nodes_tree = parser(Node {
        kind: "root",
        value: "",
        children: vec![]
    }, &result);
 
    println!("Hello, parser! {:?}", nodes_tree);
}
