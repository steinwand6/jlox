use crate::{
    token::{Object, Token},
    token_type::TokenType,
    LoxError,
};

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Result<Token, LoxError>>,
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

    pub fn scan_tokens(&mut self) -> &Vec<Result<Token, LoxError>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Ok(Token::new(
            TokenType::EOF,
            "".into(),
            Object::None,
            self.line,
        )));
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

            '!' => {
                if self.match_token('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_token('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_token('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_token('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.match_token('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\t' | '\r' => (),
            '\n' => self.line += 1,

            '"' => self.string(),

            _ => self.tokens.push(Err(LoxError(
                self.line,
                "Unexpected character.".to_string(),
            ))),
        };
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            self.advance();
        }
        if self.is_at_end() {
            self.tokens
                .push(Err(LoxError(self.line, "Unterminated string.".to_string())));
            return;
        }
        self.advance();
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_with_literal(TokenType::String, Object::STRING(value));
    }

    fn peek(&mut self) -> char {
        self.source
            .chars()
            .nth(self.current)
            .expect("peek in scanner")
    }

    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn advance(&mut self) -> char {
        let c = self
            .source
            .chars()
            .nth(self.current)
            .expect("advance in scanner");
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, Object::None);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Object) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Ok(Token::new(token_type, text, literal, self.line)));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
