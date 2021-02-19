use std::iter::Peekable;
use std::str::Chars;
use std::vec::Vec;

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

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Symbol(char),
    Identifier(String),
    IntConstant(i16),
    StringConstant(String),
}

// NOTE: lazy_static HashMap would have bee more concise, but lazy_static uses features that add 36k to WASM output.
fn is_symbol(c: char) -> bool {
    match c {
        '{' | '}' | '(' | ')' | '[' | ']' | '.' | ',' | ';' | '+' | '-' | '*' | '/' | '&' | '|'
        | '<' | '>' | '=' | '~' => true,
        _ => false,
    }
}
fn parse_symbol_char(c: char) -> Option<Token> {
    if is_symbol(c) {
        println!("{} is a symbol", c);
        Some(Token::Symbol(c))
    } else {
        None
    }
}

fn parse_symbol(chars: &mut Tokenizer) -> Option<Token> {
    chars.get_current()
        .and_then(|c| parse_symbol_char(c))
}

// NOTE: a lazy_static HashMap would have bee more concise, but lazy_static uses features that add 36k to WASM output.
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

fn parse_int(word: &str) -> Option<Token> {
    word.parse::<i16>().map(|i| Token::IntConstant(i)).ok()
}

fn is_whitespace(c: char) -> bool {
    println!("is_whitespace? {}", c);
    match c {
        ' ' | '\t' | '\n' | '\r' => true,
        _ => false,
    }
}

fn next_word(it: &mut Tokenizer) -> String {
    let mut word = String::new();
    word.push(it.get_current().unwrap());
    loop {
        let opt_c = it.next().and_then(|c| {
            if is_whitespace(c) || is_symbol(c) {
                it.backup();
                return None
            }
            word.push(c);
            Some(true)
        });

        if let None = opt_c {
            break;
        }
    }
    word
}

struct Tokenizer {
    chars: Vec<char>,
    current: Option<char>,
    pos: i32,
}

impl Tokenizer {
    pub fn new(chars: Chars<'_>) -> Tokenizer {
        Tokenizer {
            chars: chars.collect(),
            current: None,
            pos: -1,
        }
    }

    pub fn next(&mut self) -> Option<char> {
        self.pos += 1;
        if (self.pos as usize) < self.chars.len() {
            self.current = Some(self.chars[self.pos as usize]);
            self.current
        } else {
            None
        }
    }

    pub fn get_current(&self) -> Option<char> {
        self.current
    }

    pub fn backup(&mut self) {
        if self.pos >= 0 {
            self.pos -= 1;
        }
    }
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut chars = Tokenizer::new(source.chars());

    println!("got some chars");

    // TODO: handle comments
    loop {
        match chars.next() {
            Some(c) if is_whitespace(c) => {
                continue;
            }
            None => break,
            _ => (),
        };

        println!("next c is {}", chars.get_current().unwrap());

        let token = parse_symbol(&mut chars).or_else(|| {
            let word = next_word(&mut chars);
            println!("got word {}", word);
            parse_keyword(&word)
                .or_else(|| parse_int(&word))
                .or_else(|| Some(Token::Identifier(word)))
        });
        tokens.push(token.unwrap());
    }

    tokens
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_simple_expression() {
        let expected = vec![
            Token::Identifier("a".to_string()),
            Token::Symbol('+'),
            Token::IntConstant(5),
        ];

        // The same expression should parse with / without whitespace
        let result = tokenize("a+5");
        assert_eq!(
            result,
            expected
        );

        let result = tokenize("a + 5");
        assert_eq!(
            result,
            expected
        );
    }
}
