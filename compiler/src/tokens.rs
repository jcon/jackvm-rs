//! Tokenizer for Jack language.
//!
//! Provides a [`tokenize`] function for breaking a Jack language source file
//! into a Vec of tokens. Token types can be either keywords, symbols, identifiers,
//! int constants, or string constants.

use std::vec::Vec;

/// An enum representing all possible Jack language keywords.
#[derive(Debug, PartialEq)]
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

/// Represents the different types of tokens that exist in the Jack language.
#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Symbol(char),
    Identifier(String),
    IntConstant(i16),
    StringConstant(String),
}

/// Returns true if the given char is one the Jack language symbols.
/// NOTE: this function does not consider (") a symbol, because the
/// tokenizer handles string constants directly.
///
/// NOTE: lazy_static HashMap would have bee more concise, but lazy_static
/// uses features that add 36k to WASM output.
fn is_symbol(c: char) -> bool {
    match c {
        '{' | '}' | '(' | ')' | '[' | ']' | '.' | ',' | ';' | '+' | '-' | '*' | '/' | '&' | '|'
        | '<' | '>' | '=' | '~' => true,
        _ => false,
    }
}

/// Returns a Symbol token if the specified char is a Jack language token
/// otherwise None.
fn parse_symbol_char(c: char) -> Option<Token> {
    if is_symbol(c) {
        //        println!("{} is a symbol", c);
        Some(Token::Symbol(c))
    } else {
        None
    }
}

/// Returns a Keyword token for the specified word if it is one
/// of the Jack language keywords, otherwise None.
///
/// NOTE: a lazy_static HashMap would have bee more concise, but lazy_static
/// uses features that add 36k to WASM output.
fn parse_keyword(word: &str) -> Option<Token> {
    match word {
        "class" => Some(Token::Keyword(Keyword::CLASS)),
        "method" => Some(Token::Keyword(Keyword::METHOD)),
        "function" => Some(Token::Keyword(Keyword::FUNCTION)),
        "constructor" => Some(Token::Keyword(Keyword::CONSTRUCTOR)),
        "int" => Some(Token::Keyword(Keyword::INT)),
        "boolean" => Some(Token::Keyword(Keyword::BOOLEAN)),
        "char" => Some(Token::Keyword(Keyword::CHAR)),
        "void" => Some(Token::Keyword(Keyword::VOID)),
        "var" => Some(Token::Keyword(Keyword::VAR)),
        "static" => Some(Token::Keyword(Keyword::STATIC)),
        "field" => Some(Token::Keyword(Keyword::FIELD)),
        "let" => Some(Token::Keyword(Keyword::LET)),
        "do" => Some(Token::Keyword(Keyword::DO)),
        "if" => Some(Token::Keyword(Keyword::IF)),
        "else" => Some(Token::Keyword(Keyword::ELSE)),
        "while" => Some(Token::Keyword(Keyword::WHILE)),
        "return" => Some(Token::Keyword(Keyword::RETURN)),
        "true" => Some(Token::Keyword(Keyword::TRUE)),
        "false" => Some(Token::Keyword(Keyword::FALSE)),
        "null" => Some(Token::Keyword(Keyword::NULL)),
        "this" => Some(Token::Keyword(Keyword::THIS)),
        _ => None,
    }
}

/// Returns an integer constant token if this function can parse
/// the specified word as an i16, otherwise None.
fn parse_int(word: &str) -> Option<Token> {
    word.parse::<i16>().map(|i| Token::IntConstant(i)).ok()
}

/// Returns a string constant if s begins with '"', otherwise None.
fn parse_string_constant(s: &str) -> Option<Token> {
    match s.chars().next() {
        Some(c) if c != '"' => return None,
        _ => (),
    }

    let ptr = &s[1..];

    let end = ptr.find('"').unwrap_or(ptr.len());

    let word = ptr[0..end].to_string();

    Some(Token::StringConstant(word))
}

/// Returns a reference to the slice of ptr that starts with needle
/// otherwise returns an empty string.
fn advance_to<'a>(ptr: &'a str, needle: &str) -> &'a str {
    match ptr.find(needle) {
        Some(i) => &ptr[i..],
        None => "",
    }
}

