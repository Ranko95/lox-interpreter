use std::fmt::{self, Display};

use crate::token_type::{Literal, TokenType};

pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub literal: Option<Literal>,
    pub line: u32,
}

impl Token<'_> {
    pub fn new<'a>(
        token_type: TokenType,
        lexeme: &'a str,
        literal: Option<Literal>,
        line: u32,
    ) -> Token<'a> {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.token_type, self.lexeme)
    }
}
