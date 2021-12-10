mod environment;
mod error;
mod parser;

fn main() {
    let tokens = parser::tokenize("(1 200000 2.0 100_000 abc)").unwrap();
    let expr = parser::parse(&tokens).unwrap();
    println!("Expr: {:?}", expr);
}
