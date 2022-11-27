use std::rc::Rc;

use crate::literal::Literal;
use crate::token::Token;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Expr {
    Assign(AssignExpr),
    Binary(BinaryExpr),
    Call(CallExpr),
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
            Expr::Call(ce) => ce.accept(expr_visitor),
            Expr::Binary(be) => be.accept(expr_visitor),
            Expr::Grouping(ge) => ge.accept(expr_visitor),
            Expr::Literal(le) => le.accept(expr_visitor),
            Expr::Logical(le) => le.accept(expr_visitor),
            Expr::Unary(ue) => ue.accept(expr_visitor),
            Expr::Variable(ve) => ve.accept(expr_visitor),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AssignExpr {
    pub name: Token,
    pub value: Rc<Expr>,
}

impl AssignExpr {
    pub fn new(name: Token, value: Rc<Expr>) -> AssignExpr {
        AssignExpr { name, value }
    }

    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        visitor.visit_assignment_expr(self)
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct BinaryExpr {
    pub left: Rc<Expr>,
    pub operator: Token,
    pub right: Rc<Expr>,
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
        let wrapper = Rc::new(Expr::Binary(BinaryExpr::new(
            self.left,
            self.operator,
            self.right,
        )));
        visitor.visit_binary_expr(wrapper, self)
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CallExpr {
    pub callee: Rc<Expr>,
    pub paren: Token,
    pub arguments: Vec<Rc<Expr>>,
}

impl CallExpr {
    pub fn new(
        callee: Rc<Expr>,
        paren: Token,
        arguments: Vec<Rc<Expr>>,
    ) -> CallExpr {
        CallExpr {
            callee,
            paren,
            arguments,
        }
    }

    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        visitor.visit_call_expr(self)
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
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

#[derive(Debug, Hash, PartialEq, Eq)]
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

#[derive(Debug, Hash, PartialEq, Eq)]
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

#[derive(Debug, Hash, PartialEq, Eq)]
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

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct VariableExpr {
    pub name: Token,
}

impl VariableExpr {
    pub fn new(name: Token) -> VariableExpr {
        VariableExpr { name }
    }

    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> T {
        let wrapper = Rc::new(Expr::Variable(VariableExpr::new(self.name)));
        visitor.visit_variable_expr(wrapper, self)
    }
}

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&mut self, wrapper: Rc<Expr>, expr: &BinaryExpr) -> T;
    fn visit_grouping_expr(&mut self, wrapper: Rc<Expr>, expr: &GroupingExpr) -> T;
    fn visit_literal_expr(&self, wrapper: Rc<Expr>, expr: &LiteralExpr) -> T;
    fn visit_logical_exp(&mut self, wrapper: Rc<Expr>, expr: &LogicalExpr) -> T;
    fn visit_unary_expr(&mut self, wrapper: Rc<Expr>, expr: &UnaryExpr) -> T;
    fn visit_variable_expr(&self, wrapper: Rc<Expr>, expr: &VariableExpr) -> T;
    fn visit_assignment_expr(&mut self, wrapper: Rc<Expr>, expr: &AssignExpr) -> T;
    fn visit_call_expr(&mut self, wrapper: Rc<Expr>, expr: &CallExpr) -> T;
}
