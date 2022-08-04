use thiserror::Error;

use crate::Token;

#[derive(Error, Debug)]
pub(crate) enum ParsingError {
    #[error("Invalid token for parameters")]
    InvalidParameterTarget,
    #[error("Invalid scope target")]
    InvalidScopeTarget,
    #[error("Feature not supported: {0}")]
    FeatureNotSupported(String),
    #[error("Unexpected character: {0}")]
    UnexpectedCharacter(char),
    #[error("Unexpected token: {0:?}")]
    UnexpectedToken(Token),
    #[error("Unknown identifier: {0}")]
    UnexpectedIdentifier(String),
    #[error("Unexpected state: {0}")]
    UnexpectedState(String),
    #[error("Parsing did not complete correctly: {0}")]
    GeneralError(String),
}
