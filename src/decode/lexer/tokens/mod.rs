#[derive(Debug)]
#[allow(dead_code)]

pub struct Codes {
    inline_comment: String,
    multiline_comment: String,
    object_start: String,
    object_end: String,
    array_start: String,
    array_end: String,
    number: String,
    nan: String,
    big_int: String,
    sign: String,
    function: String,
    string: String,
    listing_terminator: String,
    key_terminator: String,
    boolean: String,
    date: String,
    null: String,
    whitespace: String,
    undefined: String
}

impl Codes {
    pub fn create() -> Self {
        Self {
            inline_comment: String::from("IC"),
            multiline_comment: String::from("MC"),
            object_start: String::from("OS"),
            object_end: String::from("OE"),
            array_start: String::from("AS"),
            array_end: String::from("AE"),
            number: String::from("NU"),
            nan: String::from("NA"),
            big_int: String::from("BI"),
            sign: String::from("SI"),
            function: String::from("FU"),
            string: String::from("ST"),
            listing_terminator: String::from("LT"),
            key_terminator: String::from("KT"),
            boolean: String::from("BO"),
            date: String::from("DT"),
            null: String::from("NL"),
            whitespace: String::from("WS"),
            undefined: String::from("UN")
        }
    }
}

