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

fn parse_symbol(chars: &mut Peekable<Chars<'_>>) -> Option<Token> {
    let next_c = chars.peek();
    if let Some(c) = next_c {
        parse_symbol_char(*c).and_then(|s| {
            chars.next();
            Some(s)
        })
    } else {
        None
    }
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

fn next_word(it: &mut Peekable<Chars<'_>>) -> String {
    let mut word = String::new();
    loop {
        let opt_c = it.peek();
        if let Some(c) = opt_c {
            if is_whitespace(*c) || is_symbol(*c) {
                break;
            }
            word.push(it.next().unwrap());
        } else {
            break;
        }
    }
    word
}

// struct Tokenizer<'a> {
//     source: Chars<'a>,
//     position: Option<usize>,
// }

// impl<'a> Tokenizer<'a> {
//     pub fn new(source: &str) -> Tokenizer {
//         Tokenizer {
//             source: source.chars(),
//             position: None,
//         }
//     }
//     // pub fn get_next_pos(&self, start: usize) -> Option<usize> {
//     //     let look_ahead = Some(start + 1);
//     //     loop {
//     //         let next_index: usize = match look_ahead {
//     //             None => return None,
//     //             Some(n) => n
//     //         };
//     //         let c: char = self.source[next_index];
//     //     }
//     //     0
//     // }

// }

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut chars = source.chars().peekable();

    println!("got some chars");

    // TODO: handle comments
    loop {
        match chars.peek() {
            Some(c) if is_whitespace(*c) => {
                chars.next();
                continue;
            }
            None => break,
            _ => (),
        };

        println!("next c is {}", chars.peek().unwrap());

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
        let result = tokenize("a + 5");
        assert_eq!(
            result,
            vec![
                Token::Identifier("a".to_string()),
                Token::Symbol('+'),
                Token::IntConstant(5),
            ]
        );
    }
}
