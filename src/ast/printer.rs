use super::{
    expr::{self, Expr},
    stmt::{self, Stmt},
    ExprVisitor, StmtVisitor,
};

/**
 * Will print the parsed AST
 */
#[derive(Default)]
pub struct AstPrinter;

#[allow(dead_code)]
impl AstPrinter {
    pub fn print_program(&self, e: &Vec<Stmt>) -> String {
        let mut str = String::from("");

        for s in e.iter() {
            str += &self.print_stmt(s);
        }

        str
    }

    pub fn print_stmt(&self, e: &stmt::Stmt) -> String {
        self.visit_stmt(e)
    }

    pub fn print_expr(&self, e: &expr::Expr) -> String {
        self.visit_expr(e)
    }
    fn parenthesize(&self, name: &str, exprs: Vec<&Box<Expr>>) -> String {
        let mut s = String::from("(");
        s += name;

        for e in exprs.iter() {
            s += " ";
            s += &self.visit_expr(e);
        }
        s += ")";
        s
    }
}

impl StmtVisitor<String> for AstPrinter {
    fn visit_stmt(&self, b: &super::stmt::Stmt) -> String {
        match b {
            //Todo: Add some kind of parenthesize for stmt
            Stmt::EXPR(e) => self.visit_expr(&e),
            Stmt::PRINT(e) => self.visit_expr(&e),
            Stmt::VAR(_, _) => todo!(),
            Stmt::BLOCK(_) => todo!(),
            Stmt::IF(_, _, _) => todo!(),
        }
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_expr(&self, b: &expr::Expr) -> String {
        match b {
            Expr::Binary(right, operator, left) => {
                return self.parenthesize(&operator.lexeme, vec![right, left])
            }
            Expr::Grouping(expression) => return self.parenthesize("group", vec![expression]),
            Expr::Unary(operator, right) => {
                return self.parenthesize(&operator.lexeme, vec![right])
            }
            Expr::NumberLit(value) => return value.to_string(),
            Expr::StringLit(value) => return value.to_string(),
            Expr::Boolean(v) => return v.to_string(),
            Expr::Assignment(name, value) => {
                return self.parenthesize("=", vec![&Box::new(Expr::Variable(name.clone())), value])
            }
            Expr::Variable(v) => return v.to_string(),
            _ => return String::from("nil"),
        }
    }
}

#[cfg(test)]
mod printer_tests {

    use crate::ast::token::{Token, TokenType};

    use super::*;

    #[test]
    fn print_number_literal() {
        let number_literal = Expr::NumberLit(65.0);

        let result = AstPrinter::default().print_expr(&Box::new(number_literal));
        assert!(result == String::from("65"))
    }

    #[test]
    fn print_string_literal() {
        let number_literal = Expr::StringLit(Box::new(String::from("Im string literal")));

        let result = AstPrinter::default().print_expr(&Box::new(number_literal));
        assert_eq!(result, String::from("Im string literal"))
    }
    #[test]
    fn print_factor() {
        let star_tkn = Token::new(TokenType::STAR, "*".to_string(), 1);
        let four_tkn = Expr::NumberLit(4.0);
        let three_tkn = Expr::NumberLit(3.0);
        let factor_expr = Expr::Binary(Box::new(three_tkn), Box::new(star_tkn), Box::new(four_tkn));
        let result = AstPrinter::default().print_expr(&factor_expr);
        assert_eq!(result, "(* 3 4)");
    }

    #[test]
    fn print_binary() {
        let string_literal = Box::new(Expr::StringLit(Box::new(String::from("Im string literal"))));
        let number_literal = Box::new(Expr::NumberLit(65.0));
        let tkn: Box<Token> = Box::new(Token::new(TokenType::BANGEQUAL, "!=".to_string(), 0));

        let binary_expr = Expr::Binary(number_literal, tkn, string_literal);
        let result = AstPrinter::default().print_expr(&binary_expr);

        assert_eq!(result, "(!= 65 Im string literal)".to_string())
    }

    #[test]
    fn print_complex() {
        let minus_tkn = Box::new(Token::new(TokenType::MINUS, String::from("-"), 1));
        let number_literal = Box::new(Expr::NumberLit(123.0));
        let number_literal_2 = Box::new(Expr::NumberLit(45.67));

        let unary_expr = Box::new(Expr::Unary(minus_tkn, number_literal));
        let start_tkn = Box::new(Token::new(TokenType::STAR, String::from("*"), 1));
        let grouping_expr = Box::new(Expr::Grouping(number_literal_2));

        let binary = Box::new(Expr::Binary(unary_expr, start_tkn, grouping_expr));
        let result = AstPrinter::default().print_expr(&binary);
        assert_eq!(result, "(* (- 123) (group 45.67))".to_string());
    }
}
