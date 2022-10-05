use std::rc::Rc;

use crate::{expr::Expr, token::Token};

#[derive(Debug)]
pub enum Stmt {
    Block(BlockStmt),
    Expression(ExpressionStmt),
    Function(FunctionStmt),
    If(IfStmt),
    Print(PrintStmt),
    Return(ReturnStmt),
    Var(VarStmt),
    While(WhileStmt),
}

impl Stmt {
    pub fn accept<T>(&self, stmt_visitor: &mut dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Block(bs) => bs.accept(stmt_visitor),
            Stmt::Expression(es) => es.accept(stmt_visitor),
            Stmt::Function(fs) => fs.accept(stmt_visitor),
            Stmt::Print(ps) => ps.accept(stmt_visitor),
            Stmt::Return(rs) => rs.accept(stmt_visitor),
            Stmt::Var(vs) => vs.accept(stmt_visitor),
            Stmt::If(ifs) => ifs.accept(stmt_visitor),
            Stmt::While(ws) => ws.accept(stmt_visitor),
        }
    }
}

#[derive(Debug)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

impl BlockStmt {
    pub fn new(statements: Vec<Stmt>) -> BlockStmt {
        BlockStmt { statements }
    }

    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        visitor.visit_block_stmt(self)
    }
}

#[derive(Debug)]
pub struct ExpressionStmt {
    pub expression: Rc<Expr>,
}

impl ExpressionStmt {
    pub fn new(expr: Rc<Expr>) -> ExpressionStmt {
        ExpressionStmt { expression: expr }
    }

    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        visitor.visit_expression_stmt(self)
    }
}

#[derive(Debug)]
pub struct FunctionStmt {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Rc<Vec<Stmt>>,
}

impl FunctionStmt {
    pub fn new(
        name: Token,
        params: Vec<Token>,
        body: Rc<Vec<Stmt>>,
    ) -> FunctionStmt {
        FunctionStmt { name, params, body }
    }

    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        visitor.visit_function_stmt(self)
    }
}

#[derive(Debug)]
pub struct IfStmt {
    pub condition: Rc<Expr>,
    pub then_branch: Rc<Stmt>,
    pub else_branch: Option<Rc<Stmt>>,
}

impl IfStmt {
    pub fn new(
        condition: Rc<Expr>,
        then_branch: Rc<Stmt>,
        else_branch: Option<Rc<Stmt>>,
    ) -> IfStmt {
        IfStmt {
            condition,
            then_branch,
            else_branch,
        }
    }

    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        visitor.visit_if_stmt(self)
    }
}

#[derive(Debug)]
pub struct PrintStmt {
    pub expression: Rc<Expr>,
}

impl PrintStmt {
    pub fn new(expr: Rc<Expr>) -> PrintStmt {
        PrintStmt { expression: expr }
    }

    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        visitor.visit_print_stmt(self)
    }
}

#[derive(Debug)]
pub struct ReturnStmt {
    pub keyword: Token,
    pub value: Option<Rc<Expr>>,
}

impl ReturnStmt {
    pub fn new(keyword: Token, value: Option<Rc<Expr>>) -> ReturnStmt {
        ReturnStmt { keyword, value }
    }

    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        visitor.visit_return_stmt(self)
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct WhileStmt {
    pub condition: Rc<Expr>,
    pub body: Rc<Stmt>,
}

impl WhileStmt {
    pub fn new(condition: Rc<Expr>, body: Rc<Stmt>) -> WhileStmt {
        WhileStmt { condition, body }
    }

    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        visitor.visit_while_stmt(self)
    }
}

pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> T;
    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> T;
    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> T;
    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> T;
    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> T;
    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> T;
    fn visit_function_stmt(&mut self, stmt: &FunctionStmt) -> T;
    fn visit_return_stmt(&mut self, stmt: &ReturnStmt) -> T;
}
