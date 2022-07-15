use std::rc::Rc;

use crate::{expr::Expr, token::Token};

pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Var(VarStmt),
}

impl Stmt {
    pub fn accept<T>(&self, stmt_visitor: &mut dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Expression(es) => es.accept(stmt_visitor),
            Stmt::Print(ps) => ps.accept(stmt_visitor),
            Stmt::Var(vs) => vs.accept(stmt_visitor),
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

pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Rc<Expr>>,
}

impl VarStmt {
    pub fn new(name: Token, initializer: Option<Rc<Expr>>) -> VarStmt {
        VarStmt { name, initializer }
    }

    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        visitor.visit_var_stmt(self)
    }
}

pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> T;
    fn visit_print_stmt(&self, stmt: &PrintStmt) -> T;
    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> T;
}
