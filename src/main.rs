mod environment;
mod error;
mod parser;

fn main() {
    let tokens = parser::tokenize("(1 2 abc)").unwrap();
    let expr = parser::parse(&tokens).unwrap();
    println!("Expr: {:?}", expr);
}
