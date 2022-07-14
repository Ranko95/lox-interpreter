use std::rc::Rc;

use crate::expr::Expr;

pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
}

impl Stmt {
    pub fn accept<T>(&self, stmt_visitor: &dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Expression(es) => es.accept(stmt_visitor),
            Stmt::Print(ps) => ps.accept(stmt_visitor),
        }
    }
}

pub struct ExpressionStmt {
    pub expression: Rc<Expr>,
}

impl ExpressionStmt {
    pub fn new(expr: Rc<Expr>) -> ExpressionStmt {
        ExpressionStmt { expression: expr }
    }

    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> T {
        visitor.visit_expression_stmt(self)
    }
}

pub struct PrintStmt {
    pub expression: Rc<Expr>,
}

impl PrintStmt {
    pub fn new(expr: Rc<Expr>) -> PrintStmt {
        PrintStmt { expression: expr }
    }

    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> T {
        visitor.visit_print_stmt(self)
    }
}

pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> T;
    fn visit_print_stmt(&self, stmt: &PrintStmt) -> T;
}
