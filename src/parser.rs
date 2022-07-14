use std::rc::Rc;

use crate::error_reporter::LoxError;
use crate::expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr};
use crate::stmt::{ExpressionStmt, PrintStmt, Stmt};
use crate::token::Token;
use crate::token_type::{Literal, TokenType};

/* expression grammar
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;
*/

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl Parser<'_> {
    pub fn new<'a>(tokens: &'a Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements: Vec<Stmt> = vec![];
        while !self.is_at_end() {
            if let Ok(s) = self.statement() {
                statements.push(s);
            }
        }
        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if self.is_match(vec![TokenType::Print]) {
            return self.print_statement();
        }

        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let value = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after value.".to_string(),
        )?;
        Ok(Stmt::Print(PrintStmt::new(Rc::new(value))))
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let expr = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after value.".to_string(),
        )?;
        Ok(Stmt::Expression(ExpressionStmt::new(Rc::new(expr))))
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while self.is_match(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr::new(
                Rc::new(expr),
                operator,
                Rc::new(right),
            ));
        }

        Ok(expr)
    }

    fn is_match(&mut self, types: Vec<TokenType>) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        return self.peek().token_type == token_type;
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        while self.is_match(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr::new(
                Rc::new(expr),
                operator,
                Rc::new(right),
            ));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;

        while self.is_match(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr::new(
                Rc::new(expr),
                operator,
                Rc::new(right),
            ));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while self.is_match(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr::new(
                Rc::new(expr),
                operator,
                Rc::new(right),
            ));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr::new(operator, Rc::new(right))));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(vec![TokenType::False]) {
            Ok(Expr::Literal(LiteralExpr::new(Some(Literal::Bool(false)))))
        } else if self.is_match(vec![TokenType::True]) {
            Ok(Expr::Literal(LiteralExpr::new(Some(Literal::Bool(true)))))
        } else if self.is_match(vec![TokenType::Nil]) {
            Ok(Expr::Literal(LiteralExpr::new(Some(Literal::Nil))))
        } else if self.is_match(vec![TokenType::Number, TokenType::String]) {
            Ok(Expr::Literal(LiteralExpr::new(
                self.previous().literal.clone(),
            )))
        } else if self.is_match(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;

            Ok(Expr::Grouping(GroupingExpr::new(Rc::new(expr))))
        } else {
            let current_token = self.peek().clone();
            Err(self.error(current_token, "Expression expected".to_string()))
        }
    }

    fn consume(
        &mut self,
        token_type: TokenType,
        message: String,
    ) -> Result<Token, LoxError> {
        if self.check(token_type) {
            return Ok(self.advance().clone());
        }

        let current_token = self.peek().clone();
        Err(self.error(current_token, message))
    }

    fn error(&self, token: Token, message: String) -> LoxError {
        let error = LoxError::parse_error(token, message);
        error
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }
}
