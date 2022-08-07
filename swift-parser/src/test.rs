use std::fs::File;

use anyhow::Result;

use crate::{
    parsing::{parse, Definition, PostfixModifier},
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
            Token::Identifier("async".to_owned()),
            Token::Identifier("throws".to_owned()),
            Token::RightBrace,
        ],
        tokens
    );

    Ok(())
}

#[test]
fn tokenize_returning() -> Result<()> {
    let input = File::open("../samples/Return.swift")?;

    let tokens = tokenize(input)?;

    assert_eq!(
        vec![
            Token::Identifier("protocol".to_owned()),
            Token::Identifier("Return".to_owned()),
            Token::LeftBrace,
            Token::LineComment("GET /get".to_owned()),
            Token::Identifier("func".to_owned()),
            Token::Identifier("get".to_owned()),
            Token::LeftParenthesis,
            Token::RightParenthesis,
            Token::Identifier("async".to_owned()),
            Token::Identifier("throws".to_owned()),
            Token::Operator("->".to_owned()),
            Token::Identifier("Hello".to_owned()),
            Token::RightBrace,
        ],
        tokens,
    );

    Ok(())
}

#[test]
fn tokenize_params() -> Result<()> {
    let input = File::open("../samples/QueryParameter.swift")?;

    let tokens = tokenize(input)?;

    assert_eq!(
        vec![
            Token::Identifier("protocol".to_owned()),
            Token::Identifier("QueryParameter".to_owned()),
            Token::LeftBrace,
            Token::LineComment("GET /get?q=:query".to_owned()),
            Token::Identifier("func".to_owned()),
            Token::Identifier("get".to_owned()),
            Token::LeftParenthesis,
            Token::Identifier("query".to_owned()),
            Token::Colon,
            Token::Identifier("String".to_owned()),
            Token::RightParenthesis,
            Token::Identifier("async".to_owned()),
            Token::Identifier("throws".to_owned()),
            Token::LineComment("GET /get?q=:query&q2=something".to_owned()),
            Token::Identifier("func".to_owned()),
            Token::Identifier("get".to_owned()),
            Token::LeftParenthesis,
            Token::Identifier("for".to_owned()),
            Token::Identifier("query".to_owned()),
            Token::Colon,
            Token::Identifier("String".to_owned()),
            Token::RightParenthesis,
            Token::Identifier("async".to_owned()),
            Token::Identifier("throws".to_owned()),
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
        if let Definition::Function { name, .. } = &definitions[1] {
            assert_eq!("get", name);
        } else {
            panic!("Expected function");
        }
    } else {
        panic!("Invalid parsed structure")
    }

    Ok(())
}

#[test]
fn parse_async_throws() -> Result<()> {
    let tokens = vec![
        Token::Identifier("protocol".to_owned()),
        Token::Identifier("Simple".to_owned()),
        Token::LeftBrace,
        Token::LineComment("GET /get".to_owned()),
        Token::Identifier("func".to_owned()),
        Token::Identifier("get".to_owned()),
        Token::LeftParenthesis,
        Token::RightParenthesis,
        Token::Identifier("async".to_owned()),
        Token::Identifier("throws".to_owned()),
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
        if let Definition::Function {
            name,
            parameters,
            modifiers,
            return_type,
        } = &definitions[1]
        {
            assert_eq!("get", name);
            assert!(parameters.is_empty());
            assert!(modifiers.contains(&PostfixModifier::Async));
            assert!(modifiers.contains(&PostfixModifier::Throws));
            assert!(matches!(return_type, None));
        } else {
            panic!("Expected function");
        }
    } else {
        panic!("Invalid parsed structure")
    }

    Ok(())
}

#[test]
fn parse_returns() -> Result<()> {
    let tokens = vec![
        Token::Identifier("protocol".to_owned()),
        Token::Identifier("Return".to_owned()),
        Token::LeftBrace,
        Token::LineComment("GET /get".to_owned()),
        Token::Identifier("func".to_owned()),
        Token::Identifier("get".to_owned()),
        Token::LeftParenthesis,
        Token::RightParenthesis,
        Token::Identifier("async".to_owned()),
        Token::Identifier("throws".to_owned()),
        Token::Operator("->".to_owned()),
        Token::Identifier("Hello".to_owned()),
        Token::RightBrace,
    ];

    let definitions = parse(tokens)?;
    assert_eq!(1, definitions.len());
    if let Definition::Protocol(name, definitions) = &definitions[0] {
        assert_eq!("Return", name);
        assert_eq!(2, definitions.len());
        if let Definition::Comment(comment) = &definitions[0] {
            assert_eq!("GET /get", comment);
        } else {
            panic!("Expected comment");
        }
        if let Definition::Function {
            name,
            parameters,
            modifiers,
            return_type,
        } = &definitions[1]
        {
            assert_eq!("get", name);
            assert!(parameters.is_empty());
            assert!(modifiers.contains(&PostfixModifier::Async));
            assert!(modifiers.contains(&PostfixModifier::Throws));
            if let Some(value) = return_type {
                assert_eq!("Hello", value);
            } else {
                panic!("Wanted return type");
            }
        } else {
            panic!("Expected function");
        }
    } else {
        panic!("Invalid parsed structure")
    }

    Ok(())
}

#[test]
fn parse_parameter_list() -> Result<()> {
    let tokens = vec![
        Token::Identifier("protocol".to_owned()),
        Token::Identifier("Simple".to_owned()),
        Token::LeftBrace,
        Token::LineComment("GET /get".to_owned()),
        Token::Identifier("func".to_owned()),
        Token::Identifier("get".to_owned()),
        Token::LeftParenthesis,
        Token::Identifier("query".to_owned()),
        Token::Colon,
        Token::Identifier("String".to_owned()),
        Token::RightParenthesis,
        Token::RightBrace,
    ];

    let definitions = parse(tokens)?;
    assert_eq!(1, definitions.len());
    if let Definition::Protocol(_, definitions) = &definitions[0] {
        if let Definition::Function {
            name, parameters, ..
        } = &definitions[1]
        {
            assert_eq!("get", name);
            assert_eq!(1, parameters.len());
            let parameter = &parameters[0];
            assert!(matches!(parameter.label, None));
            assert_eq!("query".to_owned(), parameter.name);
            assert_eq!("String".to_owned(), parameter.parameter_type);
        } else {
            panic!("Expected function");
        }
    } else {
        panic!("Invalid parsed structure")
    }
    Ok(())
}

#[test]
#[should_panic]
fn parse_async_throws_invalid_order() {
    let tokens = vec![
        Token::Identifier("protocol".to_owned()),
        Token::Identifier("Simple".to_owned()),
        Token::LeftBrace,
        Token::LineComment("GET /get".to_owned()),
        Token::Identifier("func".to_owned()),
        Token::Identifier("get".to_owned()),
        Token::LeftParenthesis,
        Token::RightParenthesis,
        Token::Identifier("throws".to_owned()),
        Token::Identifier("async".to_owned()),
        Token::RightBrace,
    ];

    parse(tokens).unwrap();
}
