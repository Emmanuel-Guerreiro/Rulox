pub mod expr;
pub mod parser;
pub mod printer;
pub mod scanner;
pub mod token;

mod integration_test;
pub trait ExprVisitor<T> {
    fn visit_expr(&self, b: &expr::Expr) -> T;
}
