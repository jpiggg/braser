use crate::shared::tokens::{token_types, Token};
use crate::shared::Node;

/* eslint-disable no-param-reassign */
// const tree = {
// 	Root: {
// 		type: 'object',
// 		start: 'index',
// 		end: 'index',
// 		children: [
// 			{
// 				type: 'pair',
// 				value: 'hello',
// 				start: 20,
// 				end: 24,
// 				children: [
// 					{
// 						type: 'string',
// 						value: 'world',
// 					},
// 				],
// 			},
// 		],
// 	},
// };

// const LEXEMS = {
// 	Document: 'ROOT',
// 	Object: 'OBJ',
// 	String: 'STR',
// 	Number: 'NUM',
// 	Boolean: 'BOOL',
// 	Null: 'NULL',
// 	InlineComment: 'IC',
// 	MultilineComment: 'MC',
// 	Pair: 'PAIR',
// 	Array: 'ARR',
// };

// [Token { name: "OS", value: "" }, Token { name: "ST", value: "\"ghbdtn%ß\"$" }, Token { name: "KT", value: "" }, Token { name: "ST", value: "\"ß%vbjbfjfeb \"\" 'dffdfd%ß\"$" }, Token { name: "OE", value: "" }]

fn is_valid_object_key(token_type: &str) -> bool {
	match token_type {
		"string" => true,
		"number" => true,
		"bigint" => true,
		_ => false
	}
}

fn is_key_pair(token_slice: &[Token<'_>]) -> bool {
	if token_slice[0].name != "KT" && token_slice[1].name == "KT" {
		return true;
	}

	false
}

fn create_node<'a>(token_type: &'a str, mut node: Node<'a>, mut tokens: &'a [Token<'a>]) -> (Node<'a>, &'a [Token<'a>]){
	if token_type == "objectEnd" || token_type == "arrayEnd" {
		return (node, &tokens[1..tokens.len()]);
	}

	match token_type {
		"objectStart" => {
			let (value, rest) = create_object(Node {
				kind: "object",
				value: "",
				children: vec![]
			}, &tokens[1..(tokens.len())]);

			node.children.push(value);
			tokens = rest;

		},
		"arrayStart" => {
			let (value, rest) = create_array(Node {
				kind: "array",
				value: "",
				children: vec![]
			}, &tokens[1..(tokens.len())]);

			node.children.push(value);
			tokens = rest;
		},
		_ => {
			node.children.push(Node {
				kind: token_type,
				value: tokens[0].value,
				children: vec![]
			});

			tokens = &tokens[1..(tokens.len())]
		}
	}

	(node, tokens)
}

fn create_object<'a>(mut node: Node<'a>, mut tokens: &'a [Token<'a>]) -> (Node<'a>, &'a [Token<'a>]) {
	let token_type = token_types.get(&tokens[0].name).unwrap();

	if *token_type == "objectEnd" {
		return (node, tokens);
	}

	if is_valid_object_key(token_type) && is_key_pair(&tokens[0..2]){	
		let value_type = token_types.get(&tokens[2].name).unwrap();

		let (new_node, rest) = create_node(*value_type, Node {
			kind: "pair",
			value: tokens.first().unwrap().value,
			children: vec![]
		}, &tokens[2..(tokens.len())]);
		tokens = rest;
		node.children.push(new_node);
	}

	(node, tokens)

}

fn create_array<'a>(mut node: Node<'a>, mut tokens: &'a [Token<'a>]) -> (Node<'a>, &'a [Token<'a>]){
	let token_type = token_types.get(&tokens[0].name).unwrap();

	if *token_type == "arrayEnd" {
		return (node, tokens);
	}

	let (new_node, rest) = create_node(*token_type, node.clone(), tokens);

	tokens = rest;
	node.children.push(new_node);

	(node, tokens)
}



pub fn run<'a>(root: Node<'a>, tokens: &'a [Token<'a>]) -> Node<'a> {
	if tokens.len() == 0 {
		return root;
	}

	let token = tokens[0];
	let token_type = token_types.get(token.name).unwrap();
	let (new_node, rest) = create_node(token_type, root.clone(), tokens);

	run(new_node, rest)
}