pub mod tokens;
pub mod helpers;

use std::{collections::HashMap};
use helpers::{is_quote};

const TOKEN_NAME: &'static str = "TOKEN_NAME";
const TOKEN_VALUE: &'static str  = "TOKEN_VALUE";


#[derive(Clone, Copy, Debug)]
pub struct Char {
    prev_char: Option<char>,
    cur_char:  char,
    next_char: Option<char>
}

pub fn accumulate_string(char: Char) -> bool {
    println!("Here is accumulate_string, {:?}", char);
    if is_quote(char.prev_char) && char.cur_char == '$' && char.next_char.unwrap() == '3' {
        return true;
    }

    false

}
pub fn accumulate_date(char: Char) -> bool {
    if is_quote(char.next_char) || char.next_char.unwrap() == ',' {
        return true;
    }

    false
}
pub fn accumulate_number(char: Char) -> bool {
    if is_quote(char.next_char) || char.next_char.unwrap() == ',' {
        return true;
    }

    false
}

pub fn accumulate_boolean() -> bool {
    true
}

#[derive(Copy, Clone, Debug)]
pub struct Ctx<'a> (&'static str, &'a str, &'a str, Option<&'a str>);

impl<'a> Ctx<'a> {
    fn new () -> Self {
        Ctx (TOKEN_NAME, "", "", None)
    }
}

#[derive(Debug)]
pub struct Token<'a> {
    name: &'a str,
    value: &'a str
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
        ("6$", "NA"),
        ("7$", "DT"),
        ("8$", "BI"),
        ("9$", "FU")
    ]);

    let token_types :HashMap<&str, &str> = HashMap::from([
        ("OS", "object"),
        ("OE", "object"),
        ("AS", "array"),
        ("AE", "array"),
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

    let chars: Vec<char> = source.chars().collect();
    let mut ctx = Ctx::new();

    fn should_create_token_value (token_type: &'static str) -> bool {
        match token_type {
            "string" => true,
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
                println!("Creating token name....for {:?}", char);

                let mut name: Option<&&str> = tokens.get(&char.cur_char.to_string() as &str);

                if char.prev_char != None && name == None {
                    let pair = &(char.prev_char.unwrap().to_string() + &char.cur_char.to_string() ) as &str;
                    name = tokens.get(pair);
                }

                if name != None {
                    let token_type = token_types.get(*name.clone().unwrap());

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
                    Some("number") => {
                        is_finished = accumulate_number(char);
                    },
                    Some("bigint") => {
                        is_finished = accumulate_number(char);
                    },
                    Some("boolean") => {
                        is_finished = accumulate_boolean();
                    },
                    None => println!("There is no token types provided"),
                    _ => println!("The default behavior")
                }


                if is_finished == true {
                    res.push(Token { name: ctx.1, value: ctx.2 });

                    ctx = Ctx::new();
                }

                println!("Creating token value....");
            },
            _ => println!("There is no accumulators for specific token type {}", ctx.3.unwrap()),
        }
    }

    println!("RES VARIABLE IS {:?}", res);

    res
}