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
            self.state = match self.state {
                State::None => self.handle_any_identifier(token)?,
                State::ProtocolStart => self.handle_protocol_start(token)?,
                State::ProtocolWithName(ref name) => {
                    self.handle_protocol_content(token, name.to_string())?
                }
                State::FunctionStart => self.handle_function_start(token)?,
                State::FunctionWithName(ref name) => {
                    self.handle_parameter_name(token, name.to_string())?
                }
                State::ParameterList => self.handle_parameter_list(token)?,
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
            "async" | "throws" => {
                self.update_function_modifier(identifier);
                Ok(State::None)
            }
            value => panic!("Unknown identifier: {value}"),
        }
    }

    fn update_function_modifier(&mut self, modifier: &str) {
        let (name, mut modifiers) = match self.definitions.pop() {
            Some(Definition::Function { name, modifiers }) => (name, modifiers),
            _ => panic!("expected modifier after function"),
        };

        let modifier_type = match modifier {
            "async" => PostfixModifier::Async,
            "throws" => PostfixModifier::Throws,
            value => panic!("Unknown postfix modifier: {value}"),
        };

        if matches!(modifier_type, PostfixModifier::Async)
            && modifiers.contains(&PostfixModifier::Throws)
        {
            panic!("async must come before throws")
        }

        if modifiers.contains(&modifier_type) {
            panic!("Repeat {modifier} token");
        }

        modifiers.push(modifier_type);
        self.definitions
            .push(Definition::Function { name, modifiers });
    }

    fn handle_block_close(&mut self) -> Result<()> {
        let (previous_state, mut previous_definitions) = self
            .previous_states
            .pop()
            .ok_or(ParseError::InvalidScopeTarget)?;
        if let State::ProtocolWithName(name) = previous_state {
            let mut definitions = vec![];
            std::mem::swap(&mut self.definitions, &mut definitions);
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
            let mut current_definitions = vec![];
            std::mem::swap(&mut self.definitions, &mut current_definitions);
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
            let mut current_definitions = vec![];
            std::mem::swap(&mut self.definitions, &mut current_definitions);
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
                let (previous_state, mut previous_definitions) = self
                    .previous_states
                    .pop()
                    .ok_or(ParseError::InvalidParameterTarget)?;
                if let State::FunctionWithName(name) = previous_state {
                    if !self.definitions.is_empty() {
                        panic!("The definitions must be empty");
                    }
                    previous_definitions.push(Definition::Function {
                        name,
                        modifiers: vec![],
                    });
                    self.definitions = previous_definitions;
                } else {
                    panic!("Unexpected type: {previous_state:?}");
                }
                Ok(State::None)
            }
            value => panic!("Unexpected token: {value:?}"),
        }
    }
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
}
