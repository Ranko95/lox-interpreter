use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error_reporter::LoxError;
use crate::literal::Literal;
use crate::token::Token;

pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn new_with_enclosing(
        enclosing: Rc<RefCell<Environment>>,
    ) -> Environment {
        Environment {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: Token) -> Result<Literal, LoxError> {
        match self.values.get(&name.lexeme) {
            Some(v) => Ok(v.clone()),
            None => {
                if let Some(e) = &self.enclosing {
                    return e.borrow().get(name);
                }

                let message = format!("Undefined variable '{}'.", name.lexeme);
                let error = LoxError::runtime_error(name, message);
                Err(error)
            }
        }
    }

    pub fn assign(
        &mut self,
        name: Token,
        value: Literal,
    ) -> Result<(), LoxError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value);
            return Ok(());
        }

        if let Some(e) = &mut self.enclosing {
            e.borrow_mut().assign(name.clone(), value)?;
            return Ok(());
        }

        let message = format!("Undefined variable '{}'.", name.lexeme);
        let error = LoxError::runtime_error(name, message);
        Err(error)
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }
}
