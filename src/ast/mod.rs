pub mod expr;
mod integration_test;
pub mod parser;
pub mod printer;
pub mod scanner;
pub mod stmt;
pub mod token;
pub trait ExprVisitor<T> {
    fn visit_expr(&self, b: &expr::Expr) -> T;
}

pub trait StmtVisitor<T> {
    fn visit_stmt(&self, b: &stmt::Stmt) -> T;
}
