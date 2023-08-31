use super::{expr::Expr, token::Token};
#[derive(Debug)]
pub enum Stmt {
    PRINT(Box<Expr>),
    EXPR(Box<Expr>),
    VAR(Box<Token>, Option<Box<Expr>>),
}
