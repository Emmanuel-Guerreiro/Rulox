use std::fmt::Debug;

use crate::{
    ast::{expr::Expr, token::Token, token::TokenType},
    object::Object,
};
#[derive(Debug, PartialEq)]
pub enum RuntimeError {
    TypeError(String),
    UnknownError,
    UnknownExpression(String),
}

pub struct Interpreter;

impl Default for Interpreter {
    fn default() -> Self {
        Self {}
    }
}

type EvalRes = Result<Object, RuntimeError>;

impl<'a> Interpreter {
    pub fn interpret(&self, expr: &'a Expr) {
        match self.evauluate_expr(expr) {
            Err(e) => println!("{:?}", e),
            Ok(o) => println!("{}", o),
        }
    }

    fn evauluate_expr(&self, expr: &'a Expr) -> EvalRes {
        match expr {
            Expr::NumberLit(n) => return Ok(Object::NumberObj(*n)),
            //Todo: This is quite inefficient
            Expr::StringLit(v) => return Ok(Object::StringObj(*v.clone())),
            Expr::Unary(operator, expr) => return self.handle_unary(operator, expr),
            Expr::Binary(left, op, right) => return self.handle_binary(op, left, right),
            Expr::Boolean(v) => return Ok(Object::BoolObj(*v)),
            Expr::Grouping(expr) => return self.evauluate_expr(expr), //This may be some kind of recursive
            Expr::Nil => return Ok(Object::NullObj),
        }
    }

    fn handle_unary(&self, operator: &Box<Token>, expr: &Box<Expr>) -> EvalRes {
        //This should be a number
        //Can it be forced?
        let evaluated_expression = self.evauluate_expr(&expr)?;
        match operator.token_type {
            TokenType::MINUS => {
                let a = (-evaluated_expression)?;
                return Ok(a);
            }
            TokenType::BANG => {
                let a = (!evaluated_expression)?;
                return Ok(a);
            }
            _ => Err(RuntimeError::UnknownExpression(format!(
                "Unexpected operator {:?} on unary expression",
                operator.token_type
            ))),
        }
    }

    fn handle_binary(&self, operator: &Box<Token>, left: &Box<Expr>, right: &Box<Expr>) -> EvalRes {
        let left_evaluated = self.evauluate_expr(&left)?;
        let right_evaluated = self.evauluate_expr(&right)?;

        let res = match operator.token_type {
            TokenType::BANGEQUAL => left_evaluated.neq(&right_evaluated),
            TokenType::EQUALEQUAL => left_evaluated.eq(&right_evaluated),
            TokenType::GREATER => left_evaluated.gt(&right_evaluated),
            TokenType::GREATEREQUAL => left_evaluated.gte(&right_evaluated),
            TokenType::LESS => left_evaluated.lt(&right_evaluated),
            TokenType::LESSEQUAL => left_evaluated.lte(&right_evaluated),
            TokenType::MINUS => left_evaluated - right_evaluated,
            TokenType::PLUS => left_evaluated + right_evaluated,
            TokenType::SLASH => left_evaluated / right_evaluated,
            TokenType::STAR => left_evaluated * right_evaluated,
            _ => {
                return Err(RuntimeError::UnknownExpression(format!(
                    "Unexpected token for binary operator {:?}",
                    operator.token_type
                )))
            }
        };
        res
    }
}
