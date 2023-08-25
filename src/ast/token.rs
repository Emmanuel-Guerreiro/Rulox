use std::fmt::Debug;
#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
#[warn(non_camel_case_types)]
pub enum TokenType {
    LEFTPAREN, // (
    RIGHTPAREN,
    LEFTBRACE,
    RIGHTBRACE,
    COMMA,
    DOT,
    MINUS, //Sub
    PLUS,  //Sum
    SEMICOLON,
    SLASH, //Division
    STAR,  //Product

    // One or two character tokens.
    BANG, //Negation
    BANGEQUAL,
    EQUAL,
    EQUALEQUAL,
    GREATER,
    GREATEREQUAL,
    LESS,
    LESSEQUAL,

    // Literals.
    IDENTIFIER(String), //var x =  This Is The Name Of A Variable
    STRING(String),     //"hola"
    NUMBER(f64),

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

//Literal is the current value of the token.
//If there is some value to acces, it will be done through token_type
//Rust is beautiful
#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub token_type: TokenType, //The type + the value in some cases
    pub lexeme: String,        //The substring for the token
    pub line: u8,              //Line where the token appears
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: u8) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        String::from(format!(
            "{:?} - {} - {}",
            self.token_type, self.lexeme, self.line
        ))
    }
}
