use std::{
    fs::File,
    io::{self, BufReader, Read, Write},
};

use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;
use token::Token;
use token_type::TokenType;

mod environment;
mod generate_ast;
mod interpreter;
mod parser;
mod scanner;
mod token;
mod token_type;

pub struct Lox {
    had_error: bool,
    interpreter: Interpreter,
}

impl Lox {
    pub fn new() -> Self {
        Self {
            had_error: false,
            interpreter: Interpreter::new(),
        }
    }

    pub fn run_file(&mut self, file_name: String) {
        let file = File::open(file_name).expect("open file");
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer).expect("read file");
        self.run(&buffer);
    }

    pub fn run_prompt(&mut self) {
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

        let mut parser = Parser::new(tokens.iter().flatten().collect());
        let stmts = parser.parse();
        match stmts {
            Ok(stmts) => match self.interpreter.interpret(stmts) {
                Ok(_) => (),
                Err(err) => self.error_in_interpret(err),
            },
            Err(errors) => {
                for err in errors {
                    self.error_in_parse(&err);
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

    fn error_in_parse(&mut self, parse_err: &LoxParseError) {
        if parse_err.0.token_type == TokenType::Eof {
            self.report(parse_err.0.line, "at end", &parse_err.1);
        } else {
            self.report(
                parse_err.0.line,
                &format!("at '{}'", &parse_err.0.lexeme),
                &parse_err.1,
            );
        }
    }

    fn error_in_interpret(&mut self, runtime_err: LoxRuntimeError) {
        eprintln!("{}", runtime_err.1);
        eprintln!("[line {}]", runtime_err.0.line);
    }
}

impl Default for Lox {
    fn default() -> Self {
        Self::new()
    }
}

pub struct LoxScanError(usize, String);
#[derive(Debug)]
pub struct LoxParseError(Token, String);

pub struct LoxRuntimeError(Token, String);
