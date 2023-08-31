use crate::lox::Lox;

use std::iter::Peekable;
use std::str::Chars;

use super::token::{Token, TokenType};

//The source (The actual code) string is owned by
//the scanner because after the execution of the
//scanning, there is no reason to keep the code in memory
//If there is some reason to use it in some posterior excecution,
//Borrowing and lifetimes are the go to option
pub struct Scanner<'a> {
    source: Peekable<Chars<'a>>,
    start: u8,
    line: u8,
    current: u8,
    tokens: Vec<Token>,
    lox: &'a mut Lox,
}

impl<'a> Scanner<'a> {
    pub fn new(lox: &'a mut Lox, src: &'a String) -> Self {
        Self {
            source: src.chars().peekable(),
            start: 0,
            line: 0,
            current: 0,
            tokens: Vec::new(),
            lox,
        }
    }

    //The vector of tokens will be borrowed to the parser after the Scanner excecution
    //Just before the parsing it can be sweeped
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            line: self.line,
        });
        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token_from_char(TokenType::LEFTPAREN, c),
            ')' => self.add_token_from_char(TokenType::RIGHTPAREN, c),
            '{' => self.add_token_from_char(TokenType::LEFTBRACE, c),
            '}' => self.add_token_from_char(TokenType::RIGHTBRACE, c),
            ',' => self.add_token_from_char(TokenType::COMMA, c),
            '.' => self.add_token_from_char(TokenType::DOT, c),
            '-' => self.add_token_from_char(TokenType::MINUS, c),
            '+' => self.add_token_from_char(TokenType::PLUS, c),
            ';' => self.add_token_from_char(TokenType::SEMICOLON, c),
            '*' => self.add_token_from_char(TokenType::STAR, c),
            '!' => {
                let mut tt = TokenType::BANG;
                let mut lx = c.clone().to_string();
                if self.match_next('=') {
                    tt = TokenType::BANGEQUAL;
                    lx += "=";
                }
                self.add_token(tt, lx);
            }
            '=' => {
                let mut tt = TokenType::EQUAL;
                let mut lx: String = c.clone().to_string();
                if self.match_next('=') {
                    tt = TokenType::EQUALEQUAL;
                    lx += "=";
                }
                self.add_token(tt, lx);
            }
            '<' => {
                let mut tt = TokenType::LESS;
                let mut lx = c.clone().to_string();
                if self.match_next('=') {
                    tt = TokenType::LESSEQUAL;
                    lx += "=";
                }
                self.add_token(tt, lx);
            }
            '>' => {
                let mut tt = TokenType::GREATER;
                let mut lx = c.clone().to_string();
                if self.match_next('=') {
                    tt = TokenType::GREATEREQUAL;
                    lx += "=";
                }
                self.add_token(tt, lx);
            }
            '/' => {
                //Todo: Make it more rust idiomatic
                //Some match statement should work
                if self.match_next('/') {
                    self.handle_comment();
                } else if self.match_next('*') {
                    self.handle_multiline_comment();
                } else {
                    self.add_token_from_char(TokenType::SLASH, c);
                }
            }
            ' ' | '\t' | '\r' => {}
            '\n' => self.line += 1,
            '"' => self.handle_strings(),
            _ => {
                if c.is_digit(10) {
                    self.handle_numbers(c);
                } else if c.is_alphabetic() || c == '_' {
                    //This will match any alphabetic unicode character (that may include
                    //non latin letters)
                    //This is not correct, but is easier
                    //todo: Implement a correct latin character matcher
                    self.handle_identifier(c);
                } else {
                    self.lox.error(self.line, "Unexpected character");
                }
            }
        }
    }

    fn add_token(&mut self, tt: TokenType, lexeme: String) {
        self.tokens.push(Token::new(tt, lexeme, self.line));
    }

    fn add_token_from_char(&mut self, tt: TokenType, c: char) {
        self.tokens.push(Token::new(tt, c.to_string(), self.line));
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() || *self.source.peek().unwrap() == expected {
            self.advance();
            return true;
        } else {
            return false;
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.next().unwrap()
    }

    fn is_at_end(&mut self) -> bool {
        self.source.peek().is_none()
    }

    fn handle_comment(&mut self) {
        //Comment will span until the end of line
        while self.source.peek() != Some(&'\n') && !self.is_at_end() {
            self.advance();
        }
    }

    fn handle_multiline_comment(&mut self) {
        //todo: Something arround here is broken
        let next_token = self.source.peek().unwrap().clone();

        while next_token != '*' {
            if next_token == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        let next_next_token = self.source.peek().unwrap().clone();
        while next_next_token != '/' {
            if next_token == '\n' {
                self.line += 1;
            }
            self.advance();
        }
    }

    fn handle_strings(&mut self) {
        let mut cleaned_string = String::from("");

        'find_string: while let Some(c) = self.source.peek() {
            match c {
                '"' => break 'find_string,
                '\n' => {
                    self.line += 1;
                }
                _ => cleaned_string += &c.clone().to_string(), // Just ignore the chars until the end of string
            }
            self.advance();
        }

        if self.is_at_end() {
            return;
        }
        //Jump the "
        self.advance();

        //Clonning the value isnt very performant, but simplifies
        //all the lifetimes manage inside TokenType
        self.add_token(TokenType::STRING(cleaned_string.clone()), cleaned_string);
    }

    fn handle_numbers(&mut self, first_number: char) {
        let mut number = String::from(first_number);
        let int = self.consume_while(|x| x.is_numeric());
        number += &int.to_string();
        //If the int part encounters a non numeric char will end
        //If the value is ".", that means there is a float number.
        //The process must be repeated until the number ends
        //IF there is a new "." is to call a method of the object (as in rust)
        //If there is a point, position over that one
        if self.source.peek() == Some(&'.') {
            self.advance();
            if self.source.peek().is_some() && self.source.peek().unwrap().is_numeric() {
                let decimal = self.consume_while(|x| x.is_numeric());
                number += ".";
                number += &decimal.to_string();
            }
        }

        self.add_token(TokenType::NUMBER(number.parse::<f64>().unwrap()), number);
    }

    //Will consume
    fn consume_while<F>(&mut self, f: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut colected = String::from("");
        while let Some(c) = self.source.peek() {
            if f(*c) {
                colected += &self.advance().clone().to_string();
            } else {
                break;
            }
        }
        colected
    }

    //Will read the whole identifier and store the lexeme to find the corresponding
    //keyword token
    fn handle_identifier(&mut self, first_letter: char) {
        let mut kwlexeme = String::from(first_letter);
        loop {
            let c = self.source.peek();
            if c.is_none() || !c.unwrap().is_alphanumeric() {
                break;
            }
            kwlexeme += &c.unwrap().clone().to_string();

            self.advance();
        }

        match self.keyword(&kwlexeme) {
            None => self.add_token(TokenType::IDENTIFIER(kwlexeme), String::from("")),
            Some(tt) => self.add_token(tt, kwlexeme),
        }
    }

    //Todo: There may be a more idiomatic way
    fn keyword(&self, identifier: &str) -> Option<TokenType> {
        use std::collections::HashMap;
        let mut keywords: HashMap<&str, TokenType> = HashMap::new();
        keywords.insert("and", TokenType::AND);
        keywords.insert("class", TokenType::CLASS);
        keywords.insert("else", TokenType::ELSE);
        keywords.insert("false", TokenType::FALSE);
        keywords.insert("fun", TokenType::FUN);
        keywords.insert("for", TokenType::FOR);
        keywords.insert("if", TokenType::IF);
        keywords.insert("nil", TokenType::NIL);
        keywords.insert("or", TokenType::OR);
        keywords.insert("print", TokenType::PRINT);
        keywords.insert("return", TokenType::RETURN);
        keywords.insert("super", TokenType::SUPER);
        keywords.insert("this", TokenType::THIS);
        keywords.insert("true", TokenType::TRUE);
        keywords.insert("var", TokenType::VAR);
        keywords.insert("while", TokenType::WHILE);
        match keywords.get(identifier) {
            None => None,
            Some(tt) => Some(tt.clone()),
        }
    }
}

