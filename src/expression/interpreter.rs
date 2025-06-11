use std::fmt::{Display, Formatter};
use crate::expression::{ExprLiteral, ExprVisitor, Expression};
use crate::token::{Token, TokenType};
use std::ops::{Add, Neg, Not};

#[derive(Debug)]
pub struct RuntimeError {
    msg: &'static str,
}

type RuntimeResult<T> = Result<T, RuntimeError>;

// 这个跟 ExprLiteral 基本上一样，但是语义不一样，一个表示运行时的值，另一个表示在从源码中解析出来的Token
#[derive(Debug, Clone)]
pub enum Value {
    Str(String), // strings
    Number(f64), // numbers
    Nil,         // nil
    Bool(bool),  // true or false
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Str(s) => {
                write!(f, "\"{}\"", s)
            }
            Value::Number(fl) => {
                write!(f, "{}", fl)
            }
            Value::Nil => {
                write!(f, "nil")
            }
            Value::Bool(true) => {
                write!(f, "true")
            }
            Value::Bool(false) => {
                write!(f, "false")
            }
        }
    }
}

impl Value {
    // -val
    fn negative(self) -> RuntimeResult<Value> {
        match self {
            Value::Number(n) => Ok(Value::Number(n.neg())),
            _ => Err(RuntimeError {
                msg: "Cannot apply negative operand on non-numeric values",
            }),
        }
    }

    // !val
    fn ops_not(self) -> RuntimeResult<Value> {
        Ok(Value::Bool(self.into_bool().not()))
    }

    #[allow(clippy::match_like_matches_macro)]
    fn into_bool(self) -> bool {
        match self {
            Self::Bool(false) | Self::Nil | Self::Number(0_f64) => false,
            _ => true,
        }
    }

    fn into_string(self) -> String {
        match self {
            Value::Str(s) => s,
            Value::Number(n) => n.to_string(),
            Value::Nil => "".to_string(),
            Value::Bool(true) => "true".to_string(),
            Value::Bool(false) => "false".to_string(),
        }
    }

    fn try_into_number(self) -> RuntimeResult<f64> {
        match self {
            Value::Str(s) => match s.parse::<f64>() {
                Ok(f) => Ok(f),
                Err(_) => Err(RuntimeError {
                    msg: "Error parsing numbers",
                }),
            },
            Value::Number(n) => Ok(n),
            Value::Bool(true) => Ok(1_f64),
            Value::Bool(false) | Value::Nil => Ok(0_f64),
        }
    }

    // val1 + val2
    fn add(self, other: Self) -> RuntimeResult<Value> {
        match (self, other) {
            (Value::Str(s1), Value::Str(s2)) => Ok(Value::Str(s1 + s2.as_str())),
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 + n2)),
            (Value::Str(s), Value::Number(n)) => Ok(Value::Str(s + n.to_string().as_str())), // 语法糖
            (Value::Number(n), Value::Str(s)) => Ok(Value::Str(n.to_string() + s.as_str())), // 语法糖
            _ => Err(RuntimeError {
                msg: "Cannot apply addition operand on non-numeric values",
            }),
        }
    }

    // val1 - val2
    fn sub(self, other: Self) -> RuntimeResult<Value> {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 - n2)),
            _ => Err(RuntimeError {
                msg: "Cannot apply subtraction operand on non-numeric values",
            }),
        }
    }

    // val1 * val2
    fn mul(self, other: Self) -> RuntimeResult<Value> {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 * n2)),
            _ => Err(RuntimeError {
                msg: "Cannot apply multiplication operand on non-numeric values",
            }),
        }
    }

    // val1 / val2
    fn div(self, other: Self) -> RuntimeResult<Value> {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 / n2)),
            _ => Err(RuntimeError {
                msg: "Cannot apply division operand on non-numeric values",
            }),
        }
    }

    // val1 > val2
    fn gt(self, other: Self) -> RuntimeResult<Value> {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Bool(n1 > n2)),
            _ => Err(RuntimeError {
                msg: "Cannot apply greater than operand on non-numeric values",
            }),
        }
    }

    // val1 >= val2
    fn gte(self, other: Self) -> RuntimeResult<Value> {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Bool(n1 >= n2)),
            _ => Err(RuntimeError {
                msg: "Cannot apply greater than or equal operand on non-numeric values",
            }),
        }
    }

    // val1 < val2
    fn lt(self, other: Self) -> RuntimeResult<Value> {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Bool(n1 < n2)),
            _ => Err(RuntimeError {
                msg: "Cannot apply less than operand on non-numeric values",
            }),
        }
    }

    // val1 <= val2
    fn lte(self, other: Self) -> RuntimeResult<Value> {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Bool(n1 <= n2)),
            _ => Err(RuntimeError {
                msg: "Cannot apply less than or equal operand on non-numeric values",
            }),
        }
    }

    // val1 == val2
    fn eq(self, other: Self) -> RuntimeResult<Value> {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Bool(n1 == n2)),
            (Value::Str(s1), Value::Str(s2)) => Ok(Value::Bool(s1 == s2)),
            (Value::Bool(b1), Value::Bool(b2)) => Ok(Value::Bool(b1 == b2)),
            (Value::Nil, Value::Nil) => Ok(Value::Bool(true)),
            _ => Ok(Value::Bool(false)),
        }
    }

    // val1 != val2
    fn neq(self, other: Self) -> RuntimeResult<Value> {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Bool(n1 != n2)),
            (Value::Str(s1), Value::Str(s2)) => Ok(Value::Bool(s1 != s2)),
            (Value::Bool(b1), Value::Bool(b2)) => Ok(Value::Bool(b1 != b2)),
            (Value::Nil, Value::Nil) => Ok(Value::Bool(false)),
            _ => Ok(Value::Bool(true)),
        }
    }
}

