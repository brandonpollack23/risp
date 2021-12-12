use crate::error::{RispError, RispResult};
use regex::Regex;
use std::str::FromStr;

pub fn tokenize(line: &str) -> RispResult<Vec<RispToken>> {
    Tokenizer::new().tokenize(line)
}

pub struct Tokenizer {
    bool_matcher: Regex,
    int_matcher: Regex,
    float_matcher: Regex,
    symbol_matcher: Regex,
    string_literal_matcher: Regex,
}

// TODO quote tokens and reader macros for '
impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            bool_matcher: Regex::new(r#"^(true|false)$"#).unwrap(),
            int_matcher: Regex::new(r#"^[-+]?[0-9][0-9_]*$"#).unwrap(),
            float_matcher: Regex::new(r#"^[-+]?[0-9]*([.][0-9]+|f|[.][0-9]+f)$"#).unwrap(),
            symbol_matcher: Regex::new(r#"^([A-Za-z_]+[A-Za-z0-9_]*|\+|-|\*|/)$"#).unwrap(),
            string_literal_matcher: Regex::new(r#"^".*"$"#).unwrap(),
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
            b if self.bool_matcher.is_match(b) => Ok(RispToken::Bool(bool::from_str(b)?)),
            int if self.int_matcher.is_match(int) => {
                Ok(RispToken::Integer(i32::from_str(&int.replace("_", ""))?))
            }
            float if self.float_matcher.is_match(float) => Ok(RispToken::Float(
                f64::from_str(&float.replace("f", ""))
                    .expect(&format!("Unable to parse {} as f64", float)),
            )),
            sym if self.symbol_matcher.is_match(sym) => Ok(RispToken::Symbol(sym.to_owned())),
            string_literal if self.string_literal_matcher.is_match(string_literal) => {
                Ok(RispToken::StringLiteral(string_literal.to_owned()))
            }
            other => Err(RispError::UnrecognizedToken(other.to_string())),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RispToken {
    LParen,
    RParen,
    Symbol(String),
    StringLiteral(String),
    Bool(bool),
    Float(f64),
    Integer(i32),
    // TODO char and string
    // TODO nil
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::error::RispError;
    use crate::tokenizer::{tokenize, RispToken};

    #[test]
    fn recognizes_operators_as_symbols() {
        operator_assertion("+");
        operator_assertion("-");
        operator_assertion("*");
        operator_assertion("/");
    }

    fn operator_assertion(op: &str) {
        assert_eq!(
            tokenize(op).unwrap(),
            vec![RispToken::Symbol(op.to_string())]
        );
    }

    #[test]
    fn recognizes_strings_as_symbols() {
        assert_eq!(
            tokenize("engage").unwrap(),
            vec![RispToken::Symbol("engage".to_string())]
        )
    }

    #[test]
    fn empty_string_is_nothing() {
        assert_eq!(tokenize("").unwrap(), vec![])
    }

    #[test]
    fn integer_works() {
        assert_eq!(tokenize("123").unwrap(), vec![RispToken::Integer(123)])
    }

    #[test]
    fn neg_integer_works() {
        assert_eq!(tokenize("-123").unwrap(), vec![RispToken::Integer(-123)])
    }

    #[test]
    fn float_works() {
        assert_eq!(tokenize("123.0").unwrap(), vec![RispToken::Float(123f64)]);
        assert_eq!(tokenize("123f").unwrap(), vec![RispToken::Float(123f64)]);
        assert_eq!(tokenize("123.0f").unwrap(), vec![RispToken::Float(123f64)]);
        assert_eq!(
            tokenize("-123.0f").unwrap(),
            vec![RispToken::Float(-123f64)]
        );
    }

    #[test]
    fn overflow_int_works() {
        let err = tokenize(&i64::MAX.to_string());
        assert!(err.is_err());
        assert!(matches!(
            err.unwrap_err(),
            RispError::ParseIntError(std::num::ParseIntError { .. })
        ))
    }

    #[test]
    fn overflow_float_works() {
        assert_eq!(
            tokenize(&format!("11{}f", f64::MAX.to_string())).unwrap(),
            vec![RispToken::Float(f64::INFINITY)]
        );
        assert_eq!(
            tokenize(&format!("-11{}f", f64::MAX.to_string())).unwrap(),
            vec![RispToken::Float(f64::NEG_INFINITY)]
        );
    }

    #[test]
    fn recognizes_bools() {
        assert_eq!(tokenize("true").unwrap(), vec![RispToken::Bool(true)]);
        assert_eq!(tokenize("false").unwrap(), vec![RispToken::Bool(false)]);
    }

    #[test]
    fn recognizes_empty_list() {
        assert_eq!(
            tokenize("()").unwrap(),
            vec![RispToken::LParen, RispToken::RParen]
        );
    }

    #[test]
    fn recognizes_list() {
        assert_eq!(
            tokenize("(1 2 3)").unwrap(),
            vec![
                RispToken::LParen,
                RispToken::Integer(1),
                RispToken::Integer(2),
                RispToken::Integer(3),
                RispToken::RParen
            ]
        );
    }
}