/// Returns a reference to the slice of ptr that starts right _after_ needle
/// otherwise returns an empty string.
fn advance_to_after<'a>(ptr: &'a str, needle: &str) -> &'a str {
    let ptr = advance_to(ptr, needle);
    if needle.len() < ptr.len() {
        &ptr[needle.len()..]
    } else {
        ""
    }
}

/// Returns the next word (sequence of characters that are neither whitespace
/// nor a symbol).
fn next_word<'a>(ptr: &'a str) -> (&'a str, &'a str) {
    let end = ptr
        .find(|c: char| c.is_whitespace() || is_symbol(c))
        .unwrap_or(ptr.len());
    (&ptr[..end], &ptr[end..])
}

/// Translates the string into a Vec of Jack language tokens.
/// This function performs no lexical analysis of the program.
pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    // Keep a pointer to our current position in the source.
    // We update the pointer as we consume chars to make
    // up each token.
    let mut ptr = source;

    while ptr.len() > 0 {
        let next_c = ptr.chars().next().unwrap();
        if next_c.is_whitespace() {
            ptr = &ptr[1..];
        } else if ptr.starts_with("//") {
            ptr = advance_to_after(ptr, "\n");
        } else if ptr.starts_with("/*") {
            ptr = advance_to_after(&ptr[2..], "*/");
        } else {
            let token = parse_symbol_char(next_c) // try parsing symbol
                .and_then(|t| {
                    ptr = &ptr[1..];
                    Some(t)
                })
                .or_else(|| { // try parsing string constant
                    let s = parse_string_constant(ptr);
                    if s.is_some() {
                        // move pointer past second '"'
                        ptr = advance_to_after(ptr, "\"");
                        ptr = advance_to_after(ptr, "\"");
                    }
                    s
                })
                .or_else(|| { // try parsing word as keyword, int, or identifier
                    let (word, next) = next_word(ptr);
                    ptr = next;
                    parse_keyword(&word)
                        .or_else(|| parse_int(&word))
                        .or_else(|| Some(Token::Identifier(word.to_string())))
                });
            if let Some(tok) = token {
                tokens.push(tok);
            }
        };
    }

    tokens
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_empty_program() {
        let result = tokenize("");
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_parse_simple_expression() {
        let expected = vec![
            Token::Identifier("a".to_string()),
            Token::Symbol('+'),
            Token::IntConstant(5),
        ];

        // The same expression should parse with / without whitespace
        let result = tokenize("a+5");
        assert_eq!(result, expected);

        let result = tokenize("a + 5");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_string_constant() {
        let expected = vec![
            Token::Keyword(Keyword::LET),
            Token::Identifier("world".to_string()),
            Token::Symbol('='),
            Token::StringConstant("+hello;".to_string()),
            Token::Symbol(';'),
        ];

        let result = tokenize("let world = \"+hello;\" ;");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_line_comment() {
        let expected = vec![
            Token::Keyword(Keyword::LET),
            Token::Identifier("a".to_string()),
            Token::Symbol('='),
            Token::Identifier("b".to_string()),
            Token::Symbol('-'),
            Token::Identifier("c".to_string()),
            Token::Symbol(';'),
        ];

        let result = tokenize("// a comment\nlet a = b // more comment\n - c;");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multi_line_comment_with_close() {
        let expected = vec![
            Token::Keyword(Keyword::IF),
            Token::Symbol('('),
            Token::Identifier("x".to_string()),
            Token::Symbol('='),
            Token::Identifier("y".to_string()),
            Token::Symbol(')'),
            Token::Symbol('{'),
            Token::Symbol('}'),
        ];

        let result = tokenize("/* a comment*/\nif (x = y) {\n /*/ more\n comments\n*/\n}");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multi_line_comment_no_close() {
        let expected = vec![
            Token::Keyword(Keyword::FUNCTION),
            Token::Symbol('('),
            Token::Keyword(Keyword::INT),
            Token::Identifier("x".to_string()),
            Token::Symbol(')'),
            Token::Symbol('{'),
        ];

        let result = tokenize("function (int x) { /* }");
        assert_eq!(result, expected);
    }
}
