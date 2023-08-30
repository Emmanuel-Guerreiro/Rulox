use super::expr::Expr;
#[derive(Debug)]
pub enum Stmt {
    PRINT(Expr),
    EXPR(Expr),
}
