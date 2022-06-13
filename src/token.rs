use std::fmt::{self, Display};

use crate::token_type::TokenType;

pub struct Token<'a> {
    token_type: TokenType,
    lexeme: &'a str,
    line: u32,
}

impl Token<'_> {
    pub fn new<'a>(token_type: TokenType, lexeme: &'a str, line: u32) -> Token<'a> {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.token_type, self.lexeme)
    }
}
