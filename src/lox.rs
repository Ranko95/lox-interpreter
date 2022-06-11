use std::io::{ BufReader, Read, self };
use std::path::Path;
use std::fs::File;
use std::process;


pub struct Lox {
  had_error: bool,
}

impl Lox {
  pub fn new() -> Lox {
    Lox {
      had_error: false,
    }
  }

  pub fn run(&self, source: String) {
    println!("{}", source);
  }
  
  pub fn report(&self, line: i32, place: &str, message: &str) {
    eprintln!("[line {}] Error {}: {}", line, place, message);
  }
  
  pub fn error(&self, line: i32, message: &str) {
    self.report(line, "", message);
  }
  
  pub fn run_file<P: ?Sized>(&self, path: &P)
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
