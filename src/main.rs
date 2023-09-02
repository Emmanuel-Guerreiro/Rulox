mod ast;
mod enviroment;
pub mod interpreter;
mod lox;
mod object;
mod tests;

use crate::lox::Lox;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut l = Lox::default();

    match args.len() {
        2 => l.run_file(&args[1]),
        1 => l.run_prompt(),
        _ => {
            println!("Usage: lox [script]");
            std::process::exit(64)
        }
    }
}
