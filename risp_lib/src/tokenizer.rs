use crate::error::{RispError, RispResult};
use crate::symbols_constants::{
    DEF_SYM, EQ_SYM, GTE_SYM, GT_SYM, IF_SYM, LAMBDA_SYM, LPAREN_SYM, LTE_SYM, LT_SYM, NIL_SYM,
    RPAREN_SYM,
};
use regex::Regex;
use std::str::FromStr;

pub fn tokenize(line: &str) -> RispResult<Vec<RispToken>> {
    Tokenizer::new().tokenize(line)
}

struct Tokenizer {
    char_matcher: Regex,
    bool_matcher: Regex,
    int_matcher: Regex,
    float_matcher: Regex,
    symbol_matcher: Regex,
    string_literal_matcher: Regex,
    comparison_op_matcher: Regex,
}

// TODO NOW quote tokens and reader macros for ' (quote), !=
impl Tokenizer {
    fn new() -> Tokenizer {
        Tokenizer {
            char_matcher: Regex::new(r#"\\[\x00-\xFF]"#).unwrap(),
            bool_matcher: Regex::new(r#"^(true|false)$"#).unwrap(),
            int_matcher: Regex::new(r#"^[-+]?[0-9][0-9_]*$"#).unwrap(),
            float_matcher: Regex::new(r#"^[-+]?[0-9]*([.][0-9]+|f|[.][0-9]+f)$"#).unwrap(),
            symbol_matcher: Regex::new(r#"^([A-Za-z_]+[A-Za-z0-9_]*|\+|-|\*|/)$"#).unwrap(),
            string_literal_matcher: Regex::new(r#"^".*"$"#).unwrap(),
            comparison_op_matcher: Regex::new(r#"(<|>|<=|>=|=)"#).unwrap(),
        }
    }

    fn tokenize(&self, line: &str) -> RispResult<Vec<RispToken>> {
        Self::combine_quote_pairs(
            line.replace("(", " ( ")
                .replace(")", " ) ")
                .replace("\"", " \" ")
                .split_whitespace()
                .map(|x| x.to_owned())
                .collect::<Vec<String>>(),
        )?
        .iter()
        .map(|s| self.tokenize_element(&s))
        .collect()
    }

    // TODO NOW allow escape chars for quotes
    fn combine_quote_pairs(inputs: Vec<String>) -> RispResult<Vec<String>> {
        let mut result: Vec<String> = Vec::new();
        let mut quoted_string_builder: String = String::new();
        let mut quoting = false;
        for input in inputs.iter() {
            match (quoting, input.as_str()) {
                (false, s) => {
                    if s == "\"" {
                        quoting = true;
                        quoted_string_builder = s.to_string();
                    } else {
                        result.push(s.to_owned());
                    }
                }
                (true, s) => {
                    if s == "\"" {
                        let start = quoted_string_builder.len() - 1;
                        quoted_string_builder.replace_range(start..start + 1, "\"");
                        quoting = false;
                        result.push(quoted_string_builder.clone())
                    } else {
                        quoted_string_builder.push_str(s);
                        quoted_string_builder.push(' ');
                    }
                }
            }
        }

        Ok(result)
    }

    fn tokenize_element(&self, elem: &str) -> RispResult<RispToken> {
        match elem {
            LPAREN_SYM => Ok(RispToken::LParen),
            RPAREN_SYM => Ok(RispToken::RParen),
            NIL_SYM => Ok(RispToken::Nil),
            DEF_SYM => Ok(RispToken::Def),
            IF_SYM => Ok(RispToken::If),
            LAMBDA_SYM => Ok(RispToken::Fn),

            c if self.char_matcher.is_match(c) => Ok(RispToken::Char(c.chars().nth(0).unwrap())),
            b if self.bool_matcher.is_match(b) => Ok(RispToken::Bool(bool::from_str(b)?)),
            int if self.int_matcher.is_match(int) => {
                Ok(RispToken::Integer(i32::from_str(&int.replace("_", ""))?))
            }
            float if self.float_matcher.is_match(float) => Ok(RispToken::Float(
                f64::from_str(&float.replace("f", ""))
                    .expect(&format!("Unable to parse {} as f64", float)),
            )),

            sym if self.symbol_matcher.is_match(sym) => Ok(RispToken::Symbol(sym.to_owned())),
            string_literal if self.string_literal_matcher.is_match(string_literal) =>
            // Cut out the " chars
            {
                Ok(RispToken::StringLiteral(
                    string_literal[1..string_literal.len() - 1].to_owned(),
                ))
            }

            o if self.comparison_op_matcher.is_match(o) => Ok(Self::tokenize_operator(o)),

            other => Err(RispError::UnrecognizedToken(other.to_string())),
        }
    }

    fn tokenize_operator(o: &str) -> RispToken {
        match o {
            LT_SYM => RispToken::Comparison(ComparisonOp::LT),
            LTE_SYM => RispToken::Comparison(ComparisonOp::LTE),
            GT_SYM => RispToken::Comparison(ComparisonOp::GT),
            GTE_SYM => RispToken::Comparison(ComparisonOp::GTE),
            EQ_SYM => RispToken::Comparison(ComparisonOp::EQ),
            _ => panic!(""),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RispToken {
    Nil,
    LParen,
    RParen,

    Symbol(String),
    StringLiteral(String),

    Bool(bool),
    Float(f64),
    Integer(i32),
    Char(char),

    Comparison(ComparisonOp),

    Def,
    If,

    Fn,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ComparisonOp {
    GT,
    GTE,
    LT,
    LTE,
    EQ,
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::error::RispError;
    use crate::tokenizer::{tokenize, ComparisonOp, RispToken};

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

    #[test]
    fn recognizes_comparison() {
        assert_eq!(
            tokenize("(= < <= > >=)").unwrap(),
            vec![
                RispToken::LParen,
                RispToken::Comparison(ComparisonOp::EQ),
                RispToken::Comparison(ComparisonOp::LT),
                RispToken::Comparison(ComparisonOp::LTE),
                RispToken::Comparison(ComparisonOp::GT),
                RispToken::Comparison(ComparisonOp::GTE),
                RispToken::RParen,
            ]
        );
    }

    #[test]
    fn strings_with_spaces_parse() {
        assert_eq!(
            tokenize(r#"(def test "blarg is blarg")"#).unwrap(),
            vec![
                RispToken::LParen,
                RispToken::Def,
                RispToken::Symbol("test".to_owned()),
                RispToken::StringLiteral("blarg is blarg".to_owned()),
                RispToken::RParen,
            ]
        );
    }

    #[test]
    fn recognizes_def() {
        assert_eq!(
            tokenize(r#"(def test "blarg")"#).unwrap(),
            vec![
                RispToken::LParen,
                RispToken::Def,
                RispToken::Symbol("test".to_owned()),
                RispToken::StringLiteral("blarg".to_owned()),
                RispToken::RParen,
            ]
        );
    }

    #[test]
    fn recognizes_if() {
        assert_eq!(
            tokenize(r#"(if true "true" "false")"#).unwrap(),
            vec![
                RispToken::LParen,
                RispToken::If,
                RispToken::Bool(true),
                RispToken::StringLiteral("true".to_owned()),
                RispToken::StringLiteral("false".to_owned()),
                RispToken::RParen
            ]
        );
    }

    #[test]
    fn recognizes_fn() {
        assert_eq!(
            tokenize(r#"(fn (x) (+ x 1))"#).unwrap(),
            vec![
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
                RispToken::RParen
            ]
        );
    }
}
