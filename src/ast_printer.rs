use crate::{
    error::Result,
    expr::{Expr, ExprVisitor},
};

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> Result<String> {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &String, exprs: &[&Box<Expr>]) -> Result<String> {
        let mut sb = format!("( {name}");

        for expr in exprs {
            sb = format!("{sb} {}", expr.accept(self)?);
        }
        sb = format!("{sb})");
        Ok(sb)
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_literal_expr(&self, expr: &crate::expr::LiteralExpr) -> Result<String> {
        if let Some(value) = &expr.value {
            Ok(format!("{:?}",value))
        } else {
            Ok("nil".to_string())
        }
    }

    fn visit_unary_expr(&self, expr: &crate::expr::UnaryExpr) -> Result<String> {
        self.parenthesize(&expr.operator.lexeme, &[&expr.right])
    }

    fn visit_binary_expr(&self, expr: &crate::expr::BinaryExpr) -> Result<String> {
        self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right])
    }

    fn visit_grouping_expr(&self, expr: &crate::expr::GroupingExpr) -> Result<String> {
        self.parenthesize(&"group".to_string(), &[&expr.expression])
    }
}