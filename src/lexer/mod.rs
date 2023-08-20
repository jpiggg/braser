pub mod tokens;
pub mod helpers;

use std::fmt;
use std::collections::HashMap;
use helpers::{is_double_quote, is_digit, is_sign, is_point, is_exp};
use crate::shared::tokens::{TOKEN_TYPES, Token};

const TOKEN_NAME: &'static str = "TOKEN_NAME";
const TOKEN_VALUE: &'static str  = "TOKEN_VALUE";


#[derive(Clone, Copy)]
pub struct Char {
    prev_char: Option<char>,
    cur_char:  char,
    next_char: Option<char>
}

impl fmt::Display for Char {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Char consists from: \nprev_char: {}, \nchar: {}, \nnext_char: {}", self.prev_char.unwrap_or_default(), self.cur_char, self.next_char.unwrap_or_default())
    }
}

pub fn accumulate_string(char: Char) -> bool {
    if char.prev_char.unwrap() != '$' && is_double_quote(Some(char.cur_char)) && char.prev_char.unwrap_or_default() != '\\' {
        return true;
    }



    false
}

pub fn accumulate_date(char: Char) -> bool {
    if char.cur_char == 'Z' {
        return true;
    }

    false
}

pub fn accumulate_number(char: Char) -> bool {
    if  !is_digit(char.next_char) && !is_point(char.next_char) && !is_sign(char.next_char) && !is_exp(char.next_char) {
        return true;
    }

    false
}

pub fn accumulate_boolean(char: Char) -> bool {
    if char.cur_char == '1' || char.cur_char == '0' {
        return true;
    }

    false
}

pub fn accumulate_infinity(char: Char) -> bool {
    println!("accumulate_infinity, {}", char);
    if char.cur_char == '1' {
        return true;
    }

    false
}

pub fn accumulate_function(char: Char) -> bool {
    if char.cur_char == '}' && char.next_char.unwrap_or_default() == '$' {
        return true;
    }

    false
}

#[derive(Copy, Clone, Debug)]
pub struct Ctx<'a> {
    token_status: &'static str,
    token_name: &'a str,
    token_value: &'a str,
    token_type: &'a str
}

impl<'a> Ctx<'a> {
    fn new() -> Self {
        Ctx {
            token_status: TOKEN_NAME,
            token_name: "",
            token_value: "",
            token_type: ""
        }
    }
}

pub fn run(source: &str) -> Vec<Token>{
    let tokens: HashMap<&str, &str> =  HashMap::from([
        ("a$", "OS"),
        ("}", "OE"),
        ("b$", "AS"),
        ("]", "AE"),
        (":", "KT"),
        (",", "LT"),
        ("0$", "UN"),
        ("1$", "NL"),
        ("2$", "BO"),
        ("3$", "ST"),
        ("4$", "NU"),
        ("5$", "IN"),
        ("6$", "NA"),
        ("7$", "DT"),
        ("8$", "BI"),
        ("9$", "FU")
    ]);

    let chars: Vec<char> = source.chars().collect();
    let mut ctx = Ctx::new();

    fn should_create_token_value (token_type: &'static str) -> bool {
        match token_type {
            "string" => true,
            "date" => true,
            "number" => true,
            "bigint" => true,
            "infinity" => true,
            "boolean" => true,
            "function" => true,
            _ => false
        }
    }
    
    let mut res: Vec<Token> = vec![];
    let iter: &std::str::Chars = &source.chars();
    for (i, _) in iter.clone().enumerate() {
        let char: Char = Char {
            prev_char: if i > 0 { Some(chars[&i - 1]) } else { None },
            cur_char: chars[i],
            next_char: if i < chars.len() - 1 {Some(chars[&i + 1])} else { None }
        };

        match ctx.token_status {
            "TOKEN_NAME" => {
                let mut name: Option<&&str> = tokens.get(&char.cur_char.to_string() as &str);

                if char.prev_char != None && name == None {
                    let pair = &(char.prev_char.expect("The prev char should not be None").to_string() + &char.cur_char.to_string() ) as &str;
                    name = tokens.get(pair);
                }

                if name != None {
                    ctx.token_name = name.expect("The token's name should exist");

                    let token_type = TOKEN_TYPES.get(ctx.token_name).unwrap_or_else(|| panic!("There is no token type found for token {}", ctx.token_name));

                    ctx.token_type = *token_type;
                    
                    if !should_create_token_value(token_type) {
                        res.push(Token { name: ctx.token_name.to_string(), value: ctx.token_value.to_string() });
                        ctx = Ctx::new();
                    } else {
                        ctx.token_status = TOKEN_VALUE;
                    }

                    continue;
                }

                continue;
                
            },
            "TOKEN_VALUE" => {
                // @TODO: this code avoids generation of String type which exists in Heap
                // we must to allocate new memory when we push tokens instead of borrowing them
                let mut b = [0; 8];

                let buffer_char = char.cur_char.encode_utf8(&mut b);
                let together = format!("{}{}", ctx.token_value, buffer_char);

                ctx.token_value = Box::leak(together.into_boxed_str());

                let mut is_finished: bool = false;

                match ctx.token_type {
                    "string" => {
                        is_finished = accumulate_string(char);
                    },
                    "date" => {
                        is_finished = accumulate_date(char);
                    },
                    "number" => {
                        is_finished = accumulate_number(char);
                    },
                    "bigint" => {
                        is_finished = accumulate_number(char);
                    },
                    "infinity" => {
                        is_finished = accumulate_infinity(char);
                    },
                    "boolean" => {
                        is_finished = accumulate_boolean(char);
                    },
                    "function" => {
                        is_finished = accumulate_function(char);
                    },
                    _ => panic!("There is no accumulator for specific token type {}", ctx.token_type)
                }

                if is_finished == true {
                    res.push(Token { name: ctx.token_name.to_string(), value: ctx.token_value.to_string() });

                    ctx = Ctx::new();
                }
            },
            _ => panic!("There is an unsknown token status {}", ctx.token_status),
        }
    }

    res
}