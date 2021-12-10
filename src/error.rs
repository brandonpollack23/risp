use crate::tokenizer::RispToken;
use std::num::{ParseFloatError, ParseIntError};
use std::str::ParseBoolError;
use thiserror::Error;

pub type RispResult<T> = Result<T, RispError>;

#[derive(Error, Debug, PartialEq)]
pub enum RispError {
    #[error("The input {0} is not recognized as any valid token")]
    UnrecognizedToken(String),
    #[error("The token {0:?} was not expected here")]
    UnexpectedToken(RispToken),

    #[error("Attempting to add non float/int with addition builtin")]
    AdditionError,

    #[error("Error parsing integer: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("Error parsing float: {0}")]
    ParseFloatError(#[from] ParseFloatError),
    #[error("Error parsing bool: {0}")]
    ParseBoolError(#[from] ParseBoolError),

    #[error("The previous LParen was unterminated")]
    UnterminatedList,

    #[error("The form must begin with an executable function to be evaluated")]
    FirstListElementIsNotExecutable,
}
