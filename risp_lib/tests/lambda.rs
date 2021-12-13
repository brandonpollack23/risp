use risp_lib::environment::RispEnv;
use risp_lib::eval::eval;
use risp_lib::parser::RispExp;
use risp_lib::{parser, tokenizer};

#[test]
fn lambda_works() {
    let mut env = RispEnv::default();
    let tokens = tokenizer::tokenize(r#"(def addOne (fn (x) (+ 1 x)))"#).unwrap();
    let expr = parser::parse(&tokens).unwrap();
    assert_eq!(eval(&expr, &mut env).unwrap(), RispExp::Nil);

    let tokens = tokenizer::tokenize(r#"(addOne 37)"#).unwrap();
    let expr = parser::parse(&tokens).unwrap();
    assert_eq!(eval(&expr, &mut env).unwrap(), RispExp::Integer(38));
}
