use std::fs::File;

use anyhow::Result;

use crate::{parsing::parse, tokenizing::tokenize, Token};

#[test]
fn tokenize_simple() -> Result<()> {
    let input = File::open("../samples/Simple.swift")?;

    let tokens = tokenize(input)?;

    assert_eq!(
        vec![
            Token::Identifier("protocol".to_owned()),
            Token::Identifier("SomeService".to_owned()),
            Token::LeftBrace,
            Token::LineComment("GET /get".to_owned()),
            Token::Identifier("func".to_owned()),
            Token::Identifier("get".to_owned()),
            Token::LeftParenthesis,
            Token::RightParenthesis,
            Token::RightBrace,
        ],
        tokens
    );

    Ok(())
}

#[test]
fn parse_simple() -> Result<()> {
    let tokens = vec![
        Token::Identifier("protocol".to_owned()),
        Token::Identifier("SomeService".to_owned()),
        Token::LeftBrace,
        Token::LineComment("GET /get".to_owned()),
        Token::Identifier("func".to_owned()),
        Token::Identifier("get".to_owned()),
        Token::LeftParenthesis,
        Token::RightParenthesis,
        Token::RightBrace,
    ];

    let definitions = parse(tokens)?;
    println!("Definitions: {definitions:?}");

    Ok(())
}
