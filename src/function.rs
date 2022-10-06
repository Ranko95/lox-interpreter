use std::fmt::Display;
use std::rc::Rc;

use crate::callable::LoxCallable;
use crate::environment::Environment;
use crate::error_reporter::LoxError;
use crate::interpreter::Interpreter;
use crate::literal::Literal;
use crate::stmt::{FunctionStmt, Stmt};
use crate::token::Token;

#[derive(Debug)]
pub struct LoxFunction {
    name: Token,
    params: Vec<Token>,
    body: Rc<Vec<Stmt>>,
}

impl LoxFunction {
    pub fn new(declaration: &FunctionStmt) -> LoxFunction {
        LoxFunction {
            name: declaration.name.to_owned(),
            params: declaration.params.to_owned(),
            body: Rc::clone(&declaration.body),
        }
    }
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Literal>,
    ) -> Result<Literal, LoxError> {
        let mut environment =
            Environment::new_with_enclosing(interpreter.globals());

        for (param, arg) in self.params.iter().zip(arguments.iter()) {
            environment.define(param.lexeme.to_owned(), arg.clone());
        }

        match interpreter.execute_block(&self.body, environment) {
            Ok(_) => {}
            Err(e) => match e {
                LoxError::ReturnValue { value } => return Ok(value),
                _ => {}
            },
        }

        Ok(Literal::Nil)
    }
}

impl Display for LoxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fn_name = self.name.lexeme.to_owned();
        write!(f, "<fn {fn_name}>")
    }
}
