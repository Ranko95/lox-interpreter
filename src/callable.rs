use std::fmt::{Debug, Display};

use crate::error_reporter::LoxError;
use crate::interpreter::Interpreter;
use crate::literal::Literal;

pub trait LoxCallable: Display + Debug {
    fn arity(&self) -> usize;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Literal>,
    ) -> Result<Literal, LoxError>;
}
