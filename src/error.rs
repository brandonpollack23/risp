use crate::parser::RispToken;
use thiserror::Error;

pub type RispResult<T> = Result<T, RispError>;

#[derive(Error, Debug)]
pub enum RispError {
    #[error("The input {0} is not recognized as any valid token")]
    UnrecognizedToken(String),
    #[error("The token {0:?} was not expected here")]
    UnexpectedToken(RispToken),
    #[error("The previous semicolon was unterminated")]
    UnterminatedList,
    #[error("The error failed due to {0}")]
    Reason(String),
}
