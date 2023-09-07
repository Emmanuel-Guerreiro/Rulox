/*
# Grammar to parse:

*       program        → declaration* EOF ;

*       declaration    → varDecl          -> This is a kind of stmt
*                      | statement ;

*       varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;

*   ==================== STMTs ====================

*       statement      → exprStmt       -
*                      | ifStmt         |
*                      | printStmt ;    | Match the option
*                      | blockStmt ;    -

*       printStmt      → "print" expression ";" ;

*       exprStmt       → expression ";" ;

*       blockStmt      → "{" declaration* "}" ";" In fact it is kind of a subprogram. But this notation seems more clear

        TODO: Add support of if-else
*       ifStmt         → "if" "(" expression ")" blockStmt ("else" blockStmt)?

*   ==================== EXPRs ====================

*		expression     → assignment ;

*       assignment     → IDENTIFIER "=" assignment
*                      | equality ;
*
*		equality       → comparison ( ( "!=" | "==" ) comparison )* ;
*                                     '-------------' -> Match
*                                   '-----------------------------' -> While || Undf
*
*		comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
*                               '-------------------------' -> Match
*                             '----------------------------------' -> While || Undf

*		term           → factor ( ( "-" | "+" ) factor )* ;
*                                 '-----------' -> Match
*                               '----------------------' -> While || Undf
*
*		factor         → unary ( ( "/" | "*" ) unary )* ;
*                                '-----------' -> Match
*                             '----------------------------------' -> While || Undf
*		unary          → ( "!" | "-" ) unary    -
*                        '-----------' -> Match | -> Match entre ambos
*		               | primary ;              -
*
*
*		primary        → NUMBER | STRING | "true" | "false" | "nil"
*		               | "(" expression ")" ;

*   Que surge naturalmente de esto -> Una funcion peek que devuelva Optional<Expr>
*                                     para hacer pattern matching en funcion de la
*                                     expresion hija
*/

use std::fmt::{format, Display};

use super::{
    expr::Expr,
    stmt::Stmt,
    token::{Token, TokenType},
};

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken(String),
    NonValidAssigmentTarget,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedToken(e) => write!(f, "[Error] - Parsing error: {}", e),
            Self::NonValidAssigmentTarget => {
                write!(f, "[Error] - Parsing error: Non valid assigment target")
            }
        }
    }
}

pub type ExprParserResult = Result<Expr, ParserError>;
pub type StmtParserResult = Result<Stmt, ParserError>;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

//Public API and util methods
impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }
    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut stmts: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            //TODO: !HANDLE ERROR TO AVOID PANIC ON FIRST ERROR
            let stmt = self.declaration()?;
            stmts.push(stmt);
        }

        Ok(stmts)
    }

    pub fn previous(&mut self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    pub fn get_current_and_advance(&mut self) -> Option<&Token> {
        self.advance();
        self.previous()
    }

    //Todo: Set the error inside this.
    pub fn consume(&mut self, tt: TokenType) -> bool {
        if tt == self.current_token().unwrap().token_type {
            self.advance();
            return true;
        }
        false
    }

    pub fn consume_advance_return(&mut self, tt: TokenType) -> Result<&Token, ParserError> {
        //TODO: IMpl eq for TT
        //This functions is used to check that the token has some specific type, the
        //internal lexeme for the token is ingored. Therefore, there is no check for equal value inside
        //token types (And is necessary this weird looking function istead of the built in == )
        let curr = self.current_token().unwrap().token_type.clone();

        if tt.week_comparison(&curr) {
            self.advance();
            let curr = Ok(self.previous().unwrap());
            return curr;
        }

        Err(ParserError::UnexpectedToken(format!(
            "Expected {:?}, got {:?}",
            tt,
            self.current_token().unwrap()
        )))
    }

    pub fn is_at_end(&self) -> bool {
        //If there is no more tokens or the current is EOF will end
        if let Some(tkn) = self.current_token() {
            if tkn.token_type == TokenType::EOF {
                return true;
            }
        }

        let current_token = self.tokens.get(self.current + 1);
        match current_token {
            None => return true,
            Some(_) => return false,
        }
    }

    //I cannot stand outside the tokens, because the idea is to move using consume,
    //and advance. Both of these methods do a is_at_end checking before any possible
    //movement
    pub fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    pub fn advance(&mut self) -> Option<&Token> {
        self.current += 1;
        self.tokens.get(self.current)
    }
}

