use regex::Regex;
use std::str::FromStr;

use crate::error::{RispError, RispResult};

pub fn tokenize(line: &str) -> RispResult<Vec<RispToken>> {
    Tokenizer::new().tokenize(line)
}

pub fn parse(tokens: &[RispToken]) -> RispResult<RispExp> {
    parse_internal(tokens).map(|exp| exp.0)
}

fn parse_internal<'a>(tokens: &[RispToken]) -> RispResult<(RispExp, &[RispToken])> {
    let (token, rest) = tokens
        .split_first()
        .ok_or(RispError::Reason("Could not get token".to_string()))?;
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
        RispToken::Symbol(str) => Ok(RispExp::Symbol(str.clone())),
        other => Err(RispError::UnexpectedToken(other.clone())),
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

struct Tokenizer {
    int_matcher: Regex,
    float_matcher: Regex,
    symbol_matcher: Regex,
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            int_matcher: Regex::new(r#"[0-9]+[_0-9]*[0-9]"#).unwrap(),
            float_matcher: Regex::new(r#"[0-9]+\.?[0-9]*f?"#).unwrap(),
            symbol_matcher: Regex::new(r#"[A-Za-z_]+[A-Za-z0-9_]*"#).unwrap(),
        }
    }

    fn tokenize(&self, line: &str) -> RispResult<Vec<RispToken>> {
        line.replace("(", " ( ")
            .replace(")", " ) ")
            .split_whitespace()
            .map(|x| x.to_string())
            .map(|s| match s.as_str() {
                "(" => Ok(RispToken::LParen),
                ")" => Ok(RispToken::RParen),
                token => self.tokenize_element(token),
            })
            .collect()
    }

    fn tokenize_element(&self, elem: &str) -> RispResult<RispToken> {
        match elem {
            int if self.int_matcher.is_match(int) => {
                Ok(RispToken::Integer(i64::from_str(&int.replace("_", ""))?))
            }
            float if self.float_matcher.is_match(float) => Ok(RispToken::Float(
                f64::from_str(float).expect(&format!("Unable to parse {} as f64", float)),
            )),
            sym if self.symbol_matcher.is_match(sym) => Ok(RispToken::Symbol(sym.to_string())),
            other => Err(RispError::UnrecognizedToken(other.to_string())),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RispToken {
    LParen,
    RParen,
    Symbol(String),
    Float(f64),
    Integer(i64),
    // TODO char and string
}

#[derive(Clone, Debug)]
pub enum RispExp {
    Symbol(String),
    Integer(i64),
    Float(f64),
    List(Vec<RispExp>),
}
