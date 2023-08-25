mod ast;
mod lox;
mod object;
mod tests;
use std::env;
pub mod interpreter;

use crate::lox::Lox;

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