//Stmt parsing
impl<'a> Parser<'a> {
    pub fn declaration(&mut self) -> StmtParserResult {
        let curr_tkn = self.current_token();
        if curr_tkn.is_none() {
            return Err(ParserError::UnexpectedToken(String::from(
                "Non expected EOF",
            )));
        }

        match curr_tkn.unwrap().token_type {
            TokenType::VAR => self.var_declaration(),
            _ => self.parse_stmt(),
        }
    }

    pub fn parse_stmt(&mut self) -> StmtParserResult {
        let curr_tkn = self.current_token();
        if curr_tkn.is_none() {
            return Err(ParserError::UnexpectedToken(String::from(
                "Non expected EOF",
            )));
        }

        match curr_tkn.unwrap().token_type {
            TokenType::PRINT => self.print_stmt(),
            TokenType::LEFTBRACE => self.block_stmt(),
            TokenType::IF => self.if_stmt(),
            _ => self.expr_stmt(),
        }
    }

    fn block_stmt(&mut self) -> StmtParserResult {
        //Consume the starting {
        self.advance();
        let mut internal_stmts: Vec<Box<Stmt>> = Vec::new();

        while !self.consume(TokenType::RIGHTBRACE) && !self.is_at_end() {
            let x = self.declaration()?;
            internal_stmts.push(Box::new(x));
        }

        //  The compiler complains about this code. It insists that is unreachable
        //  im not sure about that, but im not that smart either

        // if !self.consume(TokenType::RIGHTBRACE) {
        //     return return Err(ParserError::UnexpectedToken(format!(
        //         "Expected RIGHTBRACE, got {:?}",
        //         self.current_token().unwrap()
        //     )));

        Ok(Stmt::BLOCK(internal_stmts))
    }

    fn print_stmt(&mut self) -> StmtParserResult {
        self.advance();
        let expr = self.expr_rule()?;
        if !self.consume(TokenType::SEMICOLON) {
            return Err(ParserError::UnexpectedToken(format!(
                "Expected SEMICOLON, got {:?}",
                self.current_token().unwrap()
            )));
        }
        Ok(Stmt::PRINT(Box::new(expr)))
    }

    //exprStmt       → expression ";" ;
    fn expr_stmt(&mut self) -> StmtParserResult {
        let expr = self.expr_rule()?;
        if !self.consume(TokenType::SEMICOLON) {
            return Err(ParserError::UnexpectedToken(format!(
                "Expected SEMICOLON (;), got {:?}",
                self.current_token().unwrap()
            )));
        }
        Ok(Stmt::EXPR(Box::new(expr)))
    }

    fn var_declaration(&mut self) -> StmtParserResult {
        // varDecl        → "var" IDENTIFIER ( "="      expression )? ";" ;
        //                    |         |       |            |         |
        //                Start here -> Jump (-> Check  ->  Build)    MUST BE
        //                                   |-----Optional------|
        if !self.consume(TokenType::VAR) {
            return Err(ParserError::UnexpectedToken(format!(
                "Expected VAR, got {:?}",
                self.current_token().unwrap().token_type
            )));
        }
        //There must be a name

        //# This clone is ugly but works
        let name = self
            .consume_advance_return(TokenType::IDENTIFIER("".to_string()))
            .cloned();

        let mut initializer: Option<Box<Expr>> = None;
        if self.consume(TokenType::EQUAL) {
            initializer = Some(Box::new(self.expr_rule().unwrap()));
        }

        if !self.consume(TokenType::SEMICOLON) {
            return Err(ParserError::UnexpectedToken(format!(
                "Expected SEMICOLON, got {:?}",
                self.current_token().unwrap().token_type
            )));
        }
        Ok(Stmt::VAR(Box::new(name.unwrap()), initializer))
    }

