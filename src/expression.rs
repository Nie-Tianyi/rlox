use crate::token::Token;

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
