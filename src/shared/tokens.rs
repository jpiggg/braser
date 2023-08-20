use std::collections::HashMap;
use lazy_static::lazy_static; 

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
