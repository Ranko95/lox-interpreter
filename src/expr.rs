use std::rc::Rc;

use crate::literal::Literal;
use crate::token::Token;

pub enum Expr {
    Assign(AssignExpr),
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Logical(LogicalExpr),
    Unary(UnaryExpr),
    Variable(VariableExpr),
}

impl Expr {
    pub fn accept<T>(&self, expr_visitor: &mut dyn ExprVisitor<T>) -> T {
        match self {
            Expr::Assign(ae) => ae.accept(expr_visitor),
            Expr::Binary(be) => be.accept(expr_visitor),
            Expr::Grouping(ge) => ge.accept(expr_visitor),
            Expr::Literal(le) => le.accept(expr_visitor),
            Expr::Logical(le) => le.accept(expr_visitor),
            Expr::Unary(ue) => ue.accept(expr_visitor),
            Expr::Variable(ve) => ve.accept(expr_visitor),
        }
    }
}

pub struct AssignExpr {
    pub name: Token,
    pub value: Rc<Expr>,
}

pub struct BinaryExpr {
    pub left: Rc<Expr>,
    pub operator: Token,
    pub right: Rc<Expr>,
}

impl AssignExpr {
    pub fn new(name: Token, value: Rc<Expr>) -> AssignExpr {
        AssignExpr { name, value }
    }

    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        visitor.visit_assignment_expr(self)
    }
}

impl BinaryExpr {
    pub fn new(left: Rc<Expr>, operator: Token, right: Rc<Expr>) -> BinaryExpr {
        BinaryExpr {
            left,
            operator,
            right,
        }
    }

    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        visitor.visit_binary_expr(self)
    }
}

pub struct GroupingExpr {
    pub expression: Rc<Expr>,
}

impl GroupingExpr {
    pub fn new(expression: Rc<Expr>) -> GroupingExpr {
        GroupingExpr { expression }
    }

    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        visitor.visit_grouping_expr(self)
    }
}

pub struct LiteralExpr {
    pub value: Option<Literal>,
}

impl LiteralExpr {
    pub fn new(value: Option<Literal>) -> LiteralExpr {
        LiteralExpr { value }
    }

    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> T {
        visitor.visit_literal_expr(self)
    }
}

pub struct LogicalExpr {
    pub left: Rc<Expr>,
    pub operator: Token,
    pub right: Rc<Expr>,
}

impl LogicalExpr {
    pub fn new(
        left: Rc<Expr>,
        operator: Token,
        right: Rc<Expr>,
    ) -> LogicalExpr {
        LogicalExpr {
            left,
            operator,
            right,
        }
    }

    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        visitor.visit_logical_exp(self)
    }
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Rc<Expr>,
}

impl UnaryExpr {
    pub fn new(operator: Token, right: Rc<Expr>) -> UnaryExpr {
        UnaryExpr { operator, right }
    }

    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        visitor.visit_unary_expr(self)
    }
}

pub struct VariableExpr {
    pub name: Token,
}

impl VariableExpr {
    pub fn new(name: Token) -> VariableExpr {
        VariableExpr { name }
    }

    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> T {
        visitor.visit_variable_expr(self)
    }
}

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> T;
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> T;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> T;
    fn visit_logical_exp(&mut self, expr: &LogicalExpr) -> T;
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> T;
    fn visit_variable_expr(&self, expr: &VariableExpr) -> T;
    fn visit_assignment_expr(&mut self, expr: &AssignExpr) -> T;
}
