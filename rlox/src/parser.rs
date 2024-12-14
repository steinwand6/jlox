use crate::{
    generate_ast::{
        AssignExpr, BinaryExpr, BlockStmt, Expr, ExpressionStmt, GroupingExpr, LiteralExpr,
        PrintStmt, Stmt, UnaryExpr, VarStmt, VariableExpr,
    },
    token::{Object, Token},
    token_type::TokenType,
    LoxParseError,
};

pub struct Parser<'a> {
    tokens: Vec<&'a Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<&'a Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Vec<LoxParseError>> {
        let mut statements = vec![];
        let mut errors = vec![];
        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => {
                    errors.push(e);
                    self.synchronize();
                }
            }
        }
        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, LoxParseError> {
        if self.match_type(&[TokenType::Var]) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxParseError> {
        let name = self
            .consume(&TokenType::Identifier)
            .map_err(|t| LoxParseError(t, "Expect variable name.".into()))?;

        let mut initializer = Box::new(Expr::Literal(LiteralExpr::new(Object::None)));
        if self.match_type(&[TokenType::Equal]) {
            initializer = self.expression()?;
        }
        self.consume(&TokenType::SemiColon)
            .map_err(|t| LoxParseError(t, "Expect ';' after variable declaration.".into()))?;
        Ok(Stmt::Var(VarStmt::new(name, *initializer)))
    }

    fn statement(&mut self) -> Result<Stmt, LoxParseError> {
        if self.match_type(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.match_type(&[TokenType::LeftBrace]) {
            return Ok(Stmt::Block(BlockStmt::new(self.block_statement()?)));
        }
        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxParseError> {
        let value = self.expression()?;

        match self.consume(&TokenType::SemiColon) {
            Ok(_) => Ok(Stmt::Print(PrintStmt::new(*value))),
            Err(token) => Err(LoxParseError(token, "Expect ';' after value".into())),
        }
    }

    fn block_statement(&mut self) -> Result<Vec<Stmt>, LoxParseError> {
        let mut statements = vec![];
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        match self.consume(&TokenType::RightBrace) {
            Ok(_) => Ok(statements),
            Err(t) => Err(LoxParseError(t, "Expected '}' after block.".into())),
        }
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxParseError> {
        let expr = self.expression()?;
        match self.consume(&TokenType::SemiColon) {
            Ok(_) => Ok(Stmt::Expression(ExpressionStmt::new(*expr))),
            Err(token) => Err(LoxParseError(token, "Expect ';' after expression".into())),
        }
    }

    fn expression(&mut self) -> Result<Box<Expr>, LoxParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Box<Expr>, LoxParseError> {
        let expr = self.equality()?;

        if self.match_type(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            match *expr {
                Expr::Variable(var) => {
                    return Ok(Box::new(Expr::Assign(AssignExpr::new(var.name, value))));
                }
                _ => return Err(LoxParseError(equals, "Invalid assignment target.".into())),
            }
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Box<Expr>, LoxParseError> {
        let mut expr = self.comparison()?;
        while self.match_type(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Box::new(Expr::Binary(BinaryExpr::new(expr, operator, right)));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Box<Expr>, LoxParseError> {
        let mut expr = self.term()?;
        while self.match_type(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Box::new(Expr::Binary(BinaryExpr::new(expr, operator, right)));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<Expr>, LoxParseError> {
        let mut expr = self.factor()?;
        while self.match_type(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Box::new(Expr::Binary(BinaryExpr::new(expr, operator, right)));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Box<Expr>, LoxParseError> {
        let mut expr = self.unary()?;
        while self.match_type(&[TokenType::Star, TokenType::Slash]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Box::new(Expr::Binary(BinaryExpr::new(expr, operator, right)));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Box<Expr>, LoxParseError> {
        if self.match_type(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Box::new(Expr::Unary(UnaryExpr::new(operator, right))));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Box<Expr>, LoxParseError> {
        let literal = match self.peek().token_type {
            TokenType::False => LiteralExpr::new(Object::Bool(false)),
            TokenType::True => LiteralExpr::new(Object::Bool(true)),
            TokenType::Nil => LiteralExpr::new(Object::None),
            TokenType::Number => LiteralExpr::new(Object::Num(self.peek().literal.num().unwrap())),
            TokenType::String => {
                LiteralExpr::new(Object::String(self.peek().literal.str().unwrap()))
            }
            TokenType::LeftParen => {
                self.current += 1;
                let expr = self.expression()?;
                match self.consume(&TokenType::RightParen) {
                    Ok(_) => return Ok(Box::new(Expr::Grouping(GroupingExpr::new(expr)))),
                    Err(t) => return Err(LoxParseError(t, "Expecte ')' after expression.".into())),
                }
            }
            TokenType::Identifier => {
                self.current += 1;
                return Ok(Box::new(Expr::Variable(VariableExpr::new(self.previous()))));
            }
            _ => {
                return Err(LoxParseError(self.advance(), "Expect expression.".into()));
            }
        };
        self.current += 1;
        Ok(Box::new(Expr::Literal(literal)))
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn match_type(&mut self, types: &[TokenType]) -> bool {
        for expect in types {
            if self.check(expect) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: &TokenType) -> Result<Token, Token> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        Err(self.peek().clone())
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == *token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn previous(&self) -> Token {
        let t = self.tokens.get(self.current - 1).unwrap();
        (**t).clone()
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SemiColon {
                return;
            }
            match self.peek().token_type {
                TokenType::Class
                | TokenType::For
                | TokenType::Fun
                | TokenType::If
                | TokenType::Print
                | TokenType::Return
                | TokenType::Var
                | TokenType::While => return,
                _ => (),
            }
            self.advance();
        }
    }
}
