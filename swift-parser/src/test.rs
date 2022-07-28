use std::fs::File;

use anyhow::Result;

use crate::{
    parsing::{parse, Definition},
    tokenizing::tokenize,
    Token,
};

#[test]
fn tokenize_simple() -> Result<()> {
    let input = File::open("../samples/Simple.swift")?;

    let tokens = tokenize(input)?;

    assert_eq!(
        vec![
            Token::Identifier("protocol".to_owned()),
            Token::Identifier("Simple".to_owned()),
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
        Token::Identifier("Simple".to_owned()),
        Token::LeftBrace,
        Token::LineComment("GET /get".to_owned()),
        Token::Identifier("func".to_owned()),
        Token::Identifier("get".to_owned()),
        Token::LeftParenthesis,
        Token::RightParenthesis,
        Token::RightBrace,
    ];

    let definitions = parse(tokens)?;
    assert_eq!(1, definitions.len());
    if let Definition::Protocol(name, definitions) = &definitions[0] {
        assert_eq!("Simple", name);
        assert_eq!(2, definitions.len());
        if let Definition::Comment(comment) = &definitions[0] {
            assert_eq!("GET /get", comment);
        } else {
            panic!("Expected comment");
        }
        if let Definition::Function(name) = &definitions[1] {
            assert_eq!("get", name);
        } else {
            panic!("Expected function");
        }
    } else {
        panic!("Invalid parsed structure")
    }
    println!("Definitions: {definitions:?}");

    Ok(())
}
