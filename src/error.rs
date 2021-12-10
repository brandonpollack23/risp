use crate::parser::RispToken;
use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;

pub type RispResult<T> = Result<T, RispError>;

#[derive(Error, Debug)]
pub enum RispError {
    #[error("The input {0} is not recognized as any valid token")]
    UnrecognizedToken(String),
    #[error("The token {0:?} was not expected here")]
    UnexpectedToken(RispToken),

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
    #[error("The error failed due to {0}")]
    Reason(String),
}
