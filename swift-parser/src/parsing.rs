use std::{iter::Peekable, mem, vec::IntoIter};

use crate::{errors::ParsingError, Token};

use anyhow::Result;

pub(crate) fn parse(token: Vec<Token>) -> Result<Vec<Definition>> {
    Parser::new().parse(token)
}

type TokenIter = Peekable<IntoIter<Token>>;

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
        let mut tokens = tokens.into_iter().peekable();

        while let Some(token) = tokens.next() {
            match token {
                Token::Identifier(value) => self.handle_identifier(&value, &mut tokens)?,
                Token::LineComment(comment) => self.definitions.push(Definition::Comment(comment)),
                Token::RightBrace => self.pop_state()?,
                value => todo!("Not handled: {value:?}"),
            }
        }

        if !self.previous_states.is_empty() {
            return Err(ParsingError::GeneralError("Did not handle all states".into()).into());
        }
        if !matches!(self.state, State::None) {
            return Err(ParsingError::GeneralError(format!(
                "Did not finalize state: {:?}",
                self.state
            ))
            .into());
        }

        Ok(self.definitions)
    }

    fn handle_identifier(&mut self, identifier: &str, tokens: &mut TokenIter) -> Result<()> {
        match identifier {
            "protocol" => self.handle_protocol(tokens)?,
            "func" => {
                let function = self.handle_function(tokens)?;
                self.definitions.push(function);
            }
            value => todo!("Identifiers: {value:?}"),
        }
        Ok(())
    }

    fn handle_protocol(&mut self, tokens: &mut TokenIter) -> Result<()> {
        let name = match tokens.next() {
            Some(Token::Identifier(name)) => name,
            Some(token) => return Err(ParsingError::UnexpectedToken(token).into()),
            None => return Err(ParsingError::EndOfFile.into()),
        };

        match tokens.next() {
            Some(Token::LeftBrace) => {
                self.previous_states
                    .push((mem::take(&mut self.state), mem::take(&mut self.definitions)));
                self.state = State::ProtocolWithName(name);
            }
            Some(token) => return Err(ParsingError::UnexpectedToken(token).into()),
            None => return Err(ParsingError::EndOfFile.into()),
        }
        Ok(())
    }

    fn handle_function(&mut self, tokens: &mut TokenIter) -> Result<Definition> {
        let name = match tokens.next() {
            Some(Token::Identifier(name)) => name,
            Some(token) => return Err(ParsingError::UnexpectedToken(token).into()),
            None => return Err(ParsingError::EndOfFile.into()),
        };

        match tokens.next() {
            Some(Token::LeftParenthesis) => (),
            Some(token) => return Err(ParsingError::UnexpectedToken(token).into()),
            None => return Err(ParsingError::EndOfFile.into()),
        };

        let parameters = read_parameters(tokens)?;

        match tokens.next() {
            Some(Token::RightParenthesis) => (),
            Some(token) => return Err(ParsingError::UnexpectedToken(token).into()),
            None => return Err(ParsingError::EndOfFile.into()),
        };

        let modifiers = Self::parse_modifiers(tokens)?;

        let return_type: Option<String>;
        match tokens.peek() {
            Some(Token::Operator(value)) => {
                if value != "->" {
                    return Err(ParsingError::UnexpectedIdentifier("->".into()).into());
                }
                tokens.next();
                if let Some(Token::Identifier(name)) = tokens.next() {
                    return_type = Some(name)
                } else {
                    return Err(ParsingError::GeneralError("Missing return type".into()).into());
                }
            }
            _ => return_type = None,
        };

        Ok(Definition::Function {
            name,
            parameters,
            modifiers,
            return_type,
        })
    }

    fn parse_modifiers(tokens: &mut TokenIter) -> Result<Vec<PostfixModifier>> {
        let mut modifiers = vec![];

        loop {
            match tokens.peek() {
                Some(Token::Identifier(modifier)) => {
                    if modifier == "async" {
                        if modifiers.contains(&PostfixModifier::Throws) {
                            return Err(ParsingError::GeneralError(
                                "async cannot come after throws".into(),
                            )
                            .into());
                        }
                        if modifiers.contains(&PostfixModifier::Async) {
                            return Err(ParsingError::GeneralError(
                                "async cannot come after async".into(),
                            )
                            .into());
                        }
                        modifiers.push(PostfixModifier::Async);
                        tokens.next();
                    } else if modifier == "throws" {
                        if modifiers.contains(&PostfixModifier::Throws) {
                            return Err(ParsingError::GeneralError(
                                "throws cannot come after throws".into(),
                            )
                            .into());
                        }
                        modifiers.push(PostfixModifier::Throws);
                        tokens.next();
                    }
                }
                _ => break,
            }
        }
        Ok(modifiers)
    }

    fn pop_state(&mut self) -> Result<()> {
        let definitions = mem::take(&mut self.definitions);
        let (mut state, previous_definitions) = match self.previous_states.pop() {
            Some((state, definitions)) => (state, definitions),
            None => return Err(ParsingError::GeneralError("Illegal internal state".into()).into()),
        };
        mem::swap(&mut self.state, &mut state);
        self.definitions = previous_definitions;
        match state {
            State::ProtocolWithName(name) => {
                self.definitions
                    .push(Definition::Protocol(name, definitions));
                Ok(())
            }
            state => Err(ParsingError::UnexpectedState(format!("{state:?}")).into()),
        }
    }
}

