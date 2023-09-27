pub mod error;

extern crate regex;

use regex::RegexBuilder;
use self::error::ParsingError;
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

fn create_object<'a>( mut root_node: Node, tokens: &'a [Token]) -> Result<( Node, &'a [Token]), ParsingError> {
	if tokens.len() == 0 {
		return Ok((root_node, tokens));
	}

	let token_type = TOKEN_TYPES.get(&tokens[0].name.as_str()).unwrap();

	if *token_type == "objectEnd" {
		return Ok((root_node, &tokens[1..tokens.len()]));
	}

	if !is_key_pair(&tokens[0..2]) {
		return Err(ParsingError::new("INVALID_KEY_PAIR"));
    }

	if !is_valid_object_key(TOKEN_TYPES.get(&tokens[0].name.as_str()).unwrap()) {
		return Err(ParsingError::new("INVALID_OBJECT_KEY"));
	}

    let (new_node, rest) = create_node (Node {
        kind: String::from("pair"),
        value: tokens[0].value.clone(),
        children: vec![]
    }, &tokens[2..tokens.len()])?;

    root_node.children.push(new_node);
    
    let token_type_2 = TOKEN_TYPES.get(&rest[0].name.as_str()).unwrap();

    if *token_type_2 == "objectEnd" {
        return Ok((root_node, &rest[1..rest.len()]));
    } else {
        return create_object(root_node.clone(), &rest[1..rest.len()]);
    }

}

fn create_array(root_node: Node, tokens: &[Token]) -> Result<( Node, &[Token]), ParsingError> {
	if tokens.len() == 0 {
		return Ok((root_node, &tokens[1..(tokens.len())]));
	}

	let token_type = TOKEN_TYPES.get(&tokens[0].name.as_str()).unwrap();

	if *token_type == "arrayEnd" {
		return Ok((root_node, &tokens[1..tokens.len()]));
	}


	let (new_node, rest) = create_node( root_node.clone(), tokens)?;

	let token_type_2 = TOKEN_TYPES.get(&rest[0].name.as_str()).unwrap();

    if *token_type_2 == "arrayEnd" {
        return Ok((new_node, &rest[1..rest.len()]));
    } else {
        return create_array(new_node, &rest[1..rest.len()]);
    }
}

fn create_node(mut node: Node, mut tokens: &[Token]) -> Result<(Node, &[Token]), ParsingError>{
	let token_type = TOKEN_TYPES.get(&tokens[0].name.as_str()).unwrap();

	match *token_type {
		"objectStart" => {
			let root_node = Node {
				kind: String::from("object"),
				value: String::from(""),
				children: vec![]
			};

			let (object_node, rest) = create_object(root_node, &tokens[1..tokens.len()])?;

			node.children.push(object_node);
			tokens = rest;
		},
		"arrayStart" => {
			let (new_node, rest) = create_array( Node {
				kind: String::from("array"),
				value: String::from(""),
				children: vec![]
			}, &tokens[1..(tokens.len())])?;

			node.children.push(new_node);
			tokens = rest;
		},
		"function" => {
			let re = RegexBuilder::new(r"\[name=([^\]]+)+\](.*)").multi_line(false).crlf(false).build().unwrap();
			let test = re.replace(&tokens[0].value.as_str(), "$2");

			let res = re.captures(&tokens[0].value.as_str()).unwrap();
			let fn_name = res.get(1).unwrap().as_str();
			
			node.children.push(Node {
				kind: token_type.to_string(),
				value: "var ".to_string() + fn_name + " =" + &test + "; " + fn_name,
				children: vec![]
			});

			tokens = &tokens[1..(tokens.len())]
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

	Ok((node, tokens))
}

pub fn run(root: Node, tokens: &[Token]) -> Result<Node, ParsingError> {
	if tokens.len() == 0 {
		return Ok(root);
	}

	let (new_node, rest) = create_node( root.clone(), tokens)?;

	run(new_node, rest)
}