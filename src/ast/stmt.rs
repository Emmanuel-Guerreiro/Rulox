use super::{expr::Expr, token::Token};
#[derive(Debug)]
pub enum Stmt {
    PRINT(Box<Expr>),
    EXPR(Box<Expr>),
    VAR(Box<Token>, Option<Box<Expr>>), //Variable (This token contains tt=declaration), declaration
    BLOCK(Vec<Box<Stmt>>), //The block is literally the content within some brackets. It has its own scope
    IF(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>), //Condition, main body block (It is a block), else block
    WHILE(Box<Expr>, Box<Stmt>),                 //Loop condition, body (It is in fact a block)
}
