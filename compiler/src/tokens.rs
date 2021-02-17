use std::vec::Vec;

pub enum Keyword {
    CLASS,
    METHOD,
    FUNCTION,
    CONSTRUCTOR,
    INT,
    BOOLEAN,
    CHAR,
    VOID,
    VAR,
    STATIC,
    FIELD,
    LET,
    DO,
    IF,
    ELSE,
    WHILE,
    RETURN,
    TRUE,
    FALSE,
    NULL,
    THIS,
}

pub enum Token {
    Keyword(Keyword),
    Symbol(char),
    Identifier(String),
    IntConstant(i16),
    StringConstant(String),
}

// NOTE: lazy_static would have bee more concise, but it uses features that add 36k to WASM output.
pub fn parse_symbol(c: char) -> Option<Token> {
    match c {
        '{' | '}' | '(' | ')' | '[' | ']' | '.' | ',' | ';' | '+' | '-' | '*' | '/' | '&' | '|'
        | '<' | '>' | '=' | '~' => Some(Token::Symbol(c)),
        _ => None,
    }
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let s = parse_symbol('+');
    match s {
        Some(t) => vec![t],
        _ => vec![],
    }
}
