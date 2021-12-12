use risp_lib::environment::RispEnv;
use risp_lib::eval::eval;
use risp_lib::parser::RispExp;
use risp_lib::{parser, tokenizer};

#[test]
fn if_integration_test() {
    let mut env = RispEnv::default();
    let tokens = tokenizer::tokenize(r#"(if true "true" "false")"#).unwrap();
    let expr = parser::parse(&tokens).unwrap();
    assert_eq!(
        eval(&expr, &mut env).unwrap(),
        RispExp::String("true".to_owned())
    );

    let tokens = tokenizer::tokenize(r#"(if false "true" "false")"#).unwrap();
    let expr = parser::parse(&tokens).unwrap();
    assert_eq!(
        eval(&expr, &mut env).unwrap(),
        RispExp::String("false".to_owned())
    );
}
