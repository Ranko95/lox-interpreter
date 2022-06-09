use std::env;
use std::io::{ BufReader, Read, self };
use std::process;
use std::path::Path;
use std::fs::File;

fn run(source: String) {
    println!("{}", source);
}


fn run_file<P: ?Sized>(path: &P)
where
    P: AsRef<Path>,
{
    let f = File::open(path).expect("Unable to open the file");

    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer).unwrap();

    let source = String::from_utf8(buffer).unwrap();

    run(source);
}

fn run_prompt() {
    // reads a line of input from the user on the command line and returns the result.
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    if args_len > 2 {
        println!("Usage: jlox [script]");
        process::exit(64);
    } else if args_len == 2 {
        run_file(Path::new(&args[1]));
    } else {
        run_prompt();
    }
}
