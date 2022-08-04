use std::io::Read;

use anyhow::Result;
use parsing::parse;
use tokenizing::tokenize;

mod errors;
mod parsing;
#[cfg(test)]
mod test;
mod tokenizing;

pub use parsing::Definition;
pub use parsing::PostfixModifier;

pub fn read_definitions(reader: impl Read) -> Result<Vec<Definition>> {
    let tokens = tokenize(reader)?;
    parse(tokens)
}

#[derive(PartialEq, Debug)]
enum Token {
    LineComment(String),
    Identifier(String),
    LeftBrace,
    RightBrace,
    LeftParenthesis,
    RightParenthesis,
    Operator(String),
}
