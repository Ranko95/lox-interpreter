use crate::token::Token;
use crate::token_type::TokenType;
use crate::lox;

pub struct Scanner<'a> {
  source: &'a str,
  source_length: usize,
  tokens: Vec<Token<'a>>,
  start: usize,
  current: usize,
  line: u32,
}

impl Scanner<'_> {
  pub fn new(source: &str) -> Scanner {
    Scanner {
      source,
      source_length: source.len() - 1,
      tokens: Vec::new(),
      start: 0,
      current: 0,
      line: 1,
    }
  }

  pub fn scan_tokens(&mut self) -> &Vec<Token> {
    while !self.is_at_end() {
      self.start = self.current;
      self.scan_token();
    }

    self.tokens.push(
      Token::new(
        TokenType::EOF,
        "",
        self.line,
      )
    );
    
    &self.tokens
  }

  fn advance(&mut self) -> char {
    let current = self.current;
    self.current += 1;

    self.source.chars().nth(current.try_into().unwrap()).unwrap()
  }

  fn add_token(&mut self, token_type: TokenType) {
    let text = &self.source[self.start..self.current];
    self.tokens.push(Token::new(token_type, text, self.line));
  }
  
  fn scan_token(&mut self) {
    let c = self.advance();

    match c {
      '(' => self.add_token(TokenType::LeftParen),
      ')' => self.add_token(TokenType::RightParen),
      '{' => self.add_token(TokenType::LeftBrace),
      '}' => self.add_token(TokenType::RightBrace),
      ',' => self.add_token(TokenType::Comma),
      '.' => self.add_token(TokenType::Dot),
      '-' => self.add_token(TokenType::Minus),
      '+' => self.add_token(TokenType::Plus),
      ';' => self.add_token(TokenType::Semicolon),
      '*' => self.add_token(TokenType::Star),
      '!' => {
        if self.match_char('=') {
          self.add_token(TokenType::BangEqual);
        } else {
          self.add_token(TokenType::Bang);
        }
      },
      '=' => {
        if self.match_char('=') {
          self.add_token(TokenType::EqualEqual);
        } else {
          self.add_token(TokenType::Equal);
        }
      },
      '<' => {
        if self.match_char('=') {
          self.add_token(TokenType::LessEqual);
        } else {
          self.add_token(TokenType::Less);
        }
      },
      '>' => {
        if self.match_char('=') {
          self.add_token(TokenType::GreaterEqual);
        } else {
          self.add_token(TokenType::Greater);
        }
      },
      '/' => {
        if self.match_char('/') {
          // A comment goes until the end of the line.
          while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
          }
        } else {
          self.add_token(TokenType::Slash);
        }
      },
      ' ' | '\r' | '\t' => {},
      '\n' => self.line += 1,
      _ => lox::error(self.line, "Unexpected character."),
    }
  }

  fn is_at_end(&self) -> bool {
    self.current >= self.source_length.try_into().unwrap()
  }

  fn match_char(&mut self, expected: char) -> bool {
    if self.is_at_end() {
      return false;
    }

    let current_char = self
      .source
      .chars()
      .nth(
        self.current
        .try_into()
        .unwrap()
      ).unwrap();
    if current_char != expected {
      return false;
    }

    self.current += 1;
    return true;
  }

  fn peek(&self) -> char {
    if self.is_at_end() {
      return '\0';
    }

    return self.source.chars().nth(self.current.try_into().unwrap()).unwrap();
  }
}
