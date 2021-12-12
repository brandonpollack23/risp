use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ptr::addr_of;

use crate::error::{
    RispError, RispResult, EXPECTED_ARGS_LIST_FOR_FN, EXPECTED_FN_DEF_FOR_FN, TRAILING_TOKENS,
    UNEXPECTED_CLOSING_PAREN,
};
use crate::parser::RispFunction::Builtin;
use crate::symbols_constants::{
    AND_SYM, DEF_SYM, DIV_SYM, EQ_SYM, GTE_SYM, GT_SYM, IF_SYM, LTE_SYM, LT_SYM, MINUS_SYM,
    MULTIPLY_SYM, NOT_SYM, OR_SYM, PLUS_SYM, XOR_SYM,
};
use crate::tokenizer::{ComparisonOp, RispToken};

pub fn parse(tokens: &[RispToken]) -> RispResult<RispExp> {
    let result = parse_internal(tokens)?;
    if result.1.len() > 0 {
        return Err(RispError::UnexpectedToken(
            result.1.get(0).unwrap().clone(),
            TRAILING_TOKENS.to_owned(),
        ));
    }
    Ok(result.0)
}

fn parse_internal<'a>(tokens: &[RispToken]) -> RispResult<(RispExp, &[RispToken])> {
    if tokens.len() == 0 {
        return Ok((RispExp::Empty, &[]));
    }
    let (token, rest) = tokens.split_first().unwrap();
    match &token {
        RispToken::Fn => read_fn(rest),
        RispToken::LParen => read_seq(rest),
        RispToken::RParen => Err(RispError::UnexpectedToken(
            token.clone(),
            UNEXPECTED_CLOSING_PAREN.to_owned(),
        )),
        _ => Ok((parse_atom(token)?, rest)),
    }
}

fn read_fn(rest: &[RispToken]) -> RispResult<(RispExp, &[RispToken])> {
    // TODO NOW push in fn that has lists not just list
    let (args_list_begin, rest_args) = rest.split_first().ok_or(RispError::UnterminatedList)?;
    let (args, rest_fn) = match args_list_begin {
        RispToken::LParen => read_seq(rest_args)?,
        _ => Err(RispError::UnexpectedToken(
            args_list_begin.clone(),
            EXPECTED_ARGS_LIST_FOR_FN.to_owned(),
        ))?,
    };
    let (function_def_begin, rest_fn_def) =
        rest_fn.split_first().ok_or(RispError::UnterminatedList)?;
    let (body, rest_done) = match function_def_begin {
        RispToken::LParen => read_seq(rest_fn_def)?,
        _ => Err(RispError::UnexpectedToken(
            args_list_begin.clone(),
            EXPECTED_FN_DEF_FOR_FN.to_owned(),
        ))?,
    };

    Ok((
        RispExp::Func(RispFunction::Function {
            params: Box::new(args.clone()),
            body: Box::new(body.clone()),
        }),
        rest_done,
    ))
}

fn read_seq(rest: &[RispToken]) -> RispResult<(RispExp, &[RispToken])> {
    let mut res: Vec<RispExp> = vec![];
    let mut xs = rest;
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
        RispToken::Nil => Ok(RispExp::Nil),
        RispToken::Char(c) => Ok(RispExp::Char(*c)),
        RispToken::Bool(b) => Ok(RispExp::Bool(*b)),
        RispToken::Integer(i) => Ok(RispExp::Integer(*i)),
        RispToken::Float(f) => Ok(RispExp::Float(*f)),

        RispToken::Symbol(str) => parse_symbol(str),
        RispToken::StringLiteral(str) => Ok(RispExp::String(str.to_owned())),

        RispToken::Comparison(cmp) => Ok(cmp.into()),

        RispToken::Def => Ok(RispExp::Func(Builtin(RispBuiltinFunction::Def))),
        RispToken::If => Ok(RispExp::Func(Builtin(RispBuiltinFunction::If))),

        t @ (RispToken::LParen | RispToken::RParen | RispToken::Fn) => {
            Err(RispError::UnexpectedToken(t.clone(), "".to_owned()))
        }
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

impl From<&ComparisonOp> for RispExp {
    fn from(cmp: &ComparisonOp) -> Self {
        match cmp {
            ComparisonOp::GT => RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::GT)),
            ComparisonOp::GTE => RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::GTE)),
            ComparisonOp::LT => RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::LT)),
            ComparisonOp::LTE => RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::LTE)),
            ComparisonOp::EQ => RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::EQ)),
        }
    }
}

