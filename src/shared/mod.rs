pub mod tokens;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct Node<'a> {
	pub kind: &'a str,
	pub value: &'a str,
	pub children: Vec<Node<'a>>
}