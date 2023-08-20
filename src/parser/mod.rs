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

fn is_key_pair(token_slice: &[Token]) -> bool {
	if token_slice[0].name != "KT" && token_slice[1].name == "KT" {
		return true;
	}

	false
}

fn create_object<'a>( mut root_node: Node, tokens: &'a [Token]) ->( Node, &'a [Token]) {
	if tokens.len() == 0 {
		return (root_node, tokens);
	}

	let token_type = TOKEN_TYPES.get(&tokens[0].name.as_str()).unwrap();

	if *token_type == "objectEnd" {
		return (root_node, &tokens[1..tokens.len()]);
	}

	if !is_key_pair(&tokens[0..2]) {
        Error::new("Invalid key pair value");
    }

	if !is_valid_object_key(TOKEN_TYPES.get(&tokens[0].name.as_str()).unwrap()) {
		Error::new("Invalid object key type");
	}

    let (new_node, rest) = create_node (Node {
        kind: String::from("pair"),
        value: tokens[0].value.clone(),
        children: vec![]
    }, &tokens[2..tokens.len()]);

    root_node.children.push(new_node);
    
    let token_type_2 = TOKEN_TYPES.get(&rest[0].name.as_str()).unwrap();

    if *token_type_2 == "objectEnd" {
        return (root_node, &rest[1..rest.len()]);
    } else {
        return create_object(root_node.clone(), &rest[1..rest.len()]);
    }

}

fn create_array(root_node: Node, tokens: &[Token]) -> ( Node, &[Token]) {
	if tokens.len() == 0 {
		return (root_node, &tokens[1..(tokens.len())]);
	}

	let token_type = TOKEN_TYPES.get(&tokens[0].name.as_str()).unwrap();

	if *token_type == "arrayEnd" {
		return (root_node, &tokens[1..tokens.len()]);
	}


	let (new_node, rest) = create_node( root_node.clone(), tokens);

	let token_type_2 = TOKEN_TYPES.get(&rest[0].name.as_str()).unwrap();

    if *token_type_2 == "arrayEnd" {
        return (new_node, &rest[1..rest.len()]);
    } else {
        return create_array(new_node, &rest[1..rest.len()]);
    }
}

fn create_node(mut node: Node, mut tokens: &[Token]) -> (Node, &[Token]){
	let token_type = TOKEN_TYPES.get(&tokens[0].name.as_str()).unwrap();

	match *token_type {
		"objectStart" => {
			let root_node = Node {
				kind: String::from("object"),
				value: String::from(""),
				children: vec![]
			};

			let (object_node, rest) = create_object(root_node, &tokens[1..tokens.len()]);

			node.children.push(object_node);
			tokens = rest;
		},
		"arrayStart" => {
			let (new_node, rest) = create_array( Node {
				kind: String::from("array"),
				value: String::from(""),
				children: vec![]
			}, &tokens[1..(tokens.len())]);

			node.children.push(new_node);
			tokens = rest;
		},
		_ => {
			node.children.push(Node {
				kind: token_type.to_string(),
				value: tokens[0].value.clone(),
				children: vec![]
			});

			tokens = &tokens[1..(tokens.len())]
		}
	}

	(node, tokens)
}

pub fn run(root: Node, tokens: &[Token]) -> Node {
	if tokens.len() == 0 {
		return root;
	}

	let (new_node, rest) = create_node( root.clone(), tokens);

	run(new_node, rest)
}