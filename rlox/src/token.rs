use std::fmt::Display;

use crate::token_type::TokenType;

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Object,
    pub line: usize,
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum Object {
    String(String),
    Num(f64),
    Bool(bool),
    None,
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
}