fn read_parameters(tokens: &mut TokenIter) -> Result<Vec<Parameter>> {
    let mut parameters = vec![];

    loop {
        let peeked = tokens.peek();
        match peeked {
            Some(Token::RightParenthesis) => break,
            Some(Token::Identifier(_)) => (),
            Some(_) => return Err(ParsingError::UnexpectedToken(tokens.next().unwrap()).into()),
            None => return Err(ParsingError::EndOfFile.into()),
        }

        let label = match tokens.next() {
            Some(Token::Identifier(name)) => name,
            Some(token) => return Err(ParsingError::UnexpectedToken(token).into()),
            None => return Err(ParsingError::EndOfFile.into()),
        };

        let label_and_name: (Option<String>, String);
        let peeked = tokens.peek();
        match peeked {
            Some(Token::Identifier(value)) => {
                label_and_name = (Some(label), value.into());
                tokens.next();
                match tokens.next() {
                    Some(Token::Colon) => (),
                    Some(token) => return Err(ParsingError::UnexpectedToken(token).into()),
                    None => return Err(ParsingError::EndOfFile.into()),
                }
            }
            Some(Token::Colon) => {
                label_and_name = (None, label);
                tokens.next();
            }
            Some(_) => return Err(ParsingError::UnexpectedToken(tokens.next().unwrap()).into()),
            None => return Err(ParsingError::EndOfFile.into()),
        }

        let type_name = match tokens.next() {
            Some(Token::Identifier(name)) => name,
            Some(token) => return Err(ParsingError::UnexpectedToken(token).into()),
            None => return Err(ParsingError::EndOfFile.into()),
        };
        println!("Read type: {type_name}");
        parameters.push(Parameter {
            label: label_and_name.0,
            name: label_and_name.1,
            parameter_type: type_name,
        })
    }

    Ok(parameters)
}

#[derive(Debug)]
pub struct Parameter {
    pub label: Option<String>,
    pub name: String,
    pub parameter_type: String,
}

#[derive(Debug)]
pub enum Definition {
    Comment(String),
    Function {
        name: String,
        parameters: Vec<Parameter>,
        modifiers: Vec<PostfixModifier>,
        return_type: Option<String>,
    },
    Parameter(Option<String>, String, String),
    Protocol(String, Vec<Definition>),
}

#[derive(PartialEq, Debug)]
pub enum PostfixModifier {
    Async,
    Throws,
}

#[derive(Debug, Default)]
enum State {
    #[default]
    None,
    ProtocolWithName(String),
}
