//! Lexer implementation

// This is the state machine we'll be using to represent lexemes

use std::{iter::Peekable, path::Iter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lexeme {
    ByteToken(Token),
    VariableByteToken(VarToken),
    TokenLiteral(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Eq,
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    OpenSquare,
    CloseSquare,
    Newline,
    Comma,
    Ampersand,
    Semicolon,
    Colon,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VarToken {
    // 'fn' function deceleration
    Fn,
    // let variable decleration
    Let,
    // If statement
    If,
    // Else statement
    Else,
    // struct deceleration
    Struct,
    // enum deceleration
    Enum,
}

// This function maps a `char`
// to a Token value, if the byte isn't a Token then none is returned
impl Token {
    // This returns an option because we cannot always map
    // the char to a token, we only map keywords we care about
    fn from_char(racter: u8) -> Option<Token> {
        match racter {
            b'=' => Some(Token::Eq),
            b'(' => Some(Token::OpenParen),
            b')' => Some(Token::CloseParen),
            b'}' => Some(Token::CloseCurly),
            b'{' => Some(Token::OpenCurly),
            b';' => Some(Token::Semicolon),
            b',' => Some(Token::Comma),
            b']' => Some(Token::CloseSquare),
            b'[' => Some(Token::OpenSquare),
            b'\n' => Some(Token::Newline),
            b'&' => Some(Token::Ampersand),
            b':' => Some(Token::Colon),
            _ => None,
        }
    }
}

impl Lexeme {
    // tokens we know are classified as Lexemes::ByteToken
    // in the sequence, anything we don't know about needs to be
    // classified as Lexemes::TokenLiteral(Vec<u8>)
    pub fn from_literal(literal: Vec<u8>) -> Vec<Lexeme> {
        let mut peekable = literal.iter().peekable();
        // pre-reserve some space on the heap
        // so we dont need to grow when pushing in new tokens
        let mut lexems = Vec::with_capacity(literal.len());
        let mut current_literal: Vec<u8> = Vec::new();

        while let Some(curr_token) = peekable.peek() {
            let curr_token = **curr_token;
            // First check if our current token is a keyword or not
            if let Some(x) = Token::from_char(curr_token) {
                if current_literal.len() > 0 {
                    lexems.push(Lexeme::TokenLiteral(current_literal.clone()));
                    current_literal = Vec::new();
                }

                lexems.push(Lexeme::ByteToken(x.clone()));
                peekable.next();

                flush_whitespace(&mut peekable);
                flush_newline(&mut peekable);
                continue;
            }

            peekable.next();

            match curr_token {
                // possible function deceleration
                b'f' => {
                    if let Some(possible_n) = peekable.peek() {
                        if **possible_n == b'n' {
                            flush_curr_literal(&mut lexems, &mut current_literal);

                            lexems.push(Lexeme::VariableByteToken(VarToken::Fn));
                            peekable.next();

                            flush_whitespace(&mut peekable);
                        } else {
                            current_literal.push(curr_token);
                        }
                    }
                }
                // possible let variable deceleration
                b'l' => {
                    let mut cloned = peekable.clone();
                    let e = cloned.next();
                    let t = cloned.next();

                    match (e, t) {
                        (Some(b'e'), Some(b't')) => {
                            flush_curr_literal(&mut lexems, &mut current_literal);

                            lexems.push(Lexeme::VariableByteToken(VarToken::Let));
                            peekable.next();
                            peekable.next();

                            flush_whitespace(&mut peekable);
                        }
                        _ => {
                            current_literal.push(curr_token);
                        }
                    }
                }
                // possible if variable deceleration
                b'i' => {
                    if let Some(b'f') = peekable.peek() {
                        flush_curr_literal(&mut lexems, &mut current_literal);

                        lexems.push(Lexeme::VariableByteToken(VarToken::If));
                        peekable.next();
                    } else {
                        current_literal.push(curr_token);
                    }
                }
                // possible else variable deceleration or enum
                b'e' => {
                    let mut cloned = peekable.clone();
                    let l = cloned.next();
                    let s = cloned.next();
                    let e = cloned.next();

                    match (l, s, e) {
                        (Some(b'l'), Some(b's'), Some(b'e')) => {
                            flush_curr_literal(&mut lexems, &mut current_literal);

                            lexems.push(Lexeme::VariableByteToken(VarToken::Else));
                            peekable.next();
                            peekable.next();
                            peekable.next();
                        }
                        (Some(b'n'), Some(b'u'), Some(b'm')) => {
                            flush_curr_literal(&mut lexems, &mut current_literal);

                            lexems.push(Lexeme::VariableByteToken(VarToken::Enum));
                            peekable.next();
                            peekable.next();
                            peekable.next();
                        }

                        _ => {
                            current_literal.push(curr_token);
                        }
                    }
                }
                b's' => {
                    let mut cloned = peekable.clone();
                    let t = cloned.next();
                    let r = cloned.next();
                    let u = cloned.next();
                    let c = cloned.next();
                    let t_nd = cloned.next();

                    match (t, r, u, c, t_nd) {
                        (Some(b't'), Some(b'r'), Some(b'u'), Some(b'c'), Some(b't')) => {
                            flush_curr_literal(&mut lexems, &mut current_literal);

                            lexems.push(Lexeme::VariableByteToken(VarToken::Struct));
                            peekable.next();
                            peekable.next();
                            peekable.next();
                            peekable.next();
                            peekable.next();

                            flush_whitespace(&mut peekable);
                        }
                        _ => {
                            current_literal.push(curr_token);
                        }
                    }
                }
                _ => {
                    current_literal.push(curr_token);
                }
            }
        }

        lexems
    }
}

fn flush_curr_literal(lexems: &mut Vec<Lexeme>, current_literal: &mut Vec<u8>) {
    if current_literal.len() > 0 {
        lexems.push(Lexeme::TokenLiteral(current_literal.clone()));

        current_literal.clear();
    }
}

fn flush_whitespace<'a, I: Iterator<Item = &'a u8>>(peekable: &mut Peekable<I>) {
    while Some(&&b' ') == peekable.peek() {
        peekable.next();
    }
}

fn flush_newline<'a, I: Iterator<Item = &'a u8>>(peekable: &mut Peekable<I>) {
    while Some(&&b'\n') == peekable.peek() {
        peekable.next();
    }
}

fn main() {
    let code = "fn main() {
        let array = [0, 1, 2];

        if true { test } else {};
    }";

    let bytes = code.as_bytes().to_vec();

    println!("code: {}", code);

    println!("Lexed: {:?}", Lexeme::from_literal(bytes));
}

#[test]
fn test_with_all_keywords() {
    use Lexeme::*;
    use Token::*;
    use VarToken::*;

    let code = "fn(){let[,];ifconditionelse&}";

    let bytes = code.as_bytes().to_vec();

    let lexed = Lexeme::from_literal(bytes);

    assert_eq!(
        vec![
            VariableByteToken(Fn),
            ByteToken(OpenParen),
            ByteToken(CloseParen),
            ByteToken(OpenCurly),
            VariableByteToken(Let),
            ByteToken(OpenSquare),
            ByteToken(Comma),
            ByteToken(CloseSquare),
            ByteToken(Semicolon),
            VariableByteToken(If),
            TokenLiteral([99, 111, 110, 100, 116, 111, 110].to_vec()),
            VariableByteToken(Else),
            ByteToken(Ampersand),
            ByteToken(CloseCurly)
        ],
        lexed
    );
}

#[test]
fn test_normal_code() {
    use Lexeme::*;
    use Token::*;
    use VarToken::*;

    let code = "fn main() {
        let array = [0, 1, 2];

        if true { test } else {};
    }";

    let bytes = code.as_bytes().to_vec();

    let lexed = Lexeme::from_literal(bytes);

    assert_eq!(
        vec![
            TokenLiteral([32].to_vec()),
            VariableByteToken(Fn),
            TokenLiteral([32, 109, 97, 110].to_vec()),
            ByteToken(OpenParen),
            ByteToken(CloseParen),
            TokenLiteral([32].to_vec()),
            ByteToken(OpenCurly),
            ByteToken(Newline),
            TokenLiteral([32, 32, 32, 32, 32, 32, 32, 32].to_vec()),
            VariableByteToken(Let),
            TokenLiteral([32, 97, 114, 114, 97, 121, 32].to_vec()),
            ByteToken(Eq),
            TokenLiteral([32].to_vec()),
            ByteToken(OpenSquare),
            TokenLiteral([48].to_vec()),
            ByteToken(Comma),
            TokenLiteral([32, 49].to_vec()),
            ByteToken(Comma),
            TokenLiteral([32, 50].to_vec()),
            ByteToken(CloseSquare),
            ByteToken(Semicolon),
            ByteToken(Newline),
            ByteToken(Newline),
            TokenLiteral([32, 32, 32, 32, 32, 32, 32, 32].to_vec()),
            VariableByteToken(If),
            TokenLiteral([32, 116, 114, 117, 101, 32].to_vec()),
            ByteToken(OpenCurly),
            TokenLiteral([32, 116, 101, 115, 116, 32].to_vec()),
            ByteToken(CloseCurly),
            TokenLiteral([32].to_vec()),
            VariableByteToken(Else),
            TokenLiteral([32].to_vec()),
            ByteToken(OpenCurly),
            ByteToken(CloseCurly),
            ByteToken(Semicolon),
            ByteToken(Newline),
            TokenLiteral([32, 32, 32, 32].to_vec()),
            ByteToken(CloseCurly)
        ],
        lexed
    );
}
