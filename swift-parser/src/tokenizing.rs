use std::io::{BufReader, Read};
use std::iter::Peekable;

use anyhow::Result;
use utf8_chars::{BufReadCharsExt, Chars};

use crate::errors::ParsingError;
use crate::Token;

pub(crate) fn tokenize(reader: impl Read) -> Result<Vec<Token>> {
    let mut tokens = vec![];
    let mut reader = BufReader::new(reader);
    let mut chars = reader.chars().peekable();

    while let Some(char) = chars.next() {
        match char? {
            c if c.is_alphabetic() => tokens.push(read_identifier(c, &mut chars)?),
            c if c == '{' => tokens.push(Token::LeftBrace),
            c if c == '}' => tokens.push(Token::RightBrace),
            c if c == '(' => tokens.push(Token::LeftParenthesis),
            c if c == ')' => tokens.push(Token::RightParenthesis),
            c if c == '/' => tokens.push(read_comment(&mut chars)?),
            c if c == ':' => tokens.push(Token::Colon),
            c if c == ',' => tokens.push(Token::Comma),
            c if c == '-' => tokens.push(read_operator(c, &mut chars)?),
            c if c.is_whitespace() => continue,
            value => return Err(ParsingError::UnexpectedCharacter(value).into()),
        }
    }

    Ok(tokens)
}

fn read_identifier(
    prefix: char,
    iterator: &mut Peekable<Chars<'_, BufReader<impl Read>>>,
) -> Result<Token> {
    let mut name = String::new();
    name.push(prefix);
    while let Some(value) = iterator.peek() {
        let value = unwrap_char(value)?;
        if value.is_alphanumeric() {
            name.push(value)
        } else {
            break;
        }
        iterator.next();
    }

    Ok(Token::Identifier(name))
}

fn read_comment(iterator: &mut Peekable<Chars<'_, BufReader<impl Read>>>) -> Result<Token> {
    let comment_type = iterator.next().ok_or(ParsingError::EndOfFile)?;
    match comment_type {
        Ok('/') => read_line_comment(iterator),
        Ok('*') => Err(ParsingError::FeatureNotSupported("block comment".into()).into()),
        Ok(value) => Err(ParsingError::UnexpectedCharacter(value).into()),
        Err(error) => Err(error.into()),
    }
}

fn read_line_comment(iterator: &mut Peekable<Chars<'_, BufReader<impl Read>>>) -> Result<Token> {
    let mut comment = String::new();
    for char in iterator.by_ref() {
        let char = char?;
        if char == '\n' {
            break;
        }

        comment.push(char);
    }
    Ok(Token::LineComment(comment.trim().into()))
}

fn read_operator(
    prefix: char,
    iterator: &mut Peekable<Chars<'_, BufReader<impl Read>>>,
) -> Result<Token> {
    let mut operator = String::new();
    operator.push(prefix);
    let next = iterator.next().ok_or(ParsingError::EndOfFile)?;
    match next {
        Ok('>') => {
            operator.push('>');
            Ok(Token::Operator(operator))
        }
        Ok(value) => Err(ParsingError::UnexpectedCharacter(value).into()),
        Err(error) => Err(error.into()),
    }
}

fn unwrap_char(value: &Result<char, std::io::Error>) -> Result<char> {
    match value {
        Ok(value) => Ok(value.to_owned()),
        Err(error) => Err(ParsingError::GeneralError(format!("{error}")).into()),
    }
}
