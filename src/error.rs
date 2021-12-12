use crate::parser::{RispExp, RispFunction};
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

    #[error("The expression {0:?} was not expected here: {1}")]
    UnexpectedExpr(RispExp, String),
    #[error("The symbol {0} is unrecognized, did you remember to define it?")]
    UnexpectedSymbol(String),

    #[error("Primitive type mismatch: {0}")]
    TypeError(&'static str),

    #[error("Error parsing integer: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("Error parsing float: {0}")]
    ParseFloatError(#[from] ParseFloatError),
    #[error("Error parsing bool: {0}")]
    ParseBoolError(#[from] ParseBoolError),

    #[error("Arity mismatch caused by: {0:?}")]
    ArityMismatch(RispFunction),

    #[error("The previous LParen was unterminated")]
    UnterminatedList,

    #[error("{0:?} is not an evaluable function")]
    FirstFormMustBeFunction(RispExp),
}

pub const ILLEGAL_TYPE_FOR_ARITHMETIC_OP: &str =
    "Attempting to do arithmetic operation on non float/int with builtin";
