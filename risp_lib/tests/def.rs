use risp_lib::environment::RispEnv;
use risp_lib::eval::eval;
use risp_lib::parser::RispExp;
use risp_lib::{parser, tokenizer};

#[test]
fn def() {
    let mut env = RispEnv::default();
    let tokens = tokenizer::tokenize(r#"(def one 1)"#).unwrap();
    let expr = parser::parse(&tokens).unwrap();
    assert_eq!(eval(&expr, &mut env).unwrap(), RispExp::Nil);
    let tokens = tokenizer::tokenize(r#"(def two 2)"#).unwrap();
    let expr = parser::parse(&tokens).unwrap();
    assert_eq!(eval(&expr, &mut env).unwrap(), RispExp::Nil);
    let tokens = tokenizer::tokenize(r#"(+ one two)"#).unwrap();
    let expr = parser::parse(&tokens).unwrap();
    assert_eq!(eval(&expr, &mut env).unwrap(), RispExp::Integer(3));
}
