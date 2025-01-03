use std::fmt::Display;

use crate::{environment::Environment, generate_ast::FunctionStmt, token_type::TokenType};

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Object,
    pub line: usize,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Object {
    String(String),
    Num(f64),
    Bool(bool),
    Fun(Box<FunctionStmt>, Environment),
    None,
}

impl PartialEq for FunctionStmt {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.params == other.params
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Object, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Object::String(s) => s.to_string(),
            Object::Num(n) => n.to_string(),
            Object::Bool(b) => b.to_string(),
            Object::Fun(stmt, _) => stmt.name.to_string(),
            Object::None => "[None]".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl Object {
    pub fn num(&self) -> Result<f64, ()> {
        match self {
            Object::Num(n) => Ok(*n),
            _ => Err(()),
        }
    }

    pub fn str(&self) -> Result<String, ()> {
        match self {
            Object::String(str) => Ok(str.into()),
            _ => Err(()),
        }
    }

    pub fn arity(&self) -> Result<usize, ()> {
        match self {
            Object::Fun(stmt, _) => Ok(stmt.params.len()),
            _ => Err(()),
        }
    }

    pub fn get_closure(&mut self) -> Result<&mut Environment, ()> {
        match self {
            Object::Fun(_, env) => Ok(env),
            _ => Err(()),
        }
    }
}
