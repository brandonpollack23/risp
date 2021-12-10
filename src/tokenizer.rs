use crate::error::{RispError, RispResult};
use regex::Regex;
use std::str::FromStr;

pub fn tokenize(line: &str) -> RispResult<Vec<RispToken>> {
    Tokenizer::new().tokenize(line)
}

pub struct Tokenizer {
    int_matcher: Regex,
    float_matcher: Regex,
    symbol_matcher: Regex,
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            int_matcher: Regex::new(r#"^[-+]?[0-9][0-9_]*$"#).unwrap(),
            float_matcher: Regex::new(r#"^[-+]?[0-9]*([.][0-9]+|f|[.][0-9]+f)$"#).unwrap(),
            symbol_matcher: Regex::new(r#"^([A-Za-z_]+[A-Za-z0-9_]*|\+|-|\*|/)$"#).unwrap(),
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
                Ok(RispToken::Integer(i32::from_str(&int.replace("_", ""))?))
            }
            float if self.float_matcher.is_match(float) => Ok(RispToken::Float(
                f64::from_str(&float.replace("f", ""))
                    .expect(&format!("Unable to parse {} as f64", float)),
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
    Integer(i32),
    // TODO char and string
    // TODO nil
}