impl PartialOrd for RispExp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (RispExp::Integer(a), RispExp::Integer(b)) => a.partial_cmp(b),
            (RispExp::Integer(a), RispExp::Float(b)) => f64::from(*a).partial_cmp(b),
            (RispExp::Float(a), RispExp::Integer(b)) => a.partial_cmp(&f64::from(*b)),
            (RispExp::Float(a), RispExp::Float(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
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

// TODO change from Box to Ref and propogate
#[derive(Clone, PartialEq)]
pub enum RispFunction {
    Function {
        params: Box<RispExp>,
        body: Box<RispExp>,
    },
    Builtin(RispBuiltinFunction),
}

#[derive(Clone, PartialEq)]
pub enum RispBuiltinFunction {
    // Math
    Plus,
    Minus,
    Multiply,
    Divide,

    // Boolean
    Not,
    Xor,
    Or,
    And,

    // Comparison
    // TODO make these cascade (n-ary) like in clojure
    LT,
    LTE,
    GT,
    GTE,
    EQ,

    Def,
    If,
    // Maybe add set!
}

impl RispFunction {
    fn is_builtin(str: &str) -> bool {
        match str {
            PLUS_SYM | MINUS_SYM | MULTIPLY_SYM | DIV_SYM | XOR_SYM | OR_SYM | AND_SYM
            | NOT_SYM => true,
            _ => false,
        }
    }

    fn to_string(&self) -> String {
        match self {
            RispFunction::Function { params, body } => {
                format!(
                    "lambda \n\targs: {:?} \n\tbody: {:?}",
                    params.as_ref(),
                    body.as_ref()
                )
            }
            RispFunction::Builtin(RispBuiltinFunction::Plus) => PLUS_SYM.to_owned(),
            RispFunction::Builtin(RispBuiltinFunction::Minus) => MINUS_SYM.to_owned(),
            RispFunction::Builtin(RispBuiltinFunction::Multiply) => MULTIPLY_SYM.to_owned(),
            RispFunction::Builtin(RispBuiltinFunction::Divide) => DIV_SYM.to_owned(),

            RispFunction::Builtin(RispBuiltinFunction::Not) => NOT_SYM.to_owned(),
            RispFunction::Builtin(RispBuiltinFunction::Xor) => XOR_SYM.to_owned(),
            RispFunction::Builtin(RispBuiltinFunction::Or) => OR_SYM.to_owned(),
            RispFunction::Builtin(RispBuiltinFunction::And) => AND_SYM.to_owned(),

            RispFunction::Builtin(RispBuiltinFunction::LT) => LT_SYM.to_owned(),
            RispFunction::Builtin(RispBuiltinFunction::LTE) => LTE_SYM.to_owned(),
            RispFunction::Builtin(RispBuiltinFunction::GT) => GT_SYM.to_owned(),
            RispFunction::Builtin(RispBuiltinFunction::GTE) => GTE_SYM.to_owned(),
            RispFunction::Builtin(RispBuiltinFunction::EQ) => EQ_SYM.to_owned(),

            RispFunction::Builtin(RispBuiltinFunction::Def) => DEF_SYM.to_owned(),
            RispFunction::Builtin(RispBuiltinFunction::If) => IF_SYM.to_owned(),
        }
    }
}

impl From<&str> for RispFunction {
    fn from(str: &str) -> Self {
        match str {
            PLUS_SYM => RispFunction::Builtin(RispBuiltinFunction::Plus),
            MINUS_SYM => RispFunction::Builtin(RispBuiltinFunction::Minus),
            MULTIPLY_SYM => RispFunction::Builtin(RispBuiltinFunction::Multiply),
            DIV_SYM => RispFunction::Builtin(RispBuiltinFunction::Divide),
            NOT_SYM => RispFunction::Builtin(RispBuiltinFunction::Not),
            XOR_SYM => RispFunction::Builtin(RispBuiltinFunction::Xor),
            OR_SYM => RispFunction::Builtin(RispBuiltinFunction::Or),
            AND_SYM => RispFunction::Builtin(RispBuiltinFunction::And),
            LT_SYM => RispFunction::Builtin(RispBuiltinFunction::LT),
            LTE_SYM => RispFunction::Builtin(RispBuiltinFunction::LTE),
            GT_SYM => RispFunction::Builtin(RispBuiltinFunction::GT),
            GTE_SYM => RispFunction::Builtin(RispBuiltinFunction::GTE),
            EQ_SYM => RispFunction::Builtin(RispBuiltinFunction::EQ),
            DEF_SYM => RispFunction::Builtin(RispBuiltinFunction::Def),
            IF_SYM => RispFunction::Builtin(RispBuiltinFunction::If),
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

    #[test]
    fn def_works() {
        assert_eq!(
            parse(&[
                RispToken::LParen,
                RispToken::Def,
                RispToken::Symbol("lukesfather".to_owned()),
                RispToken::StringLiteral("darthvader".to_owned()),
                RispToken::RParen
            ])
            .unwrap(),
            RispExp::List(vec![
                RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Def)),
                RispExp::Symbol("lukesfather".to_owned()),
                RispExp::String("darthvader".to_owned())
            ])
        );
    }

    #[test]
    fn if_works() {
        assert_eq!(
            parse(&[
                RispToken::LParen,
                RispToken::If,
                RispToken::Bool(true),
                RispToken::StringLiteral("true".to_owned()),
                RispToken::StringLiteral("false".to_owned()),
                RispToken::RParen
            ])
            .unwrap(),
            RispExp::List(vec![
                RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::If)),
                RispExp::Bool(true),
                RispExp::String("true".to_owned()),
                RispExp::String("false".to_owned())
            ])
        );
    }

    #[test]
    fn fn_works() {
        assert_eq!(
            parse(&[
                RispToken::LParen,
                RispToken::Fn,
                RispToken::LParen,
                RispToken::Symbol("x".to_owned()),
                RispToken::RParen,
                RispToken::LParen,
                RispToken::Symbol("+".to_owned()),
                RispToken::Symbol("x".to_owned()),
                RispToken::Integer(1),
                RispToken::RParen,
                RispToken::RParen,
            ])
            .unwrap(),
            RispExp::List(vec![RispExp::Func(RispFunction::Function {
                params: Box::new(RispExp::List(vec![RispExp::Symbol("x".to_string())])),
                body: Box::new(RispExp::List(vec![
                    RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Plus)),
                    RispExp::Symbol("x".to_string()),
                    RispExp::Integer(1),
                ])),
            })])
        );
    }

    // TODO TEST non lists (ints, floats, bools, symbols etc)
}
