use crate::{token::Token, token_type::Literal};

pub enum Expr<'a> {
    Binary(BinaryExpr<'a>),
    Grouping(GroupingExpr<'a>),
    Literal(LiteralExpr),
    Unary(UnaryExpr<'a>),
}

impl Expr<'_> {
    pub fn accept<T>(&self, expr_visitor: &dyn ExprVisitor<T>) -> T {
        match self {
            Expr::Binary(be) => be.accept(expr_visitor),
            Expr::Grouping(ge) => ge.accept(expr_visitor),
            Expr::Literal(le) => le.accept(expr_visitor),
            Expr::Unary(ue) => ue.accept(expr_visitor),
        }
    }
}

pub struct BinaryExpr<'a> {
    pub left: Box<Expr<'a>>,
    pub operator: Token<'a>,
    pub right: Box<Expr<'a>>,
}

impl BinaryExpr<'_> {
    pub fn new<'a>(
        left: Box<Expr<'a>>,
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    ) -> BinaryExpr<'a> {
        BinaryExpr {
            left,
            operator,
            right,
        }
    }
}

pub struct GroupingExpr<'a> {
    pub expression: Box<Expr<'a>>,
}

impl GroupingExpr<'_> {
    pub fn new<'a>(expression: Box<Expr<'a>>) -> GroupingExpr<'a> {
        GroupingExpr { expression }
    }
}

pub struct LiteralExpr {
    pub value: Option<Literal>,
}

impl LiteralExpr {
    pub fn new(value: Option<Literal>) -> LiteralExpr {
        LiteralExpr { value }
    }
}

pub struct UnaryExpr<'a> {
    pub operator: Token<'a>,
    pub right: Box<Expr<'a>>,
}

impl UnaryExpr<'_> {
    pub fn new<'a>(operator: Token<'a>, right: Box<Expr<'a>>) -> UnaryExpr<'a> {
        UnaryExpr { operator, right }
    }
}

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> T;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> T;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> T;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> T;
}

impl BinaryExpr<'_> {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> T {
        visitor.visit_binary_expr(self)
    }
}

impl GroupingExpr<'_> {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> T {
        visitor.visit_grouping_expr(self)
    }
}

impl LiteralExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> T {
        visitor.visit_literal_expr(self)
    }
}

impl UnaryExpr<'_> {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> T {
        visitor.visit_unary_expr(self)
    }
}
