use std::io::{BufReader, Read};
use std::mem;

use anyhow::Result;
use utf8_chars::BufReadCharsExt;

use crate::Token;

pub(crate) fn tokenize(reader: impl Read) -> Result<Vec<Token>> {
    Tokenizer::new().tokenize(reader)
}

struct Tokenizer {
    state: State,
    buffer: String,
    tokens: Vec<Token>,
}

impl Tokenizer {
    fn new() -> Tokenizer {
        Tokenizer {
            state: State::None,
            buffer: String::new(),
            tokens: vec![],
        }
    }

    fn tokenize(mut self, reader: impl Read) -> Result<Vec<Token>> {
        let mut reader = BufReader::new(reader);

        for char in reader.chars() {
            let char = char?;
            match self.state {
                State::None => self.handle_unknown_token(char),
                State::Ident => self.handle_identifier(char),
                State::CommentStart => self.handle_comment_start(char),
                State::LineComment => self.handle_line_comment(char),
                State::BlockComment => todo!("Implement block comment logic"),
            }
        }

        Ok(self.tokens)
    }

    fn handle_unknown_token(&mut self, char: char) {
        if char.is_ascii_alphabetic() {
            self.state = State::Ident;
            self.buffer.push(char);
        } else if char == '{' {
            self.tokens.push(Token::LeftBrace)
        } else if char == '}' {
            self.tokens.push(Token::RightBrace);
        } else if char == '/' {
            self.state = State::CommentStart;
        } else if char == '(' {
            self.tokens.push(Token::LeftParenthesis);
        } else if char == ')' {
            self.tokens.push(Token::RightParenthesis);
        } else if !char.is_ascii_whitespace() {
            panic!("Unexpected character when reading token: {char}");
        }
    }

    fn handle_identifier(&mut self, char: char) {
        if char.is_ascii_alphanumeric() {
            self.buffer.push(char)
        } else if char.is_ascii_whitespace() {
            self.state = State::None;
            let identifier = mem::take(&mut self.buffer);
            self.tokens.push(Token::Identifier(identifier));
        } else if char == '(' {
            self.state = State::None;
            let identifier = mem::take(&mut self.buffer);
            self.tokens.push(Token::Identifier(identifier));
            self.tokens.push(Token::LeftParenthesis);
        } else {
            panic!("Unexpected character when reading identifier: {char}");
        }
    }

    fn handle_comment_start(&mut self, char: char) {
        if char == '/' {
            self.state = State::LineComment;
        } else if char == '*' {
            self.state = State::BlockComment;
        } else {
            panic!("Unexpected comment start: {char}");
        }
    }

    fn handle_line_comment(&mut self, char: char) {
        if char == '\n' {
            self.state = State::None;
            self.tokens
                .push(Token::LineComment(self.buffer.trim().to_string()));
            self.buffer = String::new();
        } else {
            self.buffer.push(char);
        }
    }
}

enum State {
    None,
    Ident,
    CommentStart,
    LineComment,
    BlockComment,
}
