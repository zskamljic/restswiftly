use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeneratingError {
    #[error("General error: {0}")]
    GeneralError(String),
}