    fn if_stmt(&mut self) -> StmtParserResult {
        // "if"        "(" expr ")" block
        //   |          | Handle |  Handle
        //Start here   Consume both

        //Jump the if
        self.advance();
        //Be sure that there is a (
        self.consume_advance_return(TokenType::LEFTPAREN)?;
        //Handle the boolean condition
        let condition = self.expr_rule()?;
        //Be sure that there is a )
        self.consume_advance_return(TokenType::RIGHTPAREN)?;
        //Be sure that there is a {
        let curr = self.current_token().unwrap();
        if curr.token_type != TokenType::LEFTBRACE {
            return Err(ParserError::UnexpectedToken(format!(
                "Expected {:?}, got {:?}",
                curr.token_type, curr
            )));
        }
        //The block will handle the closing }
        let main_block = self.block_stmt()?;
        let mut else_block: Option<Box<Stmt>> = None;
        if self.consume(TokenType::ELSE) {
            else_block = Some(Box::new(self.block_stmt()?));
        }

        Ok(Stmt::IF(
            Box::new(condition),
            Box::new(main_block),
            else_block,
        ))
    }
}

//Exprs parsing
impl<'a> Parser<'a> {
    // expression     → assignment ;
    pub fn expr_rule(&mut self) -> ExprParserResult {
        if self.is_at_end() {
            return Ok(Expr::Nil);
        }
        self.assignment_rule()
    }

    //assignment     → IDENTIFIER "=" assignment
    //               | equality ;
    pub fn assignment_rule(&mut self) -> ExprParserResult {
        //This can be a equality_expr or an identifier result
        let e = self.equality_rule()?;

        let curr = self.current_token();

        if curr.is_none() {
            return Err(ParserError::UnexpectedToken(String::from("Unexpected EOF")));
        }

        match curr.unwrap().token_type {
            TokenType::EQUAL => {
                self.advance();
                let assigment_value = self.assignment_rule()?;
                match e {
                    Expr::Variable(name) => {
                        return Ok(Expr::Assignment(name, Box::new(assigment_value)))
                    }
                    _ => return Err(ParserError::NonValidAssigmentTarget),
                }
            }
            _ => return Ok(e),
        }
    }
    //equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    //                              '-------------' -> Match
    //                            '-----------------------------'-> Loop
    pub fn equality_rule(&mut self) -> ExprParserResult {
        let left = self.comparison_rule()?;
        let curr_tkn = self.current_token();
        if curr_tkn.is_none() {
            return Err(ParserError::UnexpectedToken(String::from("Unexpected EOF")));
        }

        match curr_tkn.unwrap().token_type {
            TokenType::BANGEQUAL | TokenType::EQUALEQUAL => {
                let operator = curr_tkn.unwrap().clone();
                self.advance();
                let right = self.comparison_rule()?;
                return Ok(Expr::Binary(
                    Box::new(left),
                    Box::new(operator.clone()),
                    Box::new(right),
                ));
            }
            _ => {
                return Ok(left);
            }
        }
    }

    //comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    //                        '-------------------------' -> Match
    //                      '----------------------------------' -> While || Undf
    pub fn comparison_rule(&mut self) -> ExprParserResult {
        let left = self.term_rule()?;

        let curr_tkn = self.current_token();
        if curr_tkn.is_none() {
            return Err(ParserError::UnexpectedToken(String::from("Unexpected EOF")));
        }

        match curr_tkn.unwrap().token_type {
            TokenType::GREATER
            | TokenType::GREATEREQUAL
            | TokenType::LESS
            | TokenType::LESSEQUAL => {
                let operator = curr_tkn.unwrap().clone();
                self.advance();
                let right = self.term_rule()?;
                return Ok(Expr::Binary(
                    Box::new(left),
                    Box::new(operator.clone()),
                    Box::new(right),
                ));
            }
            _ => {
                return Ok(left);
            }
        }
    }

    //     term           → factor ( ( "-" | "+" ) factor )* ;
    //                               '-----------' -> Match
    //                             '----------------------' -> While || Undf
    pub fn term_rule(&mut self) -> ExprParserResult {
        let left = self.factor_rule()?;

        let curr_tkn = self.current_token();
        if curr_tkn.is_none() {
            return Err(ParserError::UnexpectedToken(String::from("Unexpected EOF")));
        }

        match curr_tkn.unwrap().token_type {
            TokenType::MINUS | TokenType::PLUS => {
                let operator = curr_tkn.unwrap().clone();
                self.advance();
                let right = self.factor_rule()?;
                return Ok(Expr::Binary(
                    Box::new(left),
                    Box::new(operator.clone()),
                    Box::new(right),
                ));
            }
            _ => return Ok(left),
        }
    }

