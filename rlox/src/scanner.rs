use std::ops::Index;

use crate::{
    token::{Object, Token},
    token_type::TokenType,
};

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".into(),
            Object::None,
            self.line,
        ));
        return &self.tokens;
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            '*' => self.add_token(TokenType::Star),
            ';' => self.add_token(TokenType::SemiColon),
            _ => (),
        }
    }

    fn advance(&mut self) -> char {
        let c = self
            .source
            .chars()
            .nth(self.current)
            .expect("{} advance in scanner");
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, Object::None);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Object) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
