use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    if args_len > 1 {
        println!("Usage: jlox [script]");
        process::exit(64);
    } else if args_len == 1 {
        // runFile(args[0]);
    } else {
        // runPrompt();
    }
}
