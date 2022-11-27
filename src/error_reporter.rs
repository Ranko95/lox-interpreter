use crate::{literal::Literal, token::Token, token_type::TokenType};

pub enum LoxError {
    ScanError { line: u32, message: String },
    ParseError { token: Token, message: String },
    RuntimeError { token: Token, message: String },
    ResolverError { token: Token, message: String },
    SystemError { message: String },
    ReturnValue { value: Literal },
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

    pub fn runtime_error(token: Token, message: String) -> LoxError {
        let error = LoxError::RuntimeError { token, message };
        error.report();
        error
    }

    pub fn resolver_error(token: Token, message: String) -> LoxError {
        let error = LoxError::ResolverError { token, message };
        error.report();
        error
    }

    pub fn system_error(message: String) -> LoxError {
        let error = LoxError::SystemError { message };
        error.report();
        error
    }

    pub fn return_value(value: Literal) -> LoxError {
        LoxError::ReturnValue { value }
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
            LoxError::RuntimeError { token, message } => {
                if token.token_type == TokenType::EOF {
                    eprintln!(
                        "[line {}] Error {}: {}",
                        token.line, "at end", message
                    );
                } else {
                    eprintln!("{} \n[line {}]", message, token.line);
                }
            }
            LoxError::ResolverError { token, message } => {
                eprintln!("{} \n[line {}]", message, token.line);
            }
            LoxError::SystemError { message } => {
                eprintln!("System Error: {message}");
            }
            _ => {}
        }
    }
}