#[cfg(test)]

mod scanner_test {
    use crate::{
        ast::token::{Token, TokenType},
        lox::Lox,
    };

    use super::Scanner;
    #[test]
    fn boolean_last_tkn() {
        let src = String::from("true;");
        let mut lox = Lox::default();
        let mut scanner = Scanner::new(&mut lox, &src);
        let tokens = scanner.scan_tokens();
        let expected = &vec![
            Token {
                token_type: TokenType::TRUE,
                lexeme: "true".to_string(),
                line: 0,
            },
            Token {
                token_type: TokenType::SEMICOLON,
                lexeme: ";".to_string(),
                line: 0,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                line: 0,
            },
        ];
        assert_eq!(tokens, expected)
    }
    #[test]
    fn scan_number() {
        let src = String::from("3.2");
        let mut lox = Lox::default();
        let mut scanner = Scanner::new(&mut lox, &src);
        let tokens = scanner.scan_tokens();
        let expected = &vec![
            Token {
                token_type: TokenType::NUMBER(3.2),
                lexeme: "3.2".to_string(),
                line: 0,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                line: 0,
            },
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn scan_string() {
        let src = String::from("\"Im a simple string\"");
        let mut lox = Lox::default();
        let mut scanner = Scanner::new(&mut lox, &src);
        let tokens = scanner.scan_tokens();
        let expected = &vec![
            Token {
                token_type: TokenType::STRING("Im a simple string".to_string()),
                lexeme: "Im a simple string".to_string(),
                line: 0,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                line: 0,
            },
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn scan_comment() {
        let src = String::from("//Ingore comment");
        let mut lox = Lox::default();
        let mut scanner = Scanner::new(&mut lox, &src);
        let tokens = scanner.scan_tokens();
        println!("{:?}", tokens);
        let expected = &vec![Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            line: 0,
        }];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn scan_unary_expr() {
        let src = String::from("!3");
        let mut lox = Lox::default();
        let mut scanner = Scanner::new(&mut lox, &src);
        let tokens = scanner.scan_tokens();
        let expected = &vec![
            Token {
                token_type: TokenType::BANG,
                lexeme: "!".to_string(),
                line: 0,
            },
            Token {
                token_type: TokenType::NUMBER(3.0),
                lexeme: "3".to_string(),
                line: 0,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                line: 0,
            },
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn scan_bang_eq() {
        let src = String::from("3!=4");
        let mut lox = Lox::default();
        let mut scanner = Scanner::new(&mut lox, &src);
        let tokens = scanner.scan_tokens();
        let expected = &vec![
            Token {
                token_type: TokenType::NUMBER(3.0),
                lexeme: "3".to_string(),
                line: 0,
            },
            Token {
                token_type: TokenType::BANGEQUAL,
                lexeme: "!=".to_string(),
                line: 0,
            },
            Token {
                token_type: TokenType::NUMBER(4.0),
                lexeme: "4".to_string(),
                line: 0,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                line: 0,
            },
        ];
        assert_eq!(tokens, expected);
    }
    #[test]
    fn scan_grtr() {
        let src = String::from("3>4");
        let mut lox = Lox::default();
        let mut scanner = Scanner::new(&mut lox, &src);
        let tokens = scanner.scan_tokens();
        let expected = &vec![
            Token {
                token_type: TokenType::NUMBER(3.0),
                lexeme: "3".to_string(),
                line: 0,
            },
            Token {
                token_type: TokenType::GREATER,
                lexeme: ">".to_string(),
                line: 0,
            },
            Token {
                token_type: TokenType::NUMBER(4.0),
                lexeme: "4".to_string(),
                line: 0,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                line: 0,
            },
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn variable_declaration() {
        let src = String::from("var asd = 123;");
        let mut lox = Lox::default();
        let mut scanner = Scanner::new(&mut lox, &src);
        let tokens = scanner.scan_tokens();
        println!("{:?}", tokens)
    }
}
