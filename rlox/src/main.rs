use std::env::args;

use rlox::Lox;

fn main() {
    if args().len() > 2 {
        println!("Usage: rlox [script]");
    } else if args().len() == 2 {
        let mut lox = Lox::new();
        lox.run_file(args().nth(1).unwrap());
    } else {
        let mut lox = Lox::new();
        lox.run_prompt();
    }
}
