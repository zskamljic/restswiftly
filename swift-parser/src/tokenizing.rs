use std::io::{BufReader, Read};

use anyhow::Result;
use utf8_chars::BufReadCharsExt;

use crate::Token;

pub(crate) fn tokenize(reader: impl Read) -> Result<Vec<Token>> {
    let mut reader = BufReader::new(reader);

    let mut tokens = vec![];
    let mut state = State::None;
    let mut buffer = String::new();
    for char in reader.chars() {
        let char = char?;
        match state {
            State::None => {
                if char.is_ascii_alphabetic() {
                    state = State::Ident;
                    buffer.push(char);
                } else if char == '{' {
                    tokens.push(Token::LeftBrace)
                } else if char == '}' {
                    tokens.push(Token::RightBrace);
                } else if char.is_ascii_whitespace() {
                    continue;
                } else if char == '/' {
                    state = State::CommentStart;
                } else if char == '(' {
                    tokens.push(Token::LeftParenthesis);
                } else if char == ')' {
                    tokens.push(Token::RightParenthesis);
                } else {
                    panic!("Unexpected character when reading token: {char}");
                }
            }
            State::Ident => {
                if char.is_ascii_alphanumeric() {
                    buffer.push(char)
                } else if char.is_ascii_whitespace() {
                    state = State::None;
                    tokens.push(Token::Identifier(buffer));
                    buffer = String::new();
                } else if char == '(' {
                    state = State::None;
                    tokens.push(Token::Identifier(buffer));
                    tokens.push(Token::LeftParenthesis);
                    buffer = String::new();
                } else {
                    panic!("Unexpected character when reading identifier: {char}");
                }
            }
            State::CommentStart => {
                if char == '/' {
                    state = State::LineComment;
                } else if char == '*' {
                    state = State::BlockComment;
                } else {
                    panic!("Unexpected comment start: {char}");
                }
            }
            State::LineComment => {
                if char == '\n' {
                    state = State::None;
                    tokens.push(Token::LineComment(buffer.trim().to_string()));
                    buffer = String::new();
                } else {
                    buffer.push(char);
                }
            }
            State::BlockComment => todo!("Implement block comment logic"),
        }
    }

    Ok(tokens)
}

enum State {
    None,
    Ident,
    CommentStart,
    LineComment,
    BlockComment,
}
