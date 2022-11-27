use std::collections::HashMap;
use std::rc::Rc;

use crate::error_reporter::LoxError;
use crate::expr::{
    AssignExpr, BinaryExpr, CallExpr, Expr, ExprVisitor, GroupingExpr,
    LiteralExpr, LogicalExpr, UnaryExpr, VariableExpr,
};
use crate::interpreter::Interpreter;
use crate::stmt::{
    BlockStmt, ExpressionStmt, FunctionStmt, IfStmt, PrintStmt, ReturnStmt,
    Stmt, StmtVisitor, VarStmt, WhileStmt,
};
use crate::token::Token;

pub struct Resolver<'a> {
    interpreter: &'a Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver<'_> {
    pub fn new(interpreter: &Interpreter) -> Resolver {
        Resolver {
            interpreter,
            scopes: Vec::new(),
        }
    }

    fn resolve_statements(&mut self, statements: &Vec<Stmt>) {
        for statement in statements {
            self.resolve_statement(statement);
        }
    }

    fn resolve_statement(&mut self, stmt: &Stmt) {
        stmt.accept(self);
    }

    fn resolve_expression(&mut self, expr: &Rc<Expr>) {
        expr.accept(self);
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme, false);
        }
    }
    fn define(&mut self, name: Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme, true);
        }
    }

    fn resolve_local(&self, name: Token) {
        self.scopes.iter().rev().enumerate().for_each(|(i, s)| {
            if s.contains_key(&name.lexeme) {
                // self.interpreter.resolve();
            }
        });
    }

    fn resolve_function(&mut self, function: &FunctionStmt) {
        self.begin_scope();
        for param in &function.params {
            self.declare(param.clone());
            self.define(param.clone());
        }
        self.resolve_statements(&function.body);
        self.end_scope();
    }
}

impl ExprVisitor<()> for Resolver<'_> {
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> () {
        self.resolve_expression(&expr.left);
        self.resolve_expression(&expr.right);
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> () {
        self.resolve_expression(&expr.expression);
    }

    fn visit_literal_expr(&self, _: &LiteralExpr) -> () {
        ()
    }

    fn visit_logical_exp(&mut self, expr: &LogicalExpr) -> () {
        self.resolve_expression(&expr.left);
        self.resolve_expression(&expr.right);
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> () {
        self.resolve_expression(&expr.right);
    }

    fn visit_variable_expr(&self, expr: &VariableExpr) -> () {
        if let Some(scope) = self.scopes.last() {
            if let Some(v) = scope.get(&expr.name.lexeme) {
                if *v == false {
                    LoxError::resolver_error(
                        expr.name.clone(),
                        "Can't read local variable in its own initializer."
                            .to_string(),
                    );
                }
            }
        }

        self.resolve_local(
            // &Rc::new(Expr::Variable(VariableExpr::new(expr.name.clone()))),
            expr.name.clone(),
        );
    }

    fn visit_assignment_expr(&mut self, expr: &AssignExpr) -> () {
        self.resolve_expression(&expr.value);
        self.resolve_local(expr.name.clone());
    }

    fn visit_call_expr(&mut self, expr: &CallExpr) -> () {
        self.resolve_expression(&expr.callee);

        for argument in &expr.arguments {
            self.resolve_expression(argument);
        }
    }
}

impl StmtVisitor<()> for Resolver<'_> {
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> () {
        self.resolve_expression(&stmt.expression);
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> () {
        self.resolve_expression(&stmt.expression);
    }

    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> () {
        self.declare(stmt.name.clone());
        if let Some(initializer) = &stmt.initializer {
            self.resolve_expression(initializer);
        }
        self.define(stmt.name.clone());
    }

    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> () {
        self.begin_scope();
        self.resolve_statements(&stmt.statements);
        self.end_scope();
    }

    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> () {
        self.resolve_expression(&stmt.condition);
        self.resolve_statement(&stmt.then_branch);
        if let Some(else_branch) = &stmt.else_branch {
            self.resolve_statement(else_branch);
        }
    }

    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> () {
        self.resolve_expression(&stmt.condition);
        self.resolve_statement(&stmt.body);
    }

    fn visit_function_stmt(&mut self, stmt: &FunctionStmt) -> () {
        self.declare(stmt.name.clone());
        self.define(stmt.name.clone());

        self.resolve_function(stmt);
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStmt) -> () {
        if let Some(value) = &stmt.value {
            self.resolve_expression(value);
        }
    }
}
