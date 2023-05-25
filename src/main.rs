use stringify::lexer::run as lexer;

// // 		const str =
// 'a${3$\'hello\': 3$"world", 3$"fo\\"bar\\"r": 8$5.56789e+0, 3$"date": 7$\'2022-03-02T13:39:36.614Z\',3$"ariel": 4$1234567876543212345678,3$"artem":1$,3$"artem2":0$,3$\'jpig\': 4$01, 3$\'jpig2\': 4$1.}';

// const expectedLexerResult = [
//     { value: 'a$', type: 'OS' },
//     { type: 'ST', value: "3$'hello'" },
//     { type: 'KT', value: '' },
//     { type: 'ST', value: '3$"world' },
//     { type: 'LT', value: '' },
//     { type: 'ST', value: '3$"fo\\"bar\\"r' },
//     { type: 'KT', value: '' },
//     { type: 'BI', value: '8$5.56789e+0' },
//     { type: 'LT', value: '' },
//     { type: 'ST', value: '3$"date' },
//     { type: 'KT', value: '' },
//     {
//         type: 'DT',
//         value: "7$'2022-03-02T13:39:36.614Z",
//     },
//     { type: 'LT', value: '' },
//     { type: 'ST', value: '3$"ariel' },
//     { type: 'KT', value: '' },
//     { type: 'NU', value: '4$1234567876543212345678' },
//     { type: 'LT', value: '' },
//     { type: 'ST', value: '3$"artem' },
//     { type: 'KT', value: '' },
//     { type: 'NL', value: '1$' },
//     { type: 'LT', value: '' },
//     { type: 'ST', value: '3$"artem2' },
//     { type: 'KT', value: '' },
//     { type: 'UN', value: '0$' },
//     { type: 'OE', value: '' },
// ];

fn main() {
    let str = "a${3$\"ghbdtn%ß\"$3: 3$\"ß%vbjbfjfeb \"\" \'dffdfd%ß\"$3}";
    let result = lexer(str);
    println!("Hello, world! {:?}", result);
}
