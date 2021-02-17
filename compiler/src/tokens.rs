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
    THIS
}

pub enum Token {
    Keyword(Keyword),
    Symbol(char),
    Identifier(String),
    IntConstant(i16),
    StringConstant(String)
}

pub fn parse_symbol(c: char) -> Option<Token> {
    Some(Token::Symbol(c))
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let s = parse_symbol('+');
    match s {
        Some(t) => vec!(t),
        _ => vec!()
    }
}