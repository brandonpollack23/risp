use crate::parser::{RispExp, RispFunction};
use crate::tokenizer::RispToken;
use std::num::{ParseFloatError, ParseIntError};
use std::str::ParseBoolError;
use thiserror::Error;

pub type RispResult<T> = Result<T, RispError>;

#[derive(Error, Debug, PartialEq)]
pub enum RispError {
    #[error("Generic error occured: {0}")]
    GenericError(String),

    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,

    #[error("Cannot use built in names for def")]
    InvalidName(String),

    #[error("The input {0} is not recognized as any valid token")]
    UnrecognizedToken(String),
    #[error("The token {0:?} was not expected here {1}")]
    UnexpectedToken(RispToken, String),

    #[error("The expression {0:?} was not expected here: {1}")]
    UnexpectedExpr(RispExp, String),
    #[error("The symbol {0} is unrecognized, did you remember to define it?")]
    UnexpectedSymbol(String),

    #[error("Def must be of the form (def symbol expr)")]
    MalformedDefExpression,

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

// Unexpected token
pub const TRAILING_TOKENS: &str =
    "Trailing tokens, did you type extra stuff outside parens? This is a lisp you know...";
pub const UNEXPECTED_CLOSING_PAREN: &str =
    "A closing paren was not expected here, are there too many?";
pub const EXPECTED_ARGS_LIST_FOR_FN: &str = "A fn must have an args list in it's first argument";
pub const EXPECTED_FN_DEF_FOR_FN: &str =
    "A fn must have a function definition in it's second argument";
