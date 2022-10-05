use std::rc::Rc;

use crate::error_reporter::LoxError;
use crate::expr::{
    AssignExpr, BinaryExpr, CallExpr, Expr, GroupingExpr, LiteralExpr,
    LogicalExpr, UnaryExpr, VariableExpr,
};
use crate::literal::Literal;
use crate::stmt::{
    BlockStmt, ExpressionStmt, FunctionStmt, IfStmt, PrintStmt, ReturnStmt,
    Stmt, VarStmt, WhileStmt,
};
use crate::token::Token;
use crate::token_type::TokenType;

const MAX_ARGUMENTS_COUNT: usize = 255;

/* expression grammar
expression     → assignment ;
assignment     → IDENTIFIER "=" assignment
               | logic_or ;
logic_or       → logic_and ( "or" logic_and )* ;
logic_and      → equality ( "and" equality )* ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | call ;
call           → primary ( "(" arguments? ")" )* ;
arguments      → expression ( "," expression )* ;
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
            if let Ok(s) = self.declaration() {
                statements.push(s);
            }
        }
        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.assignment()
    }

    fn declaration(&mut self) -> Result<Stmt, LoxError> {
        let result = if self.is_match(vec![TokenType::Var]) {
            self.var_declaration()
        } else if self.is_match(vec![TokenType::Fun]) {
            self.function("function")
        } else {
            self.statement()
        };

        if result.is_err() {
            self.synchronize();
        }

        result
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if self.is_match(vec![TokenType::For]) {
            return self.for_statement();
        }
        if self.is_match(vec![TokenType::If]) {
            return self.if_statement();
        }
        if self.is_match(vec![TokenType::Print]) {
            return self.print_statement();
        }
        if self.is_match(vec![TokenType::Return]) {
            return self.return_statement();
        }
        if self.is_match(vec![TokenType::While]) {
            return self.while_statement();
        }
        if self.is_match(vec![TokenType::LeftBrace]) {
            return Ok(Stmt::Block(BlockStmt::new(self.block()?)));
        }

        self.expression_statement()
    }

    fn for_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(
            TokenType::LeftParen,
            "Expect '(' after 'for'.".to_string(),
        )?;

        let initializer = if self.is_match(vec![TokenType::Semicolon]) {
            None
        } else if self.is_match(vec![TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if !self.check(TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after loop condition.".to_string(),
        )?;

        let increment = if !self.check(TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(
            TokenType::RightParen,
            "Expect ')' after for clauses.".to_string(),
        )?;

        let mut body = self.statement()?;

        if let Some(inc) = increment {
            body = Stmt::Block(BlockStmt::new(vec![
                body,
                Stmt::Expression(ExpressionStmt::new(Rc::new(inc))),
            ]))
        };

        let final_condition = if let Some(cond) = condition {
            cond
        } else {
            Expr::Literal(LiteralExpr::new(Some(Literal::Bool(true))))
        };
        body = Stmt::While(WhileStmt::new(
            Rc::new(final_condition),
            Rc::new(body),
        ));

        if let Some(init) = initializer {
            body = Stmt::Block(BlockStmt::new(vec![init, body]));
        };

        Ok(body)
    }

    fn if_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(
            TokenType::LeftParen,
            "Expect '(' after 'if'.".to_string(),
        )?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            "Expect ')' after if condition.".to_string(),
        )?;

        let then_branch = self.statement()?;

        let else_branch = if self.is_match(vec![TokenType::Else]) {
            let stmt = self.statement()?;
            Some(Rc::new(stmt))
        } else {
            None
        };

        Ok(Stmt::If(IfStmt::new(
            Rc::new(condition),
            Rc::new(then_branch),
            else_branch,
        )))
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let value = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after value.".to_string(),
        )?;
        Ok(Stmt::Print(PrintStmt::new(Rc::new(value))))
    }

    fn return_statement(&mut self) -> Result<Stmt, LoxError> {
        let keyword = self.previous().to_owned();
        let value = match self.check(TokenType::Semicolon) {
            true => None,
            false => Some(Rc::new(self.expression()?)),
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after return value.".to_string(),
        )?;

        Ok(Stmt::Return(ReturnStmt::new(keyword, value)))
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name = self.consume(
            TokenType::Identifier,
            "Expect variable name.".to_string(),
        )?;

        let initializer = if self.is_match(vec![TokenType::Equal]) {
            Some(Rc::new(self.expression()?))
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.".to_string(),
        )?;

        Ok(Stmt::Var(VarStmt::new(name, initializer)))
    }

    fn while_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(
            TokenType::LeftParen,
            "Expect '(' after 'while'.".to_string(),
        )?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            "Expect ')' after condition.".to_string(),
        )?;
        let body = self.statement()?;

        Ok(Stmt::While(WhileStmt::new(
            Rc::new(condition),
            Rc::new(body),
        )))
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let expr = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after value.".to_string(),
        )?;
        Ok(Stmt::Expression(ExpressionStmt::new(Rc::new(expr))))
    }

    fn function(&mut self, kind: &str) -> Result<Stmt, LoxError> {
        let name =
            self.consume(TokenType::Identifier, format!("Expect {kind} name"))?;

        self.consume(
            TokenType::LeftParen,
            format!("Expect '(' after {kind} name."),
        )?;

        let mut parameters: Vec<Token> = Vec::new();

        if !self.check(TokenType::RightParen) {
            parameters.push(self.consume(
                TokenType::Identifier,
                "Expect parameter name.".to_string(),
            )?);
            loop {
                if self.is_match(vec![TokenType::Comma]) {
                    if parameters.len() >= MAX_ARGUMENTS_COUNT {
                        self.error(
                            self.peek().clone(),
                            "Can't have more than 255 parameters.".to_string(),
                        );
                    }

                    parameters.push(self.consume(
                        TokenType::Identifier,
                        "Expect parameter name.".to_string(),
                    )?);
                } else {
                    break;
                }
            }
        }

        self.consume(
            TokenType::RightParen,
            "Expect ')' after parameters.".to_string(),
        )?;

        self.consume(
            TokenType::LeftBrace,
            format!("Expect '{{' before {kind} body."),
        )?;

        let body = self.block()?;

        Ok(Stmt::Function(FunctionStmt::new(
            name,
            parameters,
            Rc::new(body),
        )))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(
            TokenType::RightBrace,
            "Expect '}' after block.".to_string(),
        )?;
        Ok(statements)
    }

    fn assignment(&mut self) -> Result<Expr, LoxError> {
        let expr = self.or()?;

        if self.is_match(vec![TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            match expr {
                Expr::Variable(ve) => {
                    let name = ve.name;
                    return Ok(Expr::Assign(AssignExpr::new(
                        name,
                        Rc::new(value),
                    )));
                }
                _ => {
                    self.error(
                        equals,
                        "Invalid assignment target.".to_string(),
                    );
                }
            }
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.and()?;

        while self.is_match(vec![TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::Logical(LogicalExpr::new(
                Rc::new(expr),
                operator,
                Rc::new(right),
            ));
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.equality()?;

        while self.is_match(vec![TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Logical(LogicalExpr::new(
                Rc::new(expr),
                operator,
                Rc::new(right),
            ))
        }

        Ok(expr)
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

        self.call()
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, LoxError> {
        let mut arguments: Vec<Rc<Expr>> = Vec::new();

        if !self.check(TokenType::RightParen) {
            arguments.push(Rc::new(self.expression()?));
            loop {
                if self.is_match(vec![TokenType::Comma]) {
                    if arguments.len() >= MAX_ARGUMENTS_COUNT {
                        self.error(
                            self.peek().clone(),
                            format!(
                                "Can't have more than {} arguments.",
                                MAX_ARGUMENTS_COUNT
                            ),
                        );
                    }
                    arguments.push(Rc::new(self.expression()?));
                } else {
                    break;
                }
            }
        }

        let paren = self.consume(
            TokenType::RightParen,
            "Expect ')' after arguments.".to_string(),
        )?;

        Ok(Expr::Call(CallExpr::new(Rc::new(callee), paren, arguments)))
    }

    fn call(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.primary()?;

        loop {
            if self.is_match(vec![TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
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
        } else if self.is_match(vec![TokenType::Identifier]) {
            let name = self.previous().clone();
            Ok(Expr::Variable(VariableExpr::new(name)))
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
