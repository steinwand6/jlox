use std::rc::Rc;

use crate::{
    environment::Environment,
    generate_ast::{AssignExpr, BinaryExpr, Expr, GroupingExpr, LiteralExpr, Stmt, UnaryExpr},
    token::{Object, Token},
    token_type::TokenType,
    LoxRuntimeError,
};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), LoxRuntimeError> {
        for stmt in stmts {
            self.execute_stmt(&stmt)?;
        }

        Ok(())
    }

    fn execute_stmt(&mut self, stmt: &Stmt) -> Result<(), LoxRuntimeError> {
        match stmt {
            Stmt::Expression(stmt) => {
                self.evaluate_expr(&stmt.expression)?;
            }
            Stmt::If(stmt) => {
                if Self::is_truthy(&self.evaluate_expr(&stmt.condition)?) {
                    self.execute_stmt(&stmt.then_branch)?;
                } else if let Some(b) = &stmt.else_branch {
                    self.execute_stmt(b)?;
                }
            }
            Stmt::While(stmt) => {
                while Self::is_truthy(&self.evaluate_expr(&stmt.condition)?) {
                    self.execute_stmt(&stmt.body)?;
                }
            }
            Stmt::Block(stmt) => {
                let previous = self.environment.clone();
                self.environment = Environment::new_enclosing(Rc::new(self.environment.clone()));
                for s in &stmt.statements {
                    self.execute_stmt(s)?;
                }
                self.environment = previous;
            }
            Stmt::Print(stmt) => {
                let value = self.evaluate_expr(&stmt.expression)?;
                println!("{}", self.strigify(&value));
            }
            Stmt::Var(stmt) => {
                let value = self.evaluate_expr(&stmt.initializer)?;
                self.environment.define(&stmt.name.lexeme, &value);
            }
        }
        Ok(())
    }

    fn evaluate_expr(&mut self, expr: &Expr) -> Result<Object, LoxRuntimeError> {
        let obj = match expr {
            Expr::Assign(expr) => self.evaluate_assign(expr)?,
            Expr::Binary(expr) => self.evaluate_binary(expr)?,
            Expr::Grouping(expr) => self.evaluate_grouping(expr)?,
            Expr::Literal(expr) => self.evaluate_literal(expr)?,
            Expr::Unary(expr) => self.evaluate_unary(expr)?,
            Expr::Variable(expr) => self.environment.get(&expr.name)?,
        };
        Ok(obj)
    }

    fn evaluate_assign(&mut self, expr: &AssignExpr) -> Result<Object, LoxRuntimeError> {
        let value = self.evaluate_expr(&expr.value)?;
        self.environment.assign(&expr.name, &value)?;
        Ok(value)
    }

    fn evaluate_binary(&mut self, expr: &BinaryExpr) -> Result<Object, LoxRuntimeError> {
        let left = self.evaluate_expr(&expr.left)?;
        let right = self.evaluate_expr(&expr.right)?;

        match expr.operator.token_type {
            TokenType::Plus => match (left, right) {
                (Object::String(left), Object::String(right)) => {
                    Ok(Object::String(format!("{}{}", left, right)))
                }
                (Object::Num(left), Object::Num(right)) => Ok(Object::Num(left + right)),
                _ => Err(LoxRuntimeError(
                    expr.operator.clone(),
                    "Operands must be two numbers or two strings.".into(),
                )),
            },
            TokenType::Minus => {
                let (a, b) = self.check_number_operands(&expr.operator, &left, &right)?;
                Ok(Object::Num(a - b))
            }
            TokenType::Star => {
                let (a, b) = self.check_number_operands(&expr.operator, &left, &right)?;
                Ok(Object::Num(a * b))
            }
            TokenType::Slash => {
                let (a, b) = self.check_number_operands(&expr.operator, &left, &right)?;
                Ok(Object::Num(a / b))
            }

            TokenType::Greater => {
                let (a, b) = self.check_number_operands(&expr.operator, &left, &right)?;
                Ok(Object::Bool(a > b))
            }
            TokenType::GreaterEqual => {
                let (a, b) = self.check_number_operands(&expr.operator, &left, &right)?;
                Ok(Object::Bool(a >= b))
            }
            TokenType::Less => {
                let (a, b) = self.check_number_operands(&expr.operator, &left, &right)?;
                Ok(Object::Bool(a < b))
            }
            TokenType::LessEqual => {
                let (a, b) = self.check_number_operands(&expr.operator, &left, &right)?;
                Ok(Object::Bool(a <= b))
            }

            TokenType::BangEqual => Ok(Object::Bool(left != right)),
            TokenType::EqualEqual => Ok(Object::Bool(left == right)),
            _ => unimplemented!(),
        }
    }

    fn evaluate_grouping(&mut self, expr: &GroupingExpr) -> Result<Object, LoxRuntimeError> {
        self.evaluate_expr(&expr.expression)
    }

    fn evaluate_literal(&self, expr: &LiteralExpr) -> Result<Object, LoxRuntimeError> {
        Ok(expr.value.clone())
    }

    fn evaluate_unary(&mut self, expr: &UnaryExpr) -> Result<Object, LoxRuntimeError> {
        let right = self.evaluate_expr(&expr.right)?;

        let obj = match expr.operator.token_type {
            TokenType::Bang => Object::Bool(!Self::is_truthy(&right)),
            TokenType::Minus => {
                let num = self.check_number_operand(&expr.operator, &right)?;
                Object::Num(-num)
            }
            _ => unimplemented!(),
        };
        Ok(obj)
    }

    fn is_truthy(obj: &Object) -> bool {
        match obj {
            Object::Bool(b) => *b,
            Object::None => false,
            _ => true,
        }
    }

    fn check_number_operand(
        &self,
        operator: &Token,
        operand: &Object,
    ) -> Result<f64, LoxRuntimeError> {
        match operand.num() {
            Ok(num) => Ok(num),
            Err(_) => Err(LoxRuntimeError(
                operator.clone(),
                "Operand must be number.".into(),
            )),
        }
    }

    fn check_number_operands(
        &self,
        operator: &Token,
        a: &Object,
        b: &Object,
    ) -> Result<(f64, f64), LoxRuntimeError> {
        match (a.num(), b.num()) {
            (Ok(a), Ok(b)) => Ok((a, b)),
            _ => Err(LoxRuntimeError(
                operator.clone(),
                "Operand must be numbers.".into(),
            )),
        }
    }

    fn strigify(&self, obj: &Object) -> String {
        match obj {
            Object::String(s) => s.into(),
            Object::Bool(b) => b.to_string(),
            Object::Num(n) => n.to_string().replace(".0", ""),
            Object::None => "nil".into(),
        }
    }
}
