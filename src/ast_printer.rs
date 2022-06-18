use crate::ast::{
    BinaryExpr, Expr, ExprVisitor, GroupingExpr, LiteralExpr, UnaryExpr,
};

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &str, exprs: &Vec<&Box<Expr>>) -> String {
        let mut result_string = format!("({name}");
        for expr in exprs {
            result_string = format!("{result_string} {}", expr.accept(self));
        }
        result_string = format!("{result_string})");

        result_string
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> String {
        self.parenthesize(expr.operator.lexeme, &vec![&expr.left, &expr.right])
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> String {
        self.parenthesize("group", &vec![&expr.expression])
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> String {
        match &expr.value {
            None => "nil".to_string(),
            Some(literal) => literal.to_string(),
        }
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> String {
        self.parenthesize(expr.operator.lexeme, &vec![&expr.right])
    }
}
