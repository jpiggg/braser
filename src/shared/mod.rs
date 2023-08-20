pub mod tokens;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct Node {
	pub kind: String,
	pub value: String,
	pub children: Vec<Node>
}
