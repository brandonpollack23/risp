use crate::tokenizer::RispToken;
use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;

pub type RispResult<T> = Result<T, RispError>;

#[derive(Error, Debug)]
pub enum RispError {
    #[error("The input {0} is not recognized as any valid token")]
    UnrecognizedToken(String),
    #[error("The token {0:?} was not expected here")]
    UnexpectedToken(RispToken),

    #[error("Attempting to add non float/int with addition builtin")]
    AdditionError,

    #[error("Error parsing integer: {source}")]
    ParseIntError {
        #[from]
        source: ParseIntError,
    },
    #[error("Error parsing integer: {source}")]
    ParseFloatError {
        #[from]
        source: ParseFloatError,
    },

    #[error("The previous LParen was unterminated")]
    UnterminatedList,

    #[error("The form must begin with an executable function to be evaluated")]
    FirstListElementIsNotExecutable,
}
