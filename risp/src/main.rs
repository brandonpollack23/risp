//! This is the repl for risp
//! readline, support soft returns, eval print loop
mod risp_lineread_validator;

use risp_lib::environment::RispEnv;
use risp_lib::eval::eval;
use risp_lib::parser::parse;
use risp_lib::tokenizer::tokenize;
use rustyline;
use rustyline::error::ReadlineError;
use rustyline::Editor;

const REPL_HISTORY_PATH: &str = ".repl_history";

// TODO multiline editing with Validator trait from rustyline

fn main() {
    let env = &mut RispEnv::default();
    let mut rl = rustyline::Editor::<()>::with_config(
        rustyline::Config::builder().auto_add_history(true).build(),
    );
    setup_history(&mut rl);

    loop {
        let readline = rl.readline("lisp> ");
        match readline {
            Ok(input) => match ep(input, env) {
                Err(e) => eprintln!("{}", e),
                _ => {}
            },
            Err(ReadlineError::Eof) => handle_exit(&mut rl),
            Err(e) => eprintln!("Error reading input: {}", e),
        }
    }
}

fn setup_history(rl: &mut Editor<()>) {
    if let Err(e) = rl.load_history(".repl_history") {
        eprintln!("Error loading repl history: {}", e);
        if let Err(e) = std::fs::File::create(".repl_history") {
            eprintln!("Cannot create history file! {}", e)
        }
    }
}

fn ep(input: String, env: &mut RispEnv) -> anyhow::Result<()> {
    let token_stream = tokenize(&input)?;
    let exp = parse(&token_stream)?;
    println!("{}", eval(exp, env)?);
    Ok(())
}

fn handle_exit(rl: &mut Editor<()>) {
    println!("Goodbye!");
    rl.save_history(&REPL_HISTORY_PATH)
        .expect("Error saving history");
    std::process::exit(0);
}
