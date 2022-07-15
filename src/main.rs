use std::env;
use std::path::Path;
use std::process;

mod ast_printer;
mod environment;
mod error_reporter;
mod expr;
mod interpreter;
mod literal;
mod lox;
mod parser;
mod scanner;
mod stmt;
mod token;
mod token_type;

use lox::Lox;

fn main() {
    let mut lox = Lox::new();

    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    if args_len > 2 {
        println!("Usage: rlox [script]");
        process::exit(64);
    } else if args_len == 2 {
        lox.run_file(Path::new(&args[1]));
    } else {
        lox.run_prompt();
    }
}
