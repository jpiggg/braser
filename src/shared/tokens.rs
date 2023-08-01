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
		("BO", "boolean"),
		("ST", "string"),
		("NU", "number"),
		("NA", "nan"),
		("DT", "date"),
		("BI", "bigint"),
		("FU", "function")
	]);
}


#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Token<'a> {
    pub name: &'a str,
    pub value: &'a str
}
