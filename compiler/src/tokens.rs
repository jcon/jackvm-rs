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
    chars.get_current().and_then(|c| parse_symbol_char(c))
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

fn parse_string(it: &mut Tokenizer) -> Option<Token> {
    match it.get_current() {
        Some(c) if c != '"' => return None,
        _ => (),
    }

    let mut word = String::new();
    it.read_until(&mut word, |c| c != '"');

    it.next();

    Some(Token::StringConstant(word))
}

fn is_whitespace(c: char) -> bool {
    println!("is_whitespace? {}", c);
    match c {
        ' ' | '\t' | '\n' | '\r' => true,
        _ => false,
    }
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
        let pos = self.pos as usize;
        if pos < self.chars.len() {
            self.current = Some(self.chars[pos]);
            self.current
        } else {
            None
        }
    }

    pub fn has_next<F>(&mut self, test: F) -> bool
    where
        F: FnOnce(char) -> bool,
    {
        let pos = (self.pos + 1) as usize;
        pos < self.chars.len() && test(self.chars[pos])
    }

    pub fn get_current(&self) -> Option<char> {
        self.current
    }

    pub fn backup(&mut self) {
        if self.pos >= 0 {
            self.pos -= 1;
        }
    }

    pub fn read_until<F>(self: &mut Tokenizer, dest: &mut String, test: F)
    where
        F: FnOnce(char) -> bool + Copy,
    {
        while self.has_next(test) {
            let next = self.next().unwrap();
            dest.push(next);
        }
    }

    fn next_word(self: &mut Tokenizer) -> String {
        let mut word = String::new();
        word.push(self.get_current().unwrap());

        let is_word_char = |c| !is_whitespace(c) && !is_symbol(c);
        self.read_until(&mut word, is_word_char);
        word
    }

    pub fn matches(&mut self, pattern: &str) -> bool {
        let pos = self.pos as usize;
        if pos + pattern.len() >= self.chars.len() {
            return false;
        }

        let source = &self.chars[pos..(pos + pattern.len())];
        let pattern: Vec<char> = pattern.chars().collect();
        source == &pattern
    }

}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut chars = Tokenizer::new(source.chars());

    println!("got some chars");

    let mut dummy = String::new();
    // TODO: handle multi-line comments
    loop {
        match chars.next() {
            Some(c) if is_whitespace(c) => {
                continue;
            },
            Some(_) if chars.matches("//") => {
                println!("matched comment begin");
                chars.read_until(&mut dummy, |c| c != '\n');
                continue;
            },
            None => break,
            _ => (),
        };

        println!("next c is {}", chars.get_current().unwrap());

        let token = parse_symbol(&mut chars)
            .or_else(|| parse_string(&mut chars))
            .or_else(|| {
                let word = chars.next_word();
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
        assert_eq!(result, expected);

        let result = tokenize("a + 5");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_string() {
        let expected = vec![
            Token::Keyword(Keyword::LET),
            Token::Identifier("b".to_string()),
            Token::Symbol('='),
            Token::StringConstant("+hello;".to_string()),
            Token::Symbol(';'),
        ];

        let result = tokenize("let b = \"+hello;\" ;");
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
}