    //		factor         → unary ( ( "/" | "*" ) unary )* ;
    //                               '-----------' -> Match
    //                             '---------------------' -> While || Undf
    pub fn factor_rule(&mut self) -> ExprParserResult {
        let left = self.unary_rule()?;

        let curr_tkn = self.current_token();
        if curr_tkn.is_none() {
            return Err(ParserError::UnexpectedToken(String::from("Unexpected EOF")));
        }

        match curr_tkn.unwrap().token_type {
            TokenType::SLASH | TokenType::STAR => {
                let operator = curr_tkn.unwrap().clone();
                self.advance();
                let right = self.unary_rule()?;
                return Ok(Expr::Binary(
                    Box::new(left),
                    Box::new(operator.clone()),
                    Box::new(right),
                ));
            }
            _ => return Ok(left),
        }
    }

    //		unary          → ( "!" | "-" ) unary    -
    //                       '-----------' -> Match  | -> Match entre ambos
    // 		               | primary ;              -
    pub fn unary_rule(&mut self) -> ExprParserResult {
        let curr_tkn = self.current_token();
        if let None = curr_tkn {
            return Err(ParserError::UnexpectedToken(String::from("Unexpected EOF")));
        }
        match curr_tkn.unwrap().token_type {
            TokenType::BANG | TokenType::MINUS => {
                let operator = curr_tkn.unwrap().clone();
                self.advance();
                let u = self.unary_rule()?;

                return Ok(Expr::Unary(Box::new(operator), Box::new(u)));
            }
            _ => {
                return self.primary_rule();
            }
        }
    }
    //primary        → NUMBER | STRING | "true" | "false" | "nil"
    //                | "(" expression ")" ;
    pub fn primary_rule(&mut self) -> Result<Expr, ParserError> {
        let curr_tkn = self.get_current_and_advance();
        if let None = curr_tkn {
            return Err(ParserError::UnexpectedToken(String::from("Unexpected EOF")));
        }
        let expr: Expr;
        match &curr_tkn.unwrap().token_type {
            TokenType::TRUE => expr = Expr::Boolean(true),
            TokenType::FALSE => expr = Expr::Boolean(false),
            TokenType::NUMBER(n) => return Ok(Expr::NumberLit(*n)),
            //This clone is not the best, because a new string is being created, but i dunno how
            //to handle the borrow checker correctly
            TokenType::STRING(s) => expr = Expr::StringLit(Box::new(s.clone())),
            TokenType::IDENTIFIER(s) => expr = Expr::Variable(Box::new(s.clone())),
            TokenType::LEFTPAREN => {
                //todo:Make it more rusty
                let internal_expr: Expr = self.expr_rule()?;
                if !self.consume(TokenType::RIGHTPAREN) {
                    return Err(ParserError::UnexpectedToken(format!(
                        "Expected RIGHTPAREN (')'), got {:?}",
                        self.current_token().unwrap()
                    )));
                }
                expr = Expr::Grouping(Box::new(internal_expr));
                //Return ther expr
            }
            TokenType::EOF => expr = self.expr_rule()?,
            _ => {
                return Err(ParserError::UnexpectedToken(format!(
                    "Got token: {:?}",
                    self.current_token()
                )));
            }
        }
        Ok(expr)
    }
}

#[cfg(test)]
mod parser_tests {

    use crate::{
        ast::{
            parser::Parser,
            printer::AstPrinter,
            scanner::Scanner,
            token::{self, Token},
        },
        lox::Lox,
    };
    use std::vec;

    #[test]
    fn parse_if_stmt() {
        //Im already sure that the scanner works well
        let src = String::from("if(y==4){ var x = 3;}else{var y = 4;}");
        let mut lox = Lox::default();
        let mut scanner = Scanner::new(&mut lox, &src);
        let tokens = scanner.scan_tokens();
        let expr = Parser::new(tokens).parse().unwrap();
        println!("{:?}", expr);
        // let printed: String = AstPrinter::default().print_program(&expr);
        // println!("{:?}", printed);
    }

