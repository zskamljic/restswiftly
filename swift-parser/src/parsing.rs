use std::mem;

use crate::Token;

use anyhow::Result;
use thiserror::Error;

pub(crate) fn parse(token: Vec<Token>) -> Result<Vec<Definition>> {
    Parser::new().parse(token)
}

struct Parser {
    state: State,
    definitions: Vec<Definition>,
    previous_states: Vec<(State, Vec<Definition>)>,
}

impl Parser {
    fn new() -> Self {
        Parser {
            state: State::None,
            definitions: vec![],
            previous_states: vec![],
        }
    }

    fn parse(mut self, tokens: Vec<Token>) -> Result<Vec<Definition>> {
        for token in tokens.into_iter() {
            let state = mem::take(&mut self.state);
            self.state = match state {
                State::None => self.handle_any_identifier(token)?,
                State::ProtocolStart => self.handle_protocol_start(token)?,
                State::ProtocolWithName(name) => self.handle_protocol_content(token, name)?,
                State::FunctionStart => self.handle_function_start(token)?,
                State::FunctionWithName(name) => self.handle_parameter_name(token, name)?,
                State::ParameterList => self.handle_parameter_list(token)?,
                State::FunctionModifiers(name, modifiers) => {
                    self.handle_function_modifiers(token, name, modifiers)?
                }
                State::FunctionWithReturn(name, modifiers) => {
                    self.handle_function_return(token, name, modifiers)?
                }
            }
        }

        if !self.previous_states.is_empty() {
            panic!("Did not pop all states");
        }
        if !matches!(self.state, State::None) {
            panic!("Did not finalize state: {:?}", self.state);
        }

        Ok(self.definitions)
    }

    fn handle_any_identifier(&mut self, token: Token) -> Result<State> {
        match token {
            Token::Identifier(identifier) => self.handle_identifier(&identifier),
            Token::LineComment(value) => {
                self.definitions.push(Definition::Comment(value));
                Ok(State::None)
            }
            Token::RightBrace => {
                self.handle_block_close()?;
                Ok(State::None)
            }
            value => panic!("Unexpected token: {value:?}"),
        }
    }

    fn handle_identifier(&mut self, identifier: &str) -> Result<State> {
        match identifier {
            "protocol" => Ok(State::ProtocolStart),
            "func" => Ok(State::FunctionStart),
            value => panic!("Unknown identifier: {value}"),
        }
    }

    fn handle_block_close(&mut self) -> Result<()> {
        let (previous_state, mut previous_definitions) = self
            .previous_states
            .pop()
            .ok_or(ParseError::InvalidScopeTarget)?;
        if let State::ProtocolWithName(name) = previous_state {
            let definitions = mem::take(&mut self.definitions);
            previous_definitions.push(Definition::Protocol(name, definitions));
            self.definitions = previous_definitions;
        } else {
            panic!("Type not supported");
        }
        Ok(())
    }

    fn handle_protocol_start(&mut self, token: Token) -> Result<State> {
        match token {
            Token::Identifier(value) => Ok(State::ProtocolWithName(value)),
            value => panic!("Unexpected token: {value:?}"),
        }
    }

    fn handle_protocol_content(&mut self, token: Token, name: String) -> Result<State> {
        if matches!(token, Token::LeftBrace) {
            let current_definitions = mem::take(&mut self.definitions);
            self.previous_states
                .push((State::ProtocolWithName(name), current_definitions));
            Ok(State::None)
        } else {
            panic!("Unexpected token: {token:?}");
        }
    }

    fn handle_function_start(&mut self, token: Token) -> Result<State> {
        if let Token::Identifier(value) = token {
            Ok(State::FunctionWithName(value))
        } else {
            panic!("Unexpected token: {token:?}");
        }
    }

