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
        RispToken::Bool(b) => Ok(RispExp::Bool(*b)),
        RispToken::Integer(i) => Ok(RispExp::Integer(*i)),
        RispToken::Float(f) => Ok(RispExp::Float(*f)),
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

#[derive(Clone, Debug, PartialEq)]
pub enum RispExp {
    Symbol(String),
    Bool(bool),
    Integer(i32),
    Float(f64),
    List(Vec<RispExp>),

    Func(RispFunction),
}

#[derive(Clone)]
pub enum RispFunction {
    Function(fn(&[RispExp]) -> RispResult<RispExp>),
    Builtin(RispBuiltinFunction),
}

#[derive(Clone, PartialEq)]
pub enum RispBuiltinFunction {
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

impl PartialEq for RispFunction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RispFunction::Function(_), RispFunction::Function(_)) => true,
            (RispFunction::Builtin(s), RispFunction::Builtin(o)) => s == o,
            _ => false,
        }
    }
}

impl From<&str> for RispFunction {
    fn from(str: &str) -> Self {
        match str {
            "+" => RispFunction::Builtin(RispBuiltinFunction::Plus),
            "-" => RispFunction::Builtin(RispBuiltinFunction::Minus),
            "*" => RispFunction::Builtin(RispBuiltinFunction::Multiply),
            "/" => RispFunction::Builtin(RispBuiltinFunction::Divide),
            "xor" => RispFunction::Builtin(RispBuiltinFunction::Xor),
            "or" => RispFunction::Builtin(RispBuiltinFunction::Or),
            "and" => RispFunction::Builtin(RispBuiltinFunction::And),
            _ => panic!("This is not a valid built in!"),
        }
    }
}

impl Debug for RispFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            RispFunction::Function(f) => format!("#f@{:?}", addr_of!(f)),
            RispFunction::Builtin(RispBuiltinFunction::Plus) => "+".to_string(),
            RispFunction::Builtin(RispBuiltinFunction::Minus) => "-".to_string(),
            RispFunction::Builtin(RispBuiltinFunction::Multiply) => "*".to_string(),
            RispFunction::Builtin(RispBuiltinFunction::Divide) => "/".to_string(),
            RispFunction::Builtin(RispBuiltinFunction::Xor) => "xor".to_string(),
            RispFunction::Builtin(RispBuiltinFunction::Or) => "or".to_string(),
            RispFunction::Builtin(RispBuiltinFunction::And) => "and".to_string(),
        };

        f.write_str(&out)
    }
}
#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq, assert_ne};

    use crate::parser::{parse, RispFunction};
    use crate::parser::{RispBuiltinFunction, RispExp};
    use crate::tokenizer::RispToken;

    #[test]
    fn empty_list() {
        assert_eq!(
            parse(&[RispToken::LParen, RispToken::RParen]).unwrap(),
            RispExp::List(vec![])
        );
    }

    #[test]
    fn builtins() {
        assert_eq!(
            parse(&[
                RispToken::LParen,
                RispToken::Symbol("+".to_string()),
                RispToken::Integer(1),
                RispToken::Integer(2),
                RispToken::RParen
            ])
            .unwrap(),
            RispExp::List(vec![
                RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Plus)),
                RispExp::Integer(1),
                RispExp::Integer(2)
            ])
        );
        assert_eq!(
            parse(&[
                RispToken::LParen,
                RispToken::Symbol("-".to_string()),
                RispToken::Integer(1),
                RispToken::Integer(2),
                RispToken::RParen
            ])
            .unwrap(),
            RispExp::List(vec![
                RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Minus)),
                RispExp::Integer(1),
                RispExp::Integer(2)
            ])
        );
        assert_eq!(
            parse(&[
                RispToken::LParen,
                RispToken::Symbol("*".to_string()),
                RispToken::Integer(1),
                RispToken::Integer(2),
                RispToken::RParen
            ])
            .unwrap(),
            RispExp::List(vec![
                RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Multiply)),
                RispExp::Integer(1),
                RispExp::Integer(2)
            ])
        );
        assert_eq!(
            parse(&[
                RispToken::LParen,
                RispToken::Symbol("/".to_string()),
                RispToken::Integer(1),
                RispToken::Integer(2),
                RispToken::RParen
            ])
            .unwrap(),
            RispExp::List(vec![
                RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Divide)),
                RispExp::Integer(1),
                RispExp::Integer(2)
            ])
        );
        assert_eq!(
            parse(&[
                RispToken::LParen,
                RispToken::Symbol("xor".to_string()),
                RispToken::Integer(1),
                RispToken::Integer(2),
                RispToken::RParen
            ])
            .unwrap(),
            RispExp::List(vec![
                RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Xor)),
                RispExp::Integer(1),
                RispExp::Integer(2)
            ])
        );
        assert_eq!(
            parse(&[
                RispToken::LParen,
                RispToken::Symbol("or".to_string()),
                RispToken::Integer(1),
                RispToken::Integer(2),
                RispToken::RParen
            ])
            .unwrap(),
            RispExp::List(vec![
                RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Or)),
                RispExp::Integer(1),
                RispExp::Integer(2)
            ])
        );
        assert_eq!(
            parse(&[
                RispToken::LParen,
                RispToken::Symbol("and".to_string()),
                RispToken::Integer(1),
                RispToken::Integer(2),
                RispToken::RParen
            ])
            .unwrap(),
            RispExp::List(vec![
                RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::And)),
                RispExp::Integer(1),
                RispExp::Integer(2)
            ])
        );
    }

    #[test]
    fn builtin_ne() {
        assert_ne!(
            parse(&[
                RispToken::LParen,
                RispToken::Symbol("and".to_string()),
                RispToken::Integer(1),
                RispToken::Integer(2),
                RispToken::RParen
            ])
            .unwrap(),
            RispExp::List(vec![
                RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Or)),
                RispExp::Integer(1),
                RispExp::Integer(2)
            ])
        );
    }

    // TODO non lists (ints, floats, bools, symbols etc)
}
