use std::collections::HashMap;

use crate::error_reporter::LoxError;
use crate::literal::Literal;
use crate::token::Token;

pub struct Environment {
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: Token) -> Result<Literal, LoxError> {
        match self.values.get(&name.lexeme) {
            Some(v) => Ok(v.clone()),
            None => {
                let message = format!("Undefined variable '{}'.", name.lexeme);
                let error = LoxError::runtime_error(name, message);
                Err(error)
            }
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }
}
