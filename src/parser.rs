use crate::expression::Expression;
use crate::token::Token;

/*
 * Lox语法规则：
 * expression     → equality ;
 * equality       → comparison ( ( "!=" | "==" ) comparison )* ;
 * comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
 * term           → factor ( ( "-" | "+" ) factor )* ;
 * factor         → unary ( ( "/" | "*" ) unary )* ;
 * unary          → ( "!" | "-" ) unary | primary ;
 * primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
 */
#[allow(unused)]
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn expression(&self) -> Expression {
        self.equality()
    }

    fn equality(&self) -> Expression {
        unimplemented!()
    }

    fn comparison(&self) -> Expression {
        unimplemented!()
    }

    fn term(&self) -> Expression {
        unimplemented!()
    }

    fn factor(&self) -> Expression {
        unimplemented!()
    }

    fn unary(&self) -> Expression {
        unimplemented!()
    }

    fn primary(&self) -> Expression {
        unimplemented!()
    }
}
