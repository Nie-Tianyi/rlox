use crate::token::Token;

#[allow(dead_code)]
enum Expression {
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    Literal {
        value: Token,
    },
    Grouping {
        expression: Box<Expression>,
    },
    Operator {
        operator: Token,
    },
}