    fn handle_parameter_name(&mut self, token: Token, name: String) -> Result<State> {
        if matches!(token, Token::LeftParenthesis) {
            let current_definitions = mem::take(&mut self.definitions);
            self.previous_states
                .push((State::FunctionWithName(name), current_definitions));
            Ok(State::ParameterList)
        } else {
            panic!("Unexpected token: {token:?}");
        }
    }

    fn handle_parameter_list(&mut self, token: Token) -> Result<State> {
        match token {
            Token::RightParenthesis => {
                let (previous_state, previous_definitions) = self
                    .previous_states
                    .pop()
                    .ok_or(ParseError::InvalidParameterTarget)?;
                if let State::FunctionWithName(name) = previous_state {
                    if !self.definitions.is_empty() {
                        panic!("The definitions must be empty");
                    }
                    self.definitions = previous_definitions;
                    Ok(State::FunctionModifiers(name, vec![]))
                } else {
                    panic!("Unexpected type: {previous_state:?}");
                }
            }
            value => panic!("Unexpected token: {value:?}"),
        }
    }

    fn handle_function_modifiers(
        &mut self,
        token: Token,
        name: String,
        modifiers: Vec<PostfixModifier>,
    ) -> Result<State> {
        match token {
            Token::Identifier(ref value) => match value.as_str() {
                "async" | "throws" => Ok(State::FunctionModifiers(
                    name,
                    add_modifier(modifiers, &value)?,
                )),
                value => panic!("Unexpected identifier: {value}"),
            },
            Token::RightBrace => {
                self.definitions.push(Definition::Function {
                    name,
                    modifiers,
                    return_type: None,
                });
                self.handle_block_close()?;
                Ok(State::None)
            }
            Token::LineComment(comment) => {
                self.definitions.push(Definition::Function {
                    name,
                    modifiers,
                    return_type: None,
                });
                self.definitions.push(Definition::Comment(comment));
                Ok(State::None)
            }
            Token::Operator(value) => {
                if value != "->" {
                    panic!("Unexpected operator: {value}");
                }
                Ok(State::FunctionWithReturn(name, modifiers))
            }
            value => panic!("Unexpected token scanning for modifiers: {value:?}"),
        }
    }

    fn handle_function_return(
        &mut self,
        token: Token,
        name: String,
        modifiers: Vec<PostfixModifier>,
    ) -> Result<State> {
        let return_type = match token {
            Token::Identifier(value) => value,
            value => panic!("Illegal return type: {value:?}"),
        };

        self.definitions.push(Definition::Function {
            name,
            modifiers,
            return_type: Some(return_type),
        });
        Ok(State::None)
    }
}

fn add_modifier(
    mut modifiers: Vec<PostfixModifier>,
    modifier: &str,
) -> Result<Vec<PostfixModifier>> {
    let modifier_type = match modifier {
        "async" => PostfixModifier::Async,
        "throws" => PostfixModifier::Throws,
        value => panic!("Invalid postfix modifier: {value}"),
    };
    if modifiers.contains(&modifier_type) {
        panic!("Modifier already present: {modifier}");
    }
    if matches!(modifier_type, PostfixModifier::Async)
        && modifiers.contains(&PostfixModifier::Throws)
    {
        panic!("async must come before throws");
    }
    modifiers.push(modifier_type);

    Ok(modifiers)
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
    Function {
        name: String,
        modifiers: Vec<PostfixModifier>,
        return_type: Option<String>,
    },
    Protocol(String, Vec<Definition>),
}

#[derive(PartialEq, Debug)]
pub enum PostfixModifier {
    Async,
    Throws,
}

#[derive(Debug)]
enum State {
    None,
    ProtocolStart,
    ProtocolWithName(String),
    FunctionStart,
    FunctionWithName(String),
    ParameterList,
    FunctionModifiers(String, Vec<PostfixModifier>),
    FunctionWithReturn(String, Vec<PostfixModifier>),
}

impl Default for State {
    fn default() -> Self {
        State::None
    }
}
