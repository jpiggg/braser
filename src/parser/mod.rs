pub mod errors;

use self::errors::Error;
use crate::shared::tokens::{TOKEN_TYPES, Token};
use crate::shared::Node;

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

fn create_object<'a>( mut root_node: Node<'a>, tokens: &'a [Token<'a>]) ->( Node<'a>, &'a [Token<'a>]) {
	if tokens.len() == 0 {
		return (root_node, tokens);
	}

	let token_type = TOKEN_TYPES.get(&tokens[0].name).unwrap();

	if *token_type == "objectEnd" {
		return (root_node, &tokens[1..tokens.len()]);
	}

	if !is_key_pair(&tokens[0..2]) {
        Error::new("Invalid key pair value");
    }

	if !is_valid_object_key(TOKEN_TYPES.get(&tokens[0].name).unwrap()) {
		Error::new("Invalid object key type");
	}

    let (new_node, rest) = create_node (Node {
        kind: "pair",
        value: tokens[0].value,
        children: vec![]
    }, &tokens[2..tokens.len()]);

    root_node.children.push(new_node);
    
    let token_type_2 = TOKEN_TYPES.get(&rest[0].name).unwrap();

    if *token_type_2 == "objectEnd" {
        return (root_node, &rest[1..rest.len()]);
    } else {
        return create_object(root_node.clone(), &rest[1..rest.len()]);
    }

}

fn create_array<'a>(root_node: Node<'a>, tokens: &'a [Token<'a>]) -> ( Node<'a>, &'a [Token<'a>]) {
	if tokens.len() == 0 {
		return (root_node, &tokens[1..(tokens.len())]);
	}

	let token_type = TOKEN_TYPES.get(&tokens[0].name).unwrap();

	if *token_type == "arrayEnd" {
		return (root_node, &tokens[1..tokens.len()]);
	}


	let (new_node, rest) = create_node( root_node.clone(), tokens);

	let token_type_2 = TOKEN_TYPES.get(&rest[0].name).unwrap();

    if *token_type_2 == "arrayEnd" {
        return (new_node, &rest[1..rest.len()]);
    } else {
        return create_array(new_node, &rest[1..rest.len()]);
    }
}

fn create_node<'a>(mut node: Node<'a>, mut tokens: &'a [Token<'a>]) -> (Node<'a>, &'a [Token<'a>]){
	let token_type = TOKEN_TYPES.get(&tokens[0].name).unwrap();

	match *token_type {
		"objectStart" => {
			let root_node = Node {
				kind: "object",
				value: "",
				children: vec![]
			};

			let (object_node, rest) = create_object(root_node, &tokens[1..tokens.len()]);

			node.children.push(object_node);
			tokens = rest;
		},
		"arrayStart" => {
			let (new_node, rest) = create_array( Node {
				kind: "array",
				value: "",
				children: vec![]
			}, &tokens[1..(tokens.len())]);

			node.children.push(new_node);
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

pub fn run<'a>(root: Node<'a>, tokens: &'a [Token<'a>]) -> Node<'a> {
	if tokens.len() == 0 {
		return root;
	}

	let (new_node, rest) = create_node( root.clone(), tokens);

	run(new_node, rest)
}