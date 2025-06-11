use crate::expression::{ExprLiteral, ExprVisitor, Expression};
use crate::token::Token;

#[allow(unused)]
pub struct AstPrinter;

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary(&self, left: &Box<Expression>, op: &Token, right: &Box<Expression>) -> String {
        format!(
            "({} {} {})",
            op.lexeme(),
            left.accept(self),
            right.accept(self)
        )
    }

    fn visit_literal(&self, value: &ExprLiteral) -> String {
        ExprLiteral::to_string(value)
    }

    fn visit_grouping(&self, expr: &Box<Expression>) -> String {
        format!("(group {})", expr.accept(self))
    }

    fn visit_unary(&self, operator: &Token, right: &Box<Expression>) -> String {
        format!("({} {})", operator.lexeme(), right.accept(self))
    }
}
