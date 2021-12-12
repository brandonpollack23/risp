use pretty_assertions::assert_eq;
use risp_lib::environment::RispEnv;
use risp_lib::eval::eval;
use risp_lib::parser::RispExp;
use risp_lib::{parser, tokenizer};

#[test]
fn test_add() {
    let mut env = RispEnv::default();
    let tokens = tokenizer::tokenize("(+ 1 1)").unwrap();
    let expr = parser::parse(&tokens).unwrap();
    assert_eq!(eval(expr, &mut env).unwrap(), RispExp::Integer(2));
}

#[test]
fn test_with_negative() {
    let mut env = RispEnv::default();
    let tokens = tokenizer::tokenize("(+ 2 -1)").unwrap();
    let expr = parser::parse(&tokens).unwrap();
    assert_eq!(eval(expr, &mut env).unwrap(), RispExp::Integer(1));
}

#[test]
fn test_sub() {
    let mut env = RispEnv::default();
    let tokens = tokenizer::tokenize("(- 2 1)").unwrap();
    let expr = parser::parse(&tokens).unwrap();
    assert_eq!(eval(expr, &mut env).unwrap(), RispExp::Integer(1));
}

#[test]
fn test_nesting() {
    let mut env = RispEnv::default();
    let tokens = tokenizer::tokenize("(- 2 (+ 10 10) 37 (* 10 10))").unwrap();
    let expr = parser::parse(&tokens).unwrap();
    assert_eq!(eval(expr, &mut env).unwrap(), RispExp::Integer(-155));
}

#[test]
fn test_and() {
    let mut env = RispEnv::default();
    let tokens = tokenizer::tokenize(r#"(and true "wakeupneo")"#).unwrap();
    let expr = parser::parse(&tokens).unwrap();
    assert_eq!(eval(expr, &mut env).unwrap(), RispExp::Bool(true));

    let tokens = tokenizer::tokenize(r#"(and false "wakeupneo")"#).unwrap();
    let expr = parser::parse(&tokens).unwrap();
    assert_eq!(eval(expr, &mut env).unwrap(), RispExp::Bool(false));
}