    #[test]
    fn parse_literal() {
        let tkn = Token::new(token::TokenType::NUMBER(32.0), 32.0.to_string(), 1);

        let semicolon_tkn = Token::new(token::TokenType::SEMICOLON, ";".to_string(), 1);
        let eof_tkn = Token::new(token::TokenType::EOF, "eof".to_string(), 1);

        let expr = Parser::new(&vec![tkn, semicolon_tkn, eof_tkn])
            .parse()
            .unwrap();
        let printed: String = AstPrinter::default().print_program(&expr);
        assert_eq!(printed, "32");
    }

    #[test]
    fn parse_assignment() {
        let idnt_tkn = Token::new(
            token::TokenType::IDENTIFIER("x".to_string()),
            "x".to_string(),
            1,
        );

        let equal_tkn = Token::new(token::TokenType::EQUAL, "=".to_string(), 1);
        let expr = Token::new(token::TokenType::NUMBER(3.0), "3.0".to_string(), 1);

        let semicolon_tkn = Token::new(token::TokenType::SEMICOLON, ";".to_string(), 1);
        let eof_tkn = Token::new(token::TokenType::EOF, "eof".to_string(), 1);

        let expr = Parser::new(&vec![idnt_tkn, equal_tkn, expr, semicolon_tkn, eof_tkn])
            .assignment_rule()
            .unwrap();
        let printed = AstPrinter::default().print_expr(&expr);
        println!("Printed: {printed}");
    }

    #[test]
    fn parse_string() {
        let str = String::from("Im a simple string");
        let tkn = Token::new(token::TokenType::STRING(str.clone()), str, 1);

        let semicolon_tkn = Token::new(token::TokenType::SEMICOLON, ";".to_string(), 1);
        let eof_tkn = Token::new(token::TokenType::EOF, "eof".to_string(), 1);

        let expr = Parser::new(&vec![tkn, semicolon_tkn, eof_tkn])
            .parse()
            .unwrap();
        let printed = AstPrinter::default().print_program(&expr);
        assert_eq!(printed, "Im a simple string")
    }

    #[test]
    fn parse_boolean() {
        let bool_tkn = Token::new(token::TokenType::FALSE, String::from("false"), 1);

        let semicolon_tkn = Token::new(token::TokenType::SEMICOLON, ";".to_string(), 1);
        let eof_tkn = Token::new(token::TokenType::EOF, "eof".to_string(), 1);

        let expr = Parser::new(&vec![bool_tkn, semicolon_tkn, eof_tkn])
            .parse()
            .unwrap();
        let printed = AstPrinter::default().print_program(&expr);
        assert_eq!(printed, "false");
    }

    #[test]
    fn parse_equality_literal() {
        //2 == 3 -> (== 2 3)
        let two_tkn = Token::new(token::TokenType::NUMBER(2.0), 2.0.to_string(), 1);
        let eqeq_tkn = Token::new(token::TokenType::EQUALEQUAL, "==".to_string(), 1);
        let three_tkn = Token::new(token::TokenType::NUMBER(3.0), 3.0.to_string(), 1);

        let semicolon_tkn = Token::new(token::TokenType::SEMICOLON, ";".to_string(), 1);
        let eof_tkn = Token::new(token::TokenType::EOF, "eof".to_string(), 1);
        let expr = Parser::new(&vec![two_tkn, eqeq_tkn, three_tkn, semicolon_tkn, eof_tkn]).parse();
        // .unwrap();
        println!("{:?}", expr);
        // let printed = AstPrinter::default().print_program(&expr);
        // println!("{:?}", printed);
        // assert_eq!(printed, "(== 2 3)")
    }

    #[test]
    fn parse_comparison_literal() {
        //2 > 3 -> (> 2 3)
        let two_tkn = Token::new(token::TokenType::NUMBER(2.0), 2.0.to_string(), 1);
        let grtr_tkn = Token::new(token::TokenType::GREATER, ">".to_string(), 1);
        let three_tkn = Token::new(token::TokenType::NUMBER(3.0), 3.0.to_string(), 1);

        let semicolon_tkn = Token::new(token::TokenType::SEMICOLON, ";".to_string(), 1);
        let eof_tkn = Token::new(token::TokenType::EOF, "eof".to_string(), 1);

        let expr = Parser::new(&vec![two_tkn, grtr_tkn, three_tkn, semicolon_tkn, eof_tkn])
            .parse()
            .unwrap();

        let printed = AstPrinter::default().print_program(&expr);
        assert_eq!(printed, "(> 2 3)")
    }

