use crate::{error::Result, token::{Literal, Token}};

pub enum Expr {
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Grouping(GroupingExpr)
}

impl Expr {
    pub fn accept<T>(&self, expr_visitor: &dyn ExprVisitor<T>) -> Result<T> {
        match self {
            Expr::Literal(le) => expr_visitor.visit_literal_expr(le),
            Expr::Unary(ue) => expr_visitor.visit_unary_expr(ue),
            Expr::Binary(be) => expr_visitor.visit_binary_expr(be),
            Expr::Grouping(ge) => expr_visitor.visit_grouping_expr(ge),
        }
    }
}

pub struct LiteralExpr {
    pub value: Option<Literal>
}

pub struct GroupingExpr {
    pub expression : Box<Expr>
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>
}

pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>
}

impl LiteralExpr {
    pub fn new(value: Option<Literal>) -> Self {
        Self { value }
    }
}

impl GroupingExpr {
    pub fn new(expression: Expr) -> Self {
        Self {
            expression: Box::new(expression)
        }
    }
}

impl UnaryExpr {
    pub fn new(operator: Token, right: Expr) -> Self {
        Self {
            operator,
            right: Box::new(right)
        }
    }
}

impl BinaryExpr {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right)
        }
    }
}

pub trait ExprVisitor<T> {
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<T>;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<T>;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<T>;
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<T>;
}

