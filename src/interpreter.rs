use std::fmt::Debug;

use crate::{
    ast::{expr::Expr, stmt::Stmt, token::Token, token::TokenType},
    enviroment::{self, Enviroment},
    object::Object,
};
#[derive(Debug, PartialEq)]
pub enum RuntimeError {
    TypeError(String),
    UnknownError,
    UnknownExpression(String),
    UndefinedVariable(String),
}

pub struct Interpreter<'a> {
    enviroment: &'a mut Enviroment,
}

impl<'a> Interpreter<'a> {
    pub fn new(enviroment: &'a mut Enviroment) -> Self {
        Self { enviroment }
    }
}

type EvalRes = Result<Object, RuntimeError>;
type ExcecuteStmtRes = Result<(), RuntimeError>;

impl<'a> Interpreter<'a> {
    pub fn interpret(&mut self, stmts: &'a Vec<Stmt>) {
        for s in stmts.iter() {
            if let Err(e) = self.execute_stmt(s) {
                println!("{:?}", e);
                break;
            }
        }
    }

    fn execute_stmt(&mut self, stmt: &'a Stmt) -> ExcecuteStmtRes {
        match stmt {
            //Todo: Ingore value?
            Stmt::EXPR(e) => _ = self.evauluate_expr(e),
            Stmt::PRINT(e) => {
                let value = self.evauluate_expr(e)?;
                println!("{}", value);
            }
            Stmt::VAR(name, declaration) => self.evaluate_declaration(name, declaration)?,
            // _ => todo!(),
        };

        Ok(())
    }

    fn evaluate_declaration(
        &mut self,
        name: &'a Box<Token>,
        declaration: &'a Option<Box<Expr>>,
    ) -> ExcecuteStmtRes {
        // let val: Option<&'a Object> = match declaration {
        //     None => None,
        //     Some(expr) => self.evauluate_expr(expr),
        // };
        let mut val: Option<Box<Object>> = None;
        if let Some(e) = declaration {
            let x = self.evauluate_expr(e)?;
            val = Some(Box::new(x));
        }

        self.enviroment.define(name.lexeme.clone(), val);
        Ok(())
    }
    fn evauluate_expr(&mut self, expr: &'a Expr) -> EvalRes {
        match expr {
            Expr::NumberLit(n) => return Ok(Object::NumberObj(*n)),
            //Todo: This is quite inefficient
            Expr::StringLit(v) => return Ok(Object::StringObj(*v.clone())),
            Expr::Unary(operator, expr) => return self.handle_unary(operator, expr),
            Expr::Binary(left, op, right) => return self.handle_binary(op, left, right),
            Expr::Boolean(v) => return Ok(Object::BoolObj(*v)),
            Expr::Grouping(expr) => return self.evauluate_expr(expr), //This may be some kind of recursive
            Expr::Nil => return Ok(Object::NullObj),
            Expr::Variable(v) => return self.handle_variable_access(v),
            Expr::Assignment(name, value) => self.handle_assignment(name, value),
        }
    }

    fn handle_assignment(&mut self, name: &Box<String>, value: &'a Box<Expr>) -> EvalRes {
        let v = self.evauluate_expr(&value)?;

        match self.enviroment.assign(name, Box::new(v)) {
            Err(e) => return Err(RuntimeError::UndefinedVariable(format!("{}", e))),
            Ok(v) => Ok(*v),
        }
    }

    fn handle_variable_access(&self, name: &Box<String>) -> EvalRes {
        match self.enviroment.get(&name) {
            Ok(v) => return Ok(*v),
            Err(e) => return Err(RuntimeError::UndefinedVariable(format!("{}", e))),
        }
    }

    fn handle_unary(&mut self, operator: &Box<Token>, expr: &'a Box<Expr>) -> EvalRes {
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

    fn handle_binary(
        &mut self,
        operator: &Box<Token>,
        left: &'a Box<Expr>,
        right: &'a Box<Expr>,
    ) -> EvalRes {
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