    #[test]
    fn parse_term_literal() {
        //2 + 3 -> (+ 2 3)
        let two_tkn = Token::new(token::TokenType::NUMBER(2.0), 2.0.to_string(), 1);
        let plus_tkn = Token::new(token::TokenType::PLUS, "+".to_string(), 1);
        let three_tkn = Token::new(token::TokenType::NUMBER(3.0), 3.0.to_string(), 1);

        let semicolon_tkn = Token::new(token::TokenType::SEMICOLON, ";".to_string(), 1);
        let eof_tkn = Token::new(token::TokenType::EOF, "eof".to_string(), 1);
        let expr = Parser::new(&vec![two_tkn, plus_tkn, three_tkn, semicolon_tkn, eof_tkn])
            .parse()
            .unwrap();

        let printed = AstPrinter::default().print_program(&expr);
        assert_eq!(printed, "(+ 2 3)")
    }

    #[test]
    fn parse_factor_literal() {
        //3 * 4 -> (* 3 4)
        let three_tkn = Token::new(token::TokenType::NUMBER(3.0), 3.0.to_string(), 1);
        let star_tkn = Token::new(token::TokenType::STAR, "*".to_string(), 1);
        let four_tkn = Token::new(token::TokenType::NUMBER(4.0), 4.0.to_string(), 1);

        let semicolon_tkn = Token::new(token::TokenType::SEMICOLON, ";".to_string(), 1);
        let eof_tkn = Token::new(token::TokenType::EOF, "eof".to_string(), 1);
        let expr = Parser::new(&vec![three_tkn, star_tkn, four_tkn, semicolon_tkn, eof_tkn])
            .parse()
            .unwrap();

        let printed = AstPrinter::default().print_program(&expr);
        assert_eq!(printed, "(* 3 4)")
    }

    #[test]
    fn parse_unary() {
        let bang_tkn = Token::new(token::TokenType::BANG, "!".to_string(), 1);
        let number_tkn = Token::new(token::TokenType::NUMBER(32.0), 32.0.to_string(), 1);

        let semicolon_tkn = Token::new(token::TokenType::SEMICOLON, ";".to_string(), 1);
        let eof_tkn = Token::new(token::TokenType::EOF, "eof".to_string(), 1);
        let expr = Parser::new(&vec![bang_tkn, number_tkn, semicolon_tkn, eof_tkn])
            .parse()
            .unwrap();
        let printed = AstPrinter::default().print_program(&expr);
        println!("Printed: {:?}", printed);
        assert_eq!(printed, "(! 32)");
    }

    #[test]
    fn parse_grouped_number_literal() {
        // Parse -> (32.0)
        let left_paren = Token::new(token::TokenType::LEFTPAREN, "(".to_string(), 1);
        let right_paren = Token::new(token::TokenType::RIGHTPAREN, ")".to_string(), 1);
        let number_tkn = Token::new(token::TokenType::NUMBER(32.0), 32.0.to_string(), 1);

        let semicolon_tkn = Token::new(token::TokenType::SEMICOLON, ";".to_string(), 1);
        let eof_tkn = Token::new(token::TokenType::EOF, "eof".to_string(), 1);
        let expr = Parser::new(&vec![
            left_paren,
            number_tkn,
            right_paren,
            semicolon_tkn,
            eof_tkn,
        ])
        .parse()
        .unwrap();

        let printed = AstPrinter::default().print_program(&expr);

        assert_eq!(printed, "(group 32)");
    }

    #[test]
    fn test_end_eof() {
        let eof_tkn = Token::new(token::TokenType::EOF, "eof".to_string(), 1);
        let is = Parser::new(&vec![eof_tkn]).is_at_end();
        assert!(is)
    }
    #[test]
    fn test_end_non_eof() {
        let number_tkn = Token::new(token::TokenType::NUMBER(32.0), 32.0.to_string(), 1);

        let semicolon_tkn = Token::new(token::TokenType::SEMICOLON, ";".to_string(), 1);
        let eof_tkn = Token::new(token::TokenType::EOF, "eof".to_string(), 1);
        let is_eof = Parser::new(&vec![number_tkn, semicolon_tkn, eof_tkn]).is_at_end();
        assert!(!is_eof)
    }
}
