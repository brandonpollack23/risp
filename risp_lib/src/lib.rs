//! Library for tokenizing, parsing, and evaluating my lil' lisp
//! # TODO
//! * Better errors that point to where the error is

pub mod environment;
pub mod error;
pub mod eval;
pub mod parser;
pub mod tokenizer;

mod symbols_constants;
