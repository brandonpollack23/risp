use std::fmt::{Debug, Display, Formatter};
use std::ptr::addr_of;

use crate::error::{RispError, RispResult};
use crate::tokenizer::RispToken;

pub fn parse(tokens: &[RispToken]) -> RispResult<RispExp> {
    let result = parse_internal(tokens)?;
    if result.1.len() > 0 {
        return Err(RispError::UnexpectedToken(result.1.get(0).unwrap().clone()));
    }
    Ok(result.0)
}

fn parse_internal<'a>(tokens: &[RispToken]) -> RispResult<(RispExp, &[RispToken])> {
    if tokens.len() == 0 {
        return Ok((RispExp::Empty, &[]));
    }
    let (token, rest) = tokens.split_first().unwrap();
    match &token {
        RispToken::LParen => read_seq(rest),
        RispToken::RParen => Err(RispError::UnexpectedToken(RispToken::RParen)),
        _ => Ok((parse_atom(token)?, rest)),
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

fn parse_atom(token: &RispToken) -> RispResult<RispExp> {
    match token {
        RispToken::Char(c) => Ok(RispExp::Char(*c)),
        RispToken::Bool(b) => Ok(RispExp::Bool(*b)),
        RispToken::Integer(i) => Ok(RispExp::Integer(*i)),
        RispToken::Float(f) => Ok(RispExp::Float(*f)),
        RispToken::Symbol(str) => parse_symbol(str),
        RispToken::StringLiteral(str) => Ok(RispExp::String(str.to_owned())),
        RispToken::Nil => Ok(RispExp::Nil),
        t @ (RispToken::LParen | RispToken::RParen) => Err(RispError::UnexpectedToken(t.clone())),
    }
}

fn parse_symbol(str: &str) -> Result<RispExp, RispError> {
    match str {
        builtin if RispFunction::is_builtin(str) => Ok(RispExp::Func(builtin.into())),
        _ => Ok(RispExp::Symbol(str.to_owned())),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RispExp {
    Empty, // Used to signify empty input
    Nil,
    Symbol(String),
    Bool(bool),
    Integer(i32),
    Float(f64),
    Char(char),

    String(String),

    List(Vec<RispExp>),

    Func(RispFunction),
}

impl Display for RispExp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RispExp::Nil => "nil".to_owned(),
                RispExp::Symbol(s) => format!("'{}", s),
                RispExp::Bool(b) =>
                    if *b {
                        "#t".to_owned()
                    } else {
                        "#f".to_owned()
                    },
                RispExp::Integer(i) => i.to_string(),
                RispExp::Float(f) => f.to_string(),
                RispExp::Char(c) => c.to_string(),
                RispExp::String(s) => s.clone(),
                RispExp::List(l) => {
                    let lstr = l
                        .iter()
                        .map(|x| format!("{}", x))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("({})", lstr)
                }
                RispExp::Func(f) => format!("f@{}", f),
                RispExp::Empty => "".to_owned(),
            }
        )
    }
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
    Not,
    Xor,
    Or,
    And,
    // TODO comparison ops, equality
    // TODO def
    // TODO if
    // TODO functions/lambdas
    // TODO pow

    // Maybe add set!
}

impl RispFunction {
    fn is_builtin(str: &str) -> bool {
        match str {
            "+" | "-" | "*" | "/" | "xor" | "or" | "and" | "not" => true,
            _ => false,
        }
    }

    fn to_string(&self) -> String {
        match self {
            RispFunction::Function(f) => format!("#f@{:?}", addr_of!(f)),
            RispFunction::Builtin(RispBuiltinFunction::Plus) => "+".to_string(),
            RispFunction::Builtin(RispBuiltinFunction::Minus) => "-".to_string(),
            RispFunction::Builtin(RispBuiltinFunction::Multiply) => "*".to_string(),
            RispFunction::Builtin(RispBuiltinFunction::Divide) => "/".to_string(),
            RispFunction::Builtin(RispBuiltinFunction::Not) => "not".to_string(),
            RispFunction::Builtin(RispBuiltinFunction::Xor) => "xor".to_string(),
            RispFunction::Builtin(RispBuiltinFunction::Or) => "or".to_string(),
            RispFunction::Builtin(RispBuiltinFunction::And) => "and".to_string(),
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
            "not" => RispFunction::Builtin(RispBuiltinFunction::Not),
            "xor" => RispFunction::Builtin(RispBuiltinFunction::Xor),
            "or" => RispFunction::Builtin(RispBuiltinFunction::Or),
            "and" => RispFunction::Builtin(RispBuiltinFunction::And),
            _ => panic!("This is not a valid built in!"),
        }
    }
}

impl Debug for RispFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl Display for RispFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
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
                RispToken::Symbol("not".to_string()),
                RispToken::Integer(1),
                RispToken::Integer(2),
                RispToken::RParen
            ])
            .unwrap(),
            RispExp::List(vec![
                RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Not)),
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

    // TODO TEST non lists (ints, floats, bools, symbols etc)
}
