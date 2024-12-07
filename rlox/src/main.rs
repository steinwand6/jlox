use std::env::args;

use rlox::Lox;

fn main() {
    match args().len() {
        2 => {
            let mut lox = Lox::new();
            lox.run_file(args().nth(1).unwrap());
        }
        1 => {
            let mut lox = Lox::new();
            lox.run_prompt();
        }
        _ => {
            println!("Usage: rlox [script]");
        }
    }
}
