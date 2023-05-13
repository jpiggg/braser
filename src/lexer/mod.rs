pub mod tokens;
pub mod helpers;

use std::collections::HashMap;
use helpers::{is_quote, is_digit, is_exp, is_sign};


#[derive(Clone, Copy)]
pub struct Char {
    prev_char: Option<char>,
    cur_char:  char,
    next_char: Option<char>
}

#[derive(Clone, Copy, Debug)]
pub struct Token {
    name:  &'static str,
    value: &'static str
}

#[derive(Clone)]
struct State {
    status: &'static str,
    chars: Vec<char>,
    token_type: Option<&'static str>,
    token: Token,
    res: Vec<Token>
}

impl State{
    fn create(token_name: &'static str, chars: &str) -> Self {
        Self {
            status: token_name,
            chars: chars.chars().collect(),
            token_type: None,
            token: Token {
                name: "",
                value: ""
            },
            res: vec![]
        }
    }
    fn update_status (&mut self, status: &'static str) {
        self.status = status;
    }
    fn update_token_name (&mut self, token_name: &'static str) {
        self.token.name = token_name
    }
    fn update_token_val(&mut self, token_value: String) {
        self.token.value.to_string().push_str(&token_value)
    }
    fn update_token_type(&mut self, token_type:&'static str) {
        self.token_type = Some(token_type)
    }
    fn clear_token(&mut self) {
        self.token = Token {
            name: "",
            value: ""
        }
    }

}

pub fn accumulate_string(char: Char) -> bool {
    if is_quote(Some(char.cur_char)) && char.prev_char.unwrap() != '\\' && char.prev_char.unwrap() != '$' {
        return false;
    }

    true

}
pub fn accumulate_number(char: Char) -> bool {
    if is_digit(Some(char.cur_char)) ||
        is_sign(Some(char.cur_char)) && is_digit(char.next_char) ||
        char.cur_char == '.' && is_digit(char.next_char) ||
        is_sign(Some(char.cur_char)) && is_digit(char.next_char) ||
        is_exp(Some(char.cur_char)) && is_sign(char.next_char) {
            return true;
        }

        false
}

pub fn accumulate_boolean(char: Char) -> bool {
    // if (state.char === '1' || state.char === '0') {
	// 	return true;
	// }

	// return false;
    if char.cur_char == '1' || char.cur_char == '0' {
        return true;
    }

    false
}
// pub fn accumulate_function(char: Char) -> bool {true}

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

    const TOKEN_NAME: &str = "TOKEN_NAME";
    const TOKEN_VALUE: &str = "TOKEN_VALUE";
    let state: State = State::create(TOKEN_NAME, source);
    let iter: &std::str::Chars = &source.chars();

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


fn accumulate<F>(closure: F, state: &State, char: Char) -> ()
    where F: FnOnce() -> bool,
    {   
        // the flag is_finished means that the next char is not value of the current token
        let is_finished: bool = closure();
        let mut cloned_state = state.clone();

        cloned_state.update_token_val(char.cur_char.to_string());

        if is_finished {
            cloned_state.update_status(TOKEN_NAME);
            cloned_state.res.push(cloned_state.token);
            cloned_state.clear_token();
        }
    }

    let create_token_fn = |token: &'static str| {
        let mut cloned_state: State = state.clone();
        cloned_state.update_token_name(token);
        let token_type: Option<&&str> = token_types.get(token);

        if should_create_token_value(token_type.unwrap()) {
            cloned_state.update_status(TOKEN_VALUE);
        } else {
            cloned_state.res.push(Token{
                name: token,
                value: ""
            });


            cloned_state.update_token_type(token_type.unwrap());
        }
    };

    for (i, _) in iter.clone().enumerate() {
        let char: Char = Char {
            prev_char: if i > 0 { Some(state.chars[i - 1]) } else { None },
            cur_char: state.chars[i],
            next_char: if i < state.chars.len() - 2 {Some(state.chars[i + 1])} else { None }
        };

        match state.status {
            "TOKEN_NAME" => {
                println!("Creating token name....");

                let c: Option<&&str> = tokens.get(&char.cur_char.to_string() as &str);

                if c != None {
                    create_token_fn(*c.clone().unwrap());

                    continue;
                }

                if char.prev_char != None {
                    let c2: Option<&&str> = tokens.get(&(char.cur_char.to_string() + &char.prev_char.unwrap().to_string()) as &str);

                    if c2 != None {
                        create_token_fn(&c.unwrap());

                        continue;
                    }
                }

                continue;
                
            },
            "TOKEN_VALUE" => {
                match state.token_type {
                    Some("string") => {
                        accumulate(|| {accumulate_string(char)}, &state, char);
                    },
                    Some("number") => {
                        accumulate(|| {accumulate_number(char)}, &state, char);
                    },
                    Some("bigint") => {
                        accumulate(|| {accumulate_number(char)}, &state, char);
                    },
                    Some("boolean") => {
                        accumulate(|| {accumulate_boolean(char)}, &state, char);
                    },
                    // Some("function") => {
                    //     accumulate(|| {accumulate_function(char)}, &state, char);
                    // },
                    None => println!("There is no token types provided"),
                    _ => println!("The default behavior")
                }

                println!("Creating token value....");
            },
            _ => println!("There is no accumulators for specofic token type {}", state.token_type.unwrap()),
        }
    }

    state.res
}