use std::rc::Rc;

use crate::ast::{
    BinaryExpr, Expr, ExprVisitor, GroupingExpr, LiteralExpr, UnaryExpr,
};
use crate::error_reporter::LoxError;
use crate::token::Token;
use crate::token_type::{Literal, TokenType};

pub struct Interpreter;

impl ExprVisitor<Result<Literal, LoxError>> for Interpreter {
    fn visit_binary_expr(
        &self,
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
        }
    }

    fn visit_grouping_expr(
        &self,
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

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Literal, LoxError> {
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
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter
    }

    pub fn interpret(&self, expr: &Rc<Expr>) -> Result<String, LoxError> {
        match self.evaluate(expr) {
            Ok(v) => Ok(v.to_string()),
            Err(err) => Err(err),
        }
    }

    fn evaluate(&self, expr: &Rc<Expr>) -> Result<Literal, LoxError> {
        expr.accept(self)
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