pub struct Interpreter;

impl ExprVisitor<RuntimeResult<Value>> for Interpreter {
    fn visit_binary(
        &self,
        left: &Box<Expression>,
        operator: &Token,
        right: &Box<Expression>,
    ) -> RuntimeResult<Value> {
        let left_val = self.evaluate(left)?;
        let right_val = self.evaluate(right)?;

        match operator.token_type() {
            TokenType::Minus => left_val.sub(right_val),
            TokenType::Plus => left_val.add(right_val),
            TokenType::Slash => left_val.div(right_val),
            TokenType::Star => left_val.mul(right_val),
            TokenType::Greater => left_val.gt(right_val),
            TokenType::GreaterEqual => left_val.gte(right_val),
            TokenType::Less => left_val.lt(right_val),
            TokenType::LessEqual => left_val.lte(right_val),
            TokenType::BangEqual => left_val.neq(right_val),
            TokenType::EqualEqual => left_val.eq(right_val),
            _ => unreachable!(),
        }
    }

    fn visit_literal(&self, value: &ExprLiteral) -> RuntimeResult<Value> {
        match value {
            ExprLiteral::String(s) => Ok(Value::Str(s.clone())),
            ExprLiteral::Number(n) => Ok(Value::Number(*n)),
            ExprLiteral::Nil => Ok(Value::Nil),
            ExprLiteral::Bool(b) => Ok(Value::Bool(*b)),
        }
    }

    fn visit_grouping(&self, expr: &Box<Expression>) -> RuntimeResult<Value> {
        self.evaluate(expr)
    }

    fn visit_unary(&self, operator: &Token, right: &Box<Expression>) -> RuntimeResult<Value> {
        let right_val = self.evaluate(right)?;
        match operator.token_type() {
            TokenType::Minus => right_val.negative(),
            TokenType::Bang => right_val.ops_not(),
            _ => unreachable!(),
        }
    }
}

impl Interpreter {
    fn evaluate(&self, expr: &Expression) -> RuntimeResult<Value> {
        expr.accept(self)
    }
}

#[cfg(test)]
mod tests {
    
    use crate::expression::interpreter::Interpreter;
    use crate::parser::Parser;
    use crate::scanner::Scanner;
    
    fn assert_eq(source: &str, expected: &str) {
        let tokens = Scanner::parse(source);
        let parser = Parser::parse(tokens);
        let expr = parser.accept(&Interpreter);
        assert!(expr.is_ok());
        let val = expr.unwrap();
        assert_eq!(val.into_string(), expected);
    }
    
    fn assert_error(source: &str) {
        let tokens = Scanner::parse(source);
        let parser = Parser::parse(tokens);
        let expr = parser.accept(&Interpreter);
        assert!(expr.is_err());
    }
    
    #[test]
    fn test_1() {
        assert_eq("1 + 1;", "2");
        assert_eq("1 + 1 * 2;", "3");
        assert_eq("1 + 1 * 2 - 3;", "0");
        assert_eq("1 + 1 * 2 - 3 / 4;", "2.25");
        assert_eq("1 + 1 * 2 - 3 / 4 > 5;", "false");
        assert_eq("1 + 1 * 2 - 3 / 4 < 5;", "true");
        assert_eq("1 + 1 * 2 - 3 / 4 >= 5;", "false");
        assert_eq("1 + 1 * 2 - 3 / 4 <= 5;", "true");
        assert_eq("123 + \"123\"", "123123");
        assert_eq("123 + \"123\" == \"123123\";", "true");
        assert_eq("123 + \"123\" != \"123123\";", "false");
        assert_eq("123 + \"123\" == 123123 + 1;", "false");
    }
}
