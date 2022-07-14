use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;
use std::process;

use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;

pub struct Lox {
    had_error: bool,
    had_runtime_error: bool,
}

impl Lox {
    pub fn new() -> Lox {
        Lox {
            had_error: false,
            had_runtime_error: false,
        }
    }

    pub fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(&source);
        let tokens = scanner.scan_tokens();

        let mut parser = Parser::new(tokens);

        let statements = match parser.parse() {
            Ok(s) => s,
            Err(_) => {
                self.had_error = true;
                return;
            }
        };

        let interpreter = Interpreter::new();

        if let Err(_) = interpreter.interpret(&statements) {
            self.had_runtime_error = true;
            return;
        }
    }

    pub fn run_file<P: ?Sized>(&mut self, path: &P)
    where
        P: AsRef<Path>,
    {
        let f = File::open(path).expect("Unable to open the file");

        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();

        reader.read_to_end(&mut buffer).unwrap();

        let source = String::from_utf8(buffer).unwrap();

        self.run(source);

        if self.had_error {
            process::exit(65);
        }
        if self.had_runtime_error {
            process::exit(70);
        }
    }

    pub fn run_prompt(&mut self) {
        loop {
            let mut input = String::new();
            println!("Enter your code:");
            io::stdin().read_line(&mut input).unwrap();

            if input.trim().is_empty() {
                break;
            }

            self.run(input);

            self.had_error = false;
        }
    }
}
