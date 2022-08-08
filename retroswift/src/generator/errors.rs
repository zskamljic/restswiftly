use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeneratingError {
    #[error("General error: {0}")]
    GeneralError(String),
    #[error("Missing parameter: {0}")]
    MissingParameter(String),
    #[error("There were unused parameters: {0:?}")]
    UnusedParameters(Vec<String>),
}
