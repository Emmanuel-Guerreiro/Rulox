use super::token::Token;

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
pub enum Expr {
    Binary(Box<Expr>, Box<Token>, Box<Expr>), //left, operator, right
    Grouping(Box<Expr>),                      //expression
    NumberLit(f64),                           //value
    StringLit(Box<String>),                   //value
    Unary(Box<Token>, Box<Expr>),             //operator, right
    Variable(Box<String>),                    //name | This is used when the variable is referenced
    // Assign(Box<Token>, Box<Expr>),         //name, value
    Boolean(bool),
    Nil,
}
