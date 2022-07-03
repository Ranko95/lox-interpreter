use crate::{token::Token, token_type::TokenType};

pub enum LoxError {
    ScanError { line: u32, message: String },
    ParseError { token: Token, message: String },
}

impl LoxError {
    pub fn scan_error(line: u32, message: String) -> LoxError {
        let error = LoxError::ScanError { line, message };
        error.report();
        error
    }

    pub fn parse_error(token: Token, message: String) -> LoxError {
        let error = LoxError::ParseError { token, message };
        error.report();
        error
    }

    fn report(&self) {
        match self {
            LoxError::ScanError { line, message } => {
                eprintln!("[line {}] Error {}: {}", line, "", message);
            }
            LoxError::ParseError { token, message } => {
                if token.token_type == TokenType::EOF {
                    eprintln!(
                        "[line {}] Error {}: {}",
                        token.line, "at end", message
                    );
                } else {
                    let place = format!("at '{}'", token.lexeme);
                    eprintln!(
                        "[line {}] Error {}: {}",
                        token.line, place, message
                    );
                }
            }
        }
    }
}
