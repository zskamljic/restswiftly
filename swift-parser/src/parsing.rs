use crate::Token;

use anyhow::Result;
use thiserror::Error;

pub(crate) fn parse(tokens: Vec<Token>) -> Result<Vec<Definition>> {
    let mut state = State::None;

    let mut definitions = vec![];
    let mut previous_states: Vec<(State, Vec<Definition>)> = vec![];
    let mut iterator = tokens.into_iter();
    while let Some(token) = iterator.next() {
        match state {
            State::None => match token {
                Token::Identifier(value) => match value.as_ref() {
                    "protocol" => state = State::ProtocolStart,
                    "func" => state = State::FunctionStart,
                    value => panic!("Unknown identifier: {value}"),
                },
                Token::LineComment(value) => definitions.push(Definition::Comment(value)),
                Token::RightBrace => {
                    let (previous_state, mut previous_definitions) = previous_states
                        .pop()
                        .ok_or(ParseError::InvalidScopeTarget)?;
                    if let State::ProtocolWithName(name) = previous_state {
                        previous_definitions.push(Definition::Protocol(name, definitions));
                        definitions = previous_definitions;
                    } else {
                        panic!("Type not supported");
                    }
                    state = State::None;
                }
                value => panic!("Unexpected token: {value:?}"),
            },
            State::ProtocolStart => match token {
                Token::Identifier(value) => state = State::ProtocolWithName(value),
                value => panic!("Unexpected token: {value:?}"),
            },
            State::ProtocolWithName(name) => {
                if matches!(token, Token::LeftBrace) {
                    previous_states.push((State::ProtocolWithName(name), definitions));
                    definitions = vec![];
                    state = State::None;
                } else {
                    panic!("Unexpected token: {token:?}");
                }
            }
            State::FunctionStart => {
                if let Token::Identifier(value) = token {
                    state = State::FunctionWithName(value)
                } else {
                    panic!("Unexpected token: {token:?}");
                }
            }
            State::FunctionWithName(value) => {
                if matches!(token, Token::LeftParenthesis) {
                    previous_states.push((State::FunctionWithName(value), definitions));
                    definitions = vec![];
                    state = State::ParameterList;
                } else {
                    panic!("Unexpected token: {token:?}");
                }
            }
            State::ParameterList => match token {
                Token::RightParenthesis => {
                    let (previous_state, mut previous_definitions) = previous_states
                        .pop()
                        .ok_or(ParseError::InvalidParameterTarget)?;
                    if let State::FunctionWithName(name) = previous_state {
                        if !definitions.is_empty() {
                            panic!("The definitions must be empty");
                        }
                        previous_definitions.push(Definition::Function(name));
                        definitions = previous_definitions;
                    } else {
                        panic!("Unexpected type: {previous_state:?}");
                    }
                    state = State::None;
                }
                value => panic!("Unexpected token: {value:?}"),
            },
        }
    }

    if !previous_states.is_empty() {
        panic!("Did not pop all states");
    }
    if !matches!(state, State::None) {
        panic!("Did not finalize state: {state:?}");
    }

    Ok(definitions)
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("Invalid token for parameters")]
    InvalidParameterTarget,
    #[error("Invalid scope target")]
    InvalidScopeTarget,
}

#[derive(Debug)]
pub enum Definition {
    Comment(String),
    Function(String),
    Protocol(String, Vec<Definition>),
}

#[derive(Debug)]
enum State {
    None,
    ProtocolStart,
    ProtocolWithName(String),
    FunctionStart,
    FunctionWithName(String),
    ParameterList,
}
