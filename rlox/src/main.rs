use std::{
    env::args,
    fs::File,
    io::{self, BufReader, Read, Write},
};

use scanner::Scanner;

mod scanner;
mod token;
mod token_type;

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

struct Lox {
    had_error: bool,
}

impl Lox {
    fn new() -> Self {
        Self { had_error: false }
    }

    fn run_file(&mut self, file_name: String) {
        let file = File::open(file_name).expect("open file");
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer).expect("read file");
        self.run(&buffer);
    }

    fn run_prompt(&mut self) {
        let mut buffer = String::new();

        loop {
            buffer.clear();
            print!("> ");
            io::stdout().flush().expect("flush");
            io::stdin().read_line(&mut buffer).expect("read line");
            if buffer.is_empty() {
                return;
            }
            self.run(&buffer);

            self.had_error = false;
        }
    }

    fn run(&mut self, src: &str) {
        let mut scanner = Scanner::new(src);
        let tokens = scanner.scan_tokens();

        tokens
            .iter()
            .filter_map(|token| token.as_ref().err())
            .for_each(|err| self.error(err.0, &err.1));

        if !self.had_error {
            for token in tokens {
                match token {
                    Ok(token) => println!("{}", token),
                    Err(e) => eprintln!("{} {}", e.0, e.1),
                }
            }
        }
    }

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, place: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, place, message);
        self.had_error = true;
    }
}

pub struct LoxError(usize, String);
