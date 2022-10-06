use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::Environment;
use crate::error_reporter::LoxError;
use crate::expr::{
    AssignExpr, BinaryExpr, CallExpr, Expr, ExprVisitor, GroupingExpr,
    LiteralExpr, LogicalExpr, UnaryExpr, VariableExpr,
};
use crate::function::LoxFunction;
use crate::literal::Literal;
use crate::native_functions::Clock;
use crate::stmt::{
    BlockStmt, ExpressionStmt, FunctionStmt, IfStmt, PrintStmt, ReturnStmt,
    Stmt, StmtVisitor, VarStmt, WhileStmt,
};
use crate::token::Token;
use crate::token_type::TokenType;

pub struct Interpreter {
    globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
}

impl ExprVisitor<Result<Literal, LoxError>> for Interpreter {
    fn visit_binary_expr(
        &mut self,
        expr: &BinaryExpr,
    ) -> Result<Literal, LoxError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        let operator = &expr.operator.token_type;

        match (left, right) {
            (Literal::Number(left), Literal::Number(right)) => match operator {
                TokenType::Minus => Ok(Literal::Number(left - right)),
                TokenType::Slash => Ok(Literal::Number(left / right)),
                TokenType::Star => Ok(Literal::Number(left * right)),
                TokenType::Plus => Ok(Literal::Number(left + right)),
                TokenType::Greater => Ok(Literal::Bool(left > right)),
                TokenType::GreaterEqual => Ok(Literal::Bool(left >= right)),
                TokenType::Less => Ok(Literal::Bool(left < right)),
                TokenType::LessEqual => Ok(Literal::Bool(left <= right)),
                TokenType::BangEqual => Ok(Literal::Bool(left != right)),
                TokenType::EqualEqual => Ok(Literal::Bool(left == right)),
                _ => {
                    Err(self
                        .error(&expr.operator, "Invalid operation".to_string()))
                }
            },
            (Literal::Number(left), Literal::String(right)) => match operator {
                TokenType::Plus => {
                    Ok(Literal::String(format!("{left}{right}")))
                }
                TokenType::BangEqual => Ok(Literal::Bool(true)),
                TokenType::EqualEqual => Ok(Literal::Bool(false)),
                _ => Err(self.error(
                    &expr.operator,
                    "Operands must be numbers".to_string(),
                )),
            },
            (Literal::String(left), Literal::Number(right)) => match operator {
                TokenType::Plus => {
                    Ok(Literal::String(format!("{left}{right}")))
                }
                TokenType::BangEqual => Ok(Literal::Bool(true)),
                TokenType::EqualEqual => Ok(Literal::Bool(false)),
                _ => Err(self.error(
                    &expr.operator,
                    "Operands must be numbers".to_string(),
                )),
            },
            (Literal::String(left), Literal::String(right)) => match operator {
                TokenType::Plus => {
                    Ok(Literal::String(format!("{left}{right}")))
                }
                TokenType::BangEqual => Ok(Literal::Bool(left != right)),
                TokenType::EqualEqual => Ok(Literal::Bool(left == right)),
                _ => Err(self.error(
                    &expr.operator,
                    "Operands must be numbers".to_string(),
                )),
            },
            (Literal::Bool(left), Literal::Bool(right)) => match operator {
                TokenType::BangEqual => Ok(Literal::Bool(left != right)),
                TokenType::EqualEqual => Ok(Literal::Bool(left == right)),
                _ => Err(self.error(
                    &expr.operator,
                    "Operands must be numbers".to_string(),
                )),
            },
            (Literal::String(_), Literal::Bool(_))
            | (Literal::Bool(_), Literal::String(_)) => match operator {
                TokenType::BangEqual => Ok(Literal::Bool(true)),
                TokenType::EqualEqual => Ok(Literal::Bool(false)),
                _ => Err(self.error(
                    &expr.operator,
                    "Operands must be numbers".to_string(),
                )),
            },
            (Literal::Number(_), Literal::Bool(_))
            | (Literal::Bool(_), Literal::Number(_)) => match operator {
                TokenType::BangEqual => Ok(Literal::Bool(true)),
                TokenType::EqualEqual => Ok(Literal::Bool(false)),
                _ => Err(self.error(
                    &expr.operator,
                    "Operands must be two numbers or two strings.".to_string(),
                )),
            },
            (Literal::Nil, Literal::Nil) => match operator {
                TokenType::BangEqual => Ok(Literal::Bool(false)),
                TokenType::EqualEqual => Ok(Literal::Bool(true)),
                _ => Err(self.error(
                    &expr.operator,
                    "Operands must be two numbers or two strings.".to_string(),
                )),
            },
            (Literal::Nil, _) | (_, Literal::Nil) => match operator {
                TokenType::BangEqual => Ok(Literal::Bool(true)),
                TokenType::EqualEqual => Ok(Literal::Bool(false)),
                _ => Err(self.error(
                    &expr.operator,
                    "Operands must be two numbers or two strings.".to_string(),
                )),
            },
            _ => unreachable!(),
        }
    }

    fn visit_call_expr(
        &mut self,
        expr: &CallExpr,
    ) -> Result<Literal, LoxError> {
        let callee = self.evaluate(&expr.callee)?;
        let mut arguments: Vec<Literal> = Vec::new();
        for argument in &expr.arguments {
            arguments.push(self.evaluate(argument)?);
        }

        let function = match callee {
            Literal::Function(f) => Some(f),
            _ => None,
        };

        if let Some(function) = function {
            if arguments.len() != function.arity() {
                return Err(LoxError::runtime_error(
                    expr.paren.to_owned(),
                    format!(
                        "Expected {} arguments but got {}.",
                        function.arity(),
                        arguments.len()
                    ),
                ));
            }

            Ok(function.call(self, arguments)?)
        } else {
            Err(LoxError::runtime_error(
                expr.paren.to_owned(),
                "Can only call functions and classes.".to_string(),
            ))
        }
    }

    fn visit_grouping_expr(
        &mut self,
        expr: &GroupingExpr,
    ) -> Result<Literal, LoxError> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(
        &self,
        expr: &LiteralExpr,
    ) -> Result<Literal, LoxError> {
        Ok(expr.value.clone().unwrap())
    }

    fn visit_logical_exp(
        &mut self,
        expr: &LogicalExpr,
    ) -> Result<Literal, LoxError> {
        let left = self.evaluate(&expr.left)?;

        if expr.operator.token_type == TokenType::Or {
            if self.is_truthy(&left) {
                return Ok(left);
            }
        } else {
            if !self.is_truthy(&left) {
                return Ok(left);
            }
        }

        Ok(self.evaluate(&expr.right)?)
    }

    fn visit_unary_expr(
        &mut self,
        expr: &UnaryExpr,
    ) -> Result<Literal, LoxError> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::Minus => match right {
                Literal::Number(v) => Ok(Literal::Number(-v)),
                _ => Err(self.error(
                    &expr.operator,
                    "Operand must be a number.".to_string(),
                )),
            },
            TokenType::Bang => Ok(Literal::Bool(!self.is_truthy(&right))),
            _ => unreachable!(),
        }
    }

    fn visit_variable_expr(
        &self,
        expr: &VariableExpr,
    ) -> Result<Literal, LoxError> {
        let value = self.environment.borrow().get(expr.name.clone())?;
        match value {
            Literal::NilImplicit => {
                let error = self.error(
                    &expr.name,
                    "Variable was not explicitly initialized".to_string(),
                );
                Err(error)
            }
            _ => Ok(value),
        }
    }

    fn visit_assignment_expr(
        &mut self,
        expr: &AssignExpr,
    ) -> Result<Literal, LoxError> {
        let value = self.evaluate(&expr.value)?;
        self.environment
            .borrow_mut()
            .assign(expr.name.clone(), value.clone())?;
        Ok(value)
    }
}

