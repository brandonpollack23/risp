use risp::environment::RispEnv;
use risp::{eval, parser, tokenizer};

#[test]
fn test() {
    let mut env = RispEnv::default();
    let tokens = tokenizer::tokenize("(+ 1 1)").unwrap();
    let expr = parser::parse(&tokens).unwrap();
    println!("Expr: {:?}", expr);
    println!("Evals to: {:?}", eval::eval(expr, &mut env).unwrap());

    let tokens = tokenizer::tokenize("(+ 2 -1)").unwrap();
    let expr = parser::parse(&tokens).unwrap();
    println!("Expr: {:?}", expr);
    println!("Evals to: {:?}", eval::eval(expr, &mut env).unwrap());

    let tokens = tokenizer::tokenize("(- 1 200000 2.0 100_000 abc)").unwrap();
    let expr = parser::parse(&tokens).unwrap();
    println!("Expr: {:?}", expr);

    let tokens = tokenizer::tokenize("(* 1 200000 2.0f 100_000 abc)").unwrap();
    let expr = parser::parse(&tokens).unwrap();
    println!("Expr: {:?}", expr);

    let tokens = tokenizer::tokenize("(/ 1 200000 2f 100_000 abc)").unwrap();
    let expr = parser::parse(&tokens).unwrap();
    println!("Expr: {:?}", expr);

    let tokens = tokenizer::tokenize("(xor 1 200000 2.0 100_000 abc)").unwrap();
    let expr = parser::parse(&tokens).unwrap();
    println!("Expr: {:?}", expr);

    let tokens = tokenizer::tokenize("(or 1 200000 2.0 100_000 abc)").unwrap();
    let expr = parser::parse(&tokens).unwrap();
    println!("Expr: {:?}", expr);

    let tokens = tokenizer::tokenize("(and 1 200000 2.0 100_000 abc)").unwrap();
    let expr = parser::parse(&tokens).unwrap();
    println!("Expr: {:?}", expr);
}
