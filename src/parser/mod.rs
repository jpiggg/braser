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

fn create_node<'a>(mut node: Node<'a>, mut tokens: &'a [Token<'a>]) -> (Node<'a>, &'a [Token<'a>]){
	let token_type = token_types.get(&tokens[0].name).unwrap();

	match *token_type {
		"objectStart" => {
			if is_valid_object_key(token_types.get(&tokens[1].name).unwrap()) && is_key_pair(&tokens[1..3]){
				let mut root_node = Node {
					kind: "object",
					value: "",
					children: vec![]
				};

				let (new_node, rest) = create_node (Node {
					kind: "pair",
					value: tokens[1].value,
					children: vec![]
				}, &tokens[3..(tokens.len())]);

				root_node.children.push(new_node);
				node.children.push(root_node);
				tokens = rest;

			}
		},
		"arrayStart" => {
			let (new_node, rest) = create_node( Node {
				kind: "array",
				value: "",
				children: vec![]
			}, &tokens[1..(tokens.len())]);

			node.children.push(new_node);
			tokens = rest;
		},
		"objectEnd" => {
			return (node, &tokens[1..tokens.len()]);
		},
		"arrayEnd" => {
			return (node, &tokens[1..tokens.len()]);
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

pub fn run<'a>(root: Node<'a>, tokens: &'a [Token<'a>]) -> Node<'a> {
	if tokens.len() == 0 {
		return root;
	}

	let (new_node, rest) = create_node( root.clone(), tokens);

	run(new_node, rest)
}