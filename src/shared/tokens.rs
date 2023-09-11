use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
	pub static ref TOKENS: HashMap<&'static str, String> = HashMap::from([
		("undefined", String::from("0$")),
		("null", String::from("1$")),
		("boolean", String::from("2$")),
		("string", String::from("3$")),
		("number", String::from("4$")),
		("infinity", String::from("5$")),
		("nan", String::from("6$")),
		("date", String::from("7$")),
		("bigint", String::from("8$")),
		("function", String::from("9$")),
		("objectstart", String::from("a$")),
		("arraystart", String::from("b$"))
	]);
}

lazy_static! {
    pub static ref TOKEN_TYPES: HashMap<&'static str, &'static str> = HashMap::from([
		("OS", "objectStart"),
		("OE", "objectEnd"),
		("AS", "arrayStart"),
		("AE", "arrayEnd"),
		("KT", "key_terminator"),
		("LT", "listing_terminator"),
		("UN", "undefined"),
		("NL", "null"),
		("IN", "infinity"),
		("BO", "boolean"),
		("ST", "string"),
		("NU", "number"),
		("NA", "nan"),
		("DT", "date"),
		("BI", "bigint"),
		("FU", "function")
	]);
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct Token {
    pub name: String,
    pub value: String
}
