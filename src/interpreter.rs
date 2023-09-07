use std::fmt::Debug;

use crate::{
    ast::{expr::Expr, stmt::Stmt, token::Token, token::TokenType},
    enviroment::Environment,
    object::Object,
};
#[derive(Debug, PartialEq)]
pub enum RuntimeError {
    TypeError(String),
    UnknownError,
    UnknownExpression(String),
    UndefinedVariable(String),
    ScopeError(Option<String>),
}

pub struct Interpreter<'a> {
    enviroment: &'a mut Environment,
}

impl<'a> Interpreter<'a> {
    pub fn new(enviroment: &'a mut Environment) -> Self {
        Self { enviroment }
    }
}

type EvalRes = Result<Object, RuntimeError>;
type ExcecuteStmtRes = Result<(), RuntimeError>;

impl<'a> Interpreter<'a> {
    pub fn interpret(&mut self, stmts: &'a Vec<Stmt>) {
        for s in stmts.iter() {
            match self.execute_stmt(s) {
                Err(e) => {
                    println!("{:?}", e);
                    break;
                }
                Ok(_) => {}
            }
        }
    }

    fn execute_stmt(&mut self, stmt: &'a Stmt) -> ExcecuteStmtRes {
        match stmt {
            //Todo: Ingore value?
            Stmt::EXPR(e) => {
                _ = self.evauluate_expr(e);
                return Ok(());
            }
            Stmt::PRINT(e) => match self.evauluate_expr(e) {
                Ok(value) => {
                    println!("{}", value);
                    return Ok(());
                }
                Err(e) => {
                    panic!("{:?}", e);
                }
            },
            Stmt::VAR(name, declaration) => {
                self.evaluate_declaration(name, declaration)?;
                return Ok(());
            }
            Stmt::BLOCK(stmts) => {
                _ = self.excecute_block(stmts);
                return Ok(());
            } // _ => todo!(),
        }
    }

    fn excecute_block(&mut self, stmts: &'a Vec<Box<Stmt>>) -> ExcecuteStmtRes {
        //Initialize the new local scope for the block.
        //Any new variable will be added to the current scope,
        //But assignations and gets will try in the local,
        //and,on fail, will try one level above (Until global)
        self.enviroment.add_new_local()?;

        for stmt in stmts {
            if let Err(err) = self.execute_stmt(stmt) {
                self.enviroment.remove_local()?;
                return Err(err);
            }
        }

        self.enviroment.remove_local()?;
        Ok(())
    }

    fn evaluate_declaration(
        &mut self,
        name: &'a Box<Token>,
        declaration: &'a Option<Box<Expr>>,
    ) -> ExcecuteStmtRes {
        let mut val: Option<Object> = None;
        if let Some(e) = declaration {
            let x = self.evauluate_expr(e)?;
            val = Some(x);
        }

        self.enviroment.define(&name.lexeme.clone(), val)?;
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

        let r = self.enviroment.assign(name, v)?;
        Ok(r)
    }

    fn handle_variable_access(&self, name: &Box<String>) -> EvalRes {
        match self.enviroment.get(&name) {
            None => {
                return Err(RuntimeError::UndefinedVariable(name.to_string()));
            }
            Some(r) => Ok(r.clone()),
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