impl StmtVisitor<Result<(), LoxError>> for Interpreter {
    fn visit_expression_stmt(
        &mut self,
        stmt: &ExpressionStmt,
    ) -> Result<(), LoxError> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_function_stmt(
        &mut self,
        stmt: &FunctionStmt,
    ) -> Result<(), LoxError> {
        let function = LoxFunction::new(stmt, self.environment.clone());
        self.environment.borrow_mut().define(
            stmt.name.lexeme.to_owned(),
            Literal::Function(Rc::new(function)),
        );

        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> Result<(), LoxError> {
        let literal = self.evaluate(&stmt.condition)?;
        if self.is_truthy(&literal) {
            self.execute(&stmt.then_branch)
        } else if let Some(else_branch) = stmt.else_branch.clone() {
            self.execute(&else_branch)
        } else {
            Ok(())
        }
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Result<(), LoxError> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", value);
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStmt) -> Result<(), LoxError> {
        if let Some(value) = &stmt.value {
            Err(LoxError::return_value(self.evaluate(value)?))
        } else {
            Err(LoxError::return_value(Literal::Nil))
        }
    }

    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> Result<(), LoxError> {
        let value = if let Some(initializer) = &stmt.initializer {
            self.evaluate(&initializer)?
        } else {
            Literal::NilImplicit
        };

        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), value);
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> Result<(), LoxError> {
        let mut literal = self.evaluate(&stmt.condition)?;
        while self.is_truthy(&literal) {
            self.execute(&stmt.body)?;
            literal = self.evaluate(&stmt.condition)?;
        }
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> Result<(), LoxError> {
        self.execute_block(
            &stmt.statements,
            Environment::new_with_enclosing(self.environment.clone()),
        )
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let globals = Rc::new(RefCell::new(Environment::new()));
        globals
            .borrow_mut()
            .define("clock".to_string(), Literal::Function(Rc::new(Clock)));

        let environment = globals.clone();

        Interpreter {
            globals,
            environment,
        }
    }

    pub fn interpret(
        &mut self,
        statements: &Vec<Stmt>,
    ) -> Result<(), LoxError> {
        for statement in statements {
            self.execute(statement)?;
        }

        Ok(())
    }

    pub fn globals(&self) -> Rc<RefCell<Environment>> {
        self.globals.clone()
    }

    pub fn execute_block(
        &mut self,
        statements: &Vec<Stmt>,
        environment: Environment,
    ) -> Result<(), LoxError> {
        let previous = self.environment.clone();
        self.environment = Rc::new(RefCell::new(environment));

        let result = statements
            .iter()
            .try_for_each(|statement| self.execute(statement.clone()));

        self.environment = previous;

        result
    }

    fn evaluate(&mut self, expr: &Rc<Expr>) -> Result<Literal, LoxError> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), LoxError> {
        stmt.accept(self)
    }

    fn is_truthy(&self, literal: &Literal) -> bool {
        match literal {
            Literal::Nil => false,
            Literal::Bool(v) => *v,
            _ => true,
        }
    }

    fn error(&self, token: &Token, message: String) -> LoxError {
        let error = LoxError::runtime_error(token.clone(), message);
        error
    }
}
