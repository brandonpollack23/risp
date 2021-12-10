use std::fmt::{Debug, Formatter};
use std::ptr::addr_of;

use crate::error::{RispError, RispResult};
use crate::tokenizer::RispToken;

pub fn parse(tokens: &[RispToken]) -> RispResult<RispExp> {
    parse_internal(tokens).map(|exp| exp.0)
}

fn parse_internal<'a>(tokens: &[RispToken]) -> RispResult<(RispExp, &[RispToken])> {
    let (token, rest) = tokens.split_first().unwrap();
    match &token {
        RispToken::LParen => read_seq(rest),
        RispToken::RParen => Err(RispError::UnexpectedToken(RispToken::RParen)),
        _ => Ok((parse_atom(token)?, rest)),
    }
}

fn parse_atom(token: &RispToken) -> RispResult<RispExp> {
    match token {
        RispToken::Integer(x) => Ok(RispExp::Integer(*x)),
        RispToken::Float(x) => Ok(RispExp::Float(*x)),
        RispToken::Symbol(str) => parse_symbol(str),
        other => Err(RispError::UnexpectedToken(other.clone())),
    }
}

fn parse_symbol(str: &str) -> Result<RispExp, RispError> {
    match str {
        builtin if RispFunction::is_builtin(str) => Ok(RispExp::Func(builtin.into())),
        _ => Ok(RispExp::Symbol(str.to_string())),
    }
}

fn read_seq(tokens: &[RispToken]) -> RispResult<(RispExp, &[RispToken])> {
    let mut res: Vec<RispExp> = vec![];
    let mut xs = tokens;
    loop {
        let (next_token, rest) = xs.split_first().ok_or(RispError::UnterminatedList)?;
        if next_token == &RispToken::RParen {
            return Ok((RispExp::List(res), rest));
        }

        let (exp, new_xs) = parse_internal(&xs)?;
        res.push(exp);
        xs = new_xs;
    }
}

#[derive(Clone, Debug)]
pub enum RispExp {
    Symbol(String),
    Integer(i32),
    Float(f64),
    List(Vec<RispExp>),

    Func(RispFunction),
}

#[derive(Clone)]
pub enum RispFunction {
    Function(fn(&[RispExp]) -> RispResult<RispExp>),
    Plus,
    Minus,
    Multiply,
    Divide,
    Xor,
    Or,
    And,
    // TODO if
    // TODO pow
}

impl RispFunction {
    fn is_builtin(str: &str) -> bool {
        match str {
            "+" | "-" | "*" | "/" | "xor" | "or" | "and" => true,
            _ => false,
        }
    }
}

impl From<&str> for RispFunction {
    fn from(str: &str) -> Self {
        match str {
            "+" => RispFunction::Plus,
            "-" => RispFunction::Minus,
            "*" => RispFunction::Multiply,
            "/" => RispFunction::Divide,
            "xor" => RispFunction::Xor,
            "or" => RispFunction::Or,
            "and" => RispFunction::And,
            _ => panic!("This is not a valid built in!"),
        }
    }
}

impl Debug for RispFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            RispFunction::Function(f) => format!("#f@{:?}", addr_of!(f)),
            RispFunction::Plus => "+".to_string(),
            RispFunction::Minus => "-".to_string(),
            RispFunction::Multiply => "*".to_string(),
            RispFunction::Divide => "/".to_string(),
            RispFunction::Xor => "xor".to_string(),
            RispFunction::Or => "or".to_string(),
            RispFunction::And => "and".to_string(),
        };

        f.write_str(&out)
    }
}
