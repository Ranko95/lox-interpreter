use std::collections::HashMap;

use crate::error_reporter;
use crate::token::Token;
use crate::token_type::{Literal, TokenType};

const RADIX: u32 = 10;

pub struct Scanner<'a> {
    source: &'a str,
    source_length: usize,
    tokens: Vec<Token<'a>>,
    keywords: HashMap<&'a str, TokenType>,
    start: usize,
    current: usize,
    line: u32,
}

impl Scanner<'_> {
    pub fn new(source: &str) -> Scanner {
        let mut keywords: HashMap<&str, TokenType> = HashMap::new();
        keywords.insert("and", TokenType::And);
        keywords.insert("class", TokenType::Class);
        keywords.insert("else", TokenType::Else);
        keywords.insert("false", TokenType::False);
        keywords.insert("for", TokenType::For);
        keywords.insert("fun", TokenType::Fun);
        keywords.insert("if", TokenType::If);
        keywords.insert("nil", TokenType::Nil);
        keywords.insert("or", TokenType::Or);
        keywords.insert("print", TokenType::Print);
        keywords.insert("return", TokenType::Return);
        keywords.insert("super", TokenType::Super);
        keywords.insert("this", TokenType::This);
        keywords.insert("true", TokenType::True);
        keywords.insert("var", TokenType::Var);
        keywords.insert("while", TokenType::While);

        Scanner {
            source,
            source_length: source.len() - 1,
            tokens: Vec::new(),
            keywords,
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

        self.tokens
            .push(Token::new(TokenType::EOF, "", None, self.line));

        &self.tokens
    }

    fn advance(&mut self) -> char {
        let current = self.current;
        self.current += 1;

        self.source
            .chars()
            .nth(current.try_into().unwrap())
            .unwrap()
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    fn add_token_with_literal(
        &mut self,
        token_type: TokenType,
        literal: Option<Literal>,
    ) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
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
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.match_char('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_char('*') {
                    // A multiline comment goes untill closing */ sign
                    // Multiline comments can be nested
                    let mut stack = vec![self.line];

                    while stack.len() != 0 && !self.is_at_end() {
                        let char = self.peek();
                        let next_char = self.peek_next();
                        if char == '*' && next_char == '/' {
                            stack.pop();
                            self.advance();
                        } else if char == '/' && next_char == '*' {
                            stack.push(self.line);
                            self.advance();
                        } else if char == '\n' {
                            self.line += 1;
                        }
                        self.advance();
                    }

                    if stack.len() != 0 && self.is_at_end() {
                        let line = stack.pop().unwrap_or(self.line);
                        error_reporter::error(
                            line,
                            "Don't forget to close a multiline comment with closing sign: '*/'.",
                        );
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            c => {
                if c.is_digit(RADIX) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    error_reporter::error(self.line, "Unexpected character.");
                }
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source_length
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        let current_char = self
            .source
            .chars()
            .nth(self.current.try_into().unwrap())
            .unwrap();

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

        return self
            .source
            .chars()
            .nth(self.current.try_into().unwrap())
            .unwrap();
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        return self
            .source
            .chars()
            .nth((self.current + 1).try_into().unwrap())
            .unwrap();
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error_reporter::error(self.line, "Unterminated string.");
            return;
        }

        self.advance();

        // Trim the surrounding quotes.
        let value = &self.source[self.start + 1..self.current - 1];

        self.add_token_with_literal(
            TokenType::String,
            Some(Literal::String(value.to_string())),
        );
    }

    fn number(&mut self) {
        while self.peek().is_digit(RADIX) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(RADIX) {
            self.advance();

            while self.peek().is_digit(RADIX) {
                self.advance();
            }
        }

        let value = &self.source[self.start..self.current];

        self.add_token_with_literal(
            TokenType::Number,
            Some(Literal::Number(value.parse().unwrap())),
        );
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = self
            .keywords
            .get(text)
            .unwrap_or(&TokenType::Identifier)
            .clone();

        self.add_token(token_type);
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || c.is_digit(RADIX)
    }
}
