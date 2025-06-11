use crate::expression::{ExprLiteral, ExprVisitor, Expression};
use crate::token::Token;

pub struct Interpreter;

impl ExprVisitor<Expression> for Interpreter {
    fn visit_binary(&self, left: &Box<Expression>, operator: &Token, right: &Box<Expression>) -> Expression  {
        todo!()
    }

    fn visit_literal(&self, value: &ExprLiteral) -> Expression {
        todo!()
    }

    fn visit_grouping(&self, expr: &Box<Expression>) -> Expression  {
        todo!()
    }

    fn visit_unary(&self, operator: &Token, right: &Box<Expression>) -> Expression {
        todo!()
    }
}