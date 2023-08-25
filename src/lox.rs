use std::{fs, path::Path};

use crate::{
    ast::{parser::Parser, scanner::Scanner},
    interpreter::Interpreter,
};

pub struct Lox {
    had_error: bool,
}

impl Default for Lox {
    fn default() -> Self {
        Self { had_error: false }
    }
}

impl Lox {
    pub fn run_file(&mut self, string_path: &String) {
        let path = Path::new(string_path);
        if !path.exists() {
            println!("Error: There is no {} to interpretate", string_path);
            return;
        }
        let content = fs::read_to_string(string_path).unwrap();
        self.run(content);
    }

    //Todo: Implement prompt. At the moment im not sure
    // if i want to run command line by line or support
    // multiples lines and enter an specific command to run
    pub fn run_prompt(&self) {
        println!("> ");
        todo!();
    }

    pub fn run(&mut self, content: String) {
        println!("Content: {content}");

        //Run scanner
        let mut scanner = Scanner::new(self, &content);
        let tokens = scanner.scan_tokens();
        //Run parser
        let mut parser = Parser::new(tokens);
        let exprs = parser.parse();
        if let Err(e) = exprs {
            panic!("{:?}", e);
        }

        // let _ast_str = AstPrinter::default().print(&exprs.unwrap());
        // println!("AST -> {ast_str}");

        //Run the code
        Interpreter::default().interpret(&exprs.unwrap());
    }

    pub fn error(&mut self, line: u8, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: u8, place: &str, message: &str) {
        println!("[line {line}] Error {place} : {message}");
        self.had_error = true;
    }
}
