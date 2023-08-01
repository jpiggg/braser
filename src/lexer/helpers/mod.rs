pub fn is_digit(char: Option<char>) -> bool {
    match char {
        Some(char) => char >= '0' && char <= '9',
        None => false
    }
}

pub fn is_sign(char: Option<char>) -> bool {
    match char {
        Some (char) => char == '-' || char == '+',
        None => false
    }
}

pub fn is_exp(char: Option<char>) -> bool {
    match char {
        Some(char) => char == 'e' || char == 'E',
        None => false
    }
}

pub fn is_single_quote(char: Option<char>) -> bool {
    match char {
        Some(char) => char == '\'',
        None => false
    }
}

pub fn is_double_quote(char: Option<char>) -> bool {
    match char {
        Some(char) => char == '"',
        None => false
    }
}