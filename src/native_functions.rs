use std::fmt::Display;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error_reporter::LoxError;
use crate::interpreter::Interpreter;
use crate::literal::Literal;

use crate::callable::LoxCallable;

#[derive(Debug)]
pub struct Clock;

impl LoxCallable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Literal>,
    ) -> Result<Literal, LoxError> {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(d) => Ok(Literal::Number(d.as_micros() as f64)),
            Err(e) => Err(LoxError::system_error(format!(
                "Invalid duration: {:?}",
                e.duration()
            ))),
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "native clock function")
    }
}
