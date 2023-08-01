pub mod tokens;
pub mod helpers;

use std::fmt;
use std::collections::HashMap;
use helpers::{is_double_quote, is_single_quote, is_digit, is_sign, is_exp};
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
        writeln!(f, "Char consists from: \nprev_char: {}, \nchar: {}, \nnext_char: {}", self.prev_char.unwrap(), self.cur_char, self.next_char.unwrap_or_default())
    }
}

pub fn accumulate_string(char: Char) -> bool {
    println!("{}", char);
    if char.prev_char.unwrap() != '$' && is_double_quote(Some(char.cur_char)) && char.prev_char.unwrap() != '\\' {
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
    if is_digit(Some(char.cur_char)) || 
        is_sign(Some(char.cur_char)) && is_digit(char.next_char) ||
        char.cur_char == '.' && is_digit(char.next_char) ||
        char.cur_char == '.' && is_digit(char.prev_char) ||
        is_exp(Some(char.cur_char)) && is_sign(char.next_char) {
        return false;
    }

    true
}

pub fn accumulate_boolean(char: Char) -> bool {
    if char.cur_char == '1' || char.cur_char == '0' {
        return true;
    }

    false
}

pub fn accumulate_function(char: Char) -> bool {
    if char.cur_char == '}' && char.next_char.unwrap() == '$' {
        return true;
    }

    false
}


#[derive(Copy, Clone, Debug)]
pub struct Ctx<'a> (&'static str, &'a str, &'a str, Option<&'a str>);

impl<'a> Ctx<'a> {
    fn new () -> Self {
        Ctx (TOKEN_NAME, "", "", None)
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
            next_char: if i < chars.len() - 2 {Some(chars[&i + 1])} else { None }
        };


        match ctx.0 {
            "TOKEN_NAME" => {
                let mut name: Option<&&str> = tokens.get(&char.cur_char.to_string() as &str);

                if char.prev_char != None && name == None {
                    let pair = &(char.prev_char.unwrap().to_string() + &char.cur_char.to_string() ) as &str;
                    name = tokens.get(pair);
                }

                if name != None {
                    let token_type = TOKEN_TYPES.get(*name.clone().unwrap());

                    ctx.1 = name.unwrap();
                    ctx.3 = Some(*token_type.unwrap());
                    

                    if !should_create_token_value(token_type.unwrap()) {
                        res.push(Token { name: ctx.1, value: ctx.2 });
                    } else {
                        ctx.0 = TOKEN_VALUE;
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
                let together = format!("{}{}", ctx.2, buffer_char);

                ctx.2 = Box::leak(together.into_boxed_str());

                let mut is_finished: bool = false;

                match ctx.3 {
                    Some("string") => {
                        is_finished = accumulate_string(char);
                    },
                    Some("date") => {
                        is_finished = accumulate_date(char);
                    },
                    Some("number") => {
                        is_finished = accumulate_number(char);
                    },
                    Some("bigint") => {
                        is_finished = accumulate_number(char);
                    },
                    Some("boolean") => {
                        is_finished = accumulate_boolean(char);
                    },
                    Some("function") => {
                        is_finished = accumulate_function(char);
                    },
                    None => println!("There is no token types provided"),
                    _ => println!("The default behavior")
                }


                if is_finished == true {
                    res.push(Token { name: ctx.1, value: ctx.2 });

                    ctx = Ctx::new();
                }
            },
            _ => println!("There is no accumulators for specific token type {}", ctx.3.unwrap()),
        }
    }

    println!("RES VARIABLE IS {:?}", res);

    res
}