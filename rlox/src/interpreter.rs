use crate::{
    generate_ast::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, Stmt, UnaryExpr},
    token::{Object, Token},
    token_type::TokenType,
    LoxRuntimeError,
};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&self, stmts: Vec<&Stmt>) -> Result<(), LoxRuntimeError> {
        for stmt in stmts {
            self.execute_stmt(stmt)?;
        }

        Ok(())
    }

    fn execute_stmt(&self, stmt: &Stmt) -> Result<(), LoxRuntimeError> {
        match stmt {
            Stmt::Expression(stmt) => {
                self.evaluate_expr(&stmt.expression)?;
            }
            Stmt::Print(stmt) => {
                let value = self.evaluate_expr(&stmt.expression)?;
                println!("{}", self.strigify(&value));
            }
        }
        Ok(())
    }

    fn evaluate_expr(&self, expr: &Expr) -> Result<Object, LoxRuntimeError> {
        let obj = match expr {
            Expr::Binary(expr) => self.evaluate_binary(expr)?,
            Expr::Grouping(expr) => self.evaluate_grouping(expr)?,
            Expr::Literal(expr) => self.evaluate_literal(expr)?,
            Expr::Unary(expr) => self.evaluate_unary(expr)?,
        };
        Ok(obj)
    }

    fn evaluate_binary(&self, expr: &BinaryExpr) -> Result<Object, LoxRuntimeError> {
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

    fn evaluate_grouping(&self, expr: &GroupingExpr) -> Result<Object, LoxRuntimeError> {
        self.evaluate_expr(&expr.expression)
    }

    fn evaluate_literal(&self, expr: &LiteralExpr) -> Result<Object, LoxRuntimeError> {
        Ok(expr.value.clone())
    }

    fn evaluate_unary(&self, expr: &UnaryExpr) -> Result<Object, LoxRuntimeError> {
        let right = self.evaluate_expr(&expr.right)?;

        let obj = match expr.operator.token_type {
            TokenType::Bang => Object::Bool(!self.is_truthy(&right)),
            TokenType::Minus => {
                let num = self.check_number_operand(&expr.operator, &right)?;
                Object::Num(-num)
            }
            _ => unimplemented!(),
        };
        Ok(obj)
    }

    fn is_truthy(&self, obj: &Object) -> bool {
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
