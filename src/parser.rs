use crate::expression::{ExprLiteral, Expression};
use crate::token::TokenType::*;
use crate::token::{Literal, Token, TokenType};
use std::cell::RefCell;
use std::fmt::Display;
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
    current: RefCell<usize>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: RefCell::new(0),
        }
    }

    fn matches(&self, types: &[TokenType]) -> bool {
        for tt in types {
            if self.check(*tt) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type() == token_type
    }

    fn advance(&self) -> &Token {
        if !self.is_at_end() {
            *self.current.borrow_mut() += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type() == EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[*self.current.borrow()]
    }

    fn previous(&self) -> &Token {
        &self.tokens[*self.current.borrow() - 1]
    }

    fn expression(&self) -> Expression {
        self.equality()
    }

    fn equality(&self) -> Expression {
        let mut expr = self.comparison();

        while self.matches(&[BangEqual, EqualEqual]) {
            let token_operator = self.previous();
            let right = self.comparison();
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: token_operator.clone(),
                right: Box::new(right),
            }
        }

        expr
    }

    fn comparison(&self) -> Expression {
        let mut expr = self.term();

        while self.matches(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            }
        }

        expr
    }

    fn term(&self) -> Expression {
        let mut expr = self.factor();

        while self.matches(&[Minus, Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            }
        }

        expr
    }

    fn factor(&self) -> Expression {
        let mut expr = self.unary();
        while self.matches(&[Slash, Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            }
        }

        expr
    }

    fn unary(&self) -> Expression {
        if self.matches(&[Bang, Minus]) {
            let op = self.previous();
            let right = self.unary();

            return Expression::Unary {
                operator: op.clone(),
                right: Box::new(right),
            };
        }

        self.primary()
    }

    fn primary(&self) -> Expression {
        if self.matches(&[False]) {
            return Expression::Literal {
                value: ExprLiteral::False,
            };
        }

        if self.matches(&[True]) {
            return Expression::Literal {
                value: ExprLiteral::True,
            };
        }

        if self.matches(&[Nil]) {
            return Expression::Literal {
                value: ExprLiteral::Nil,
            };
        }

        if self.matches(&[Number]) {
            let val = match self.previous().literal() {
                Literal::Number(i) => *i,
                _ => {
                    panic!("error parsing Number");
                }
            };

            return Expression::Literal {
                value: ExprLiteral::Number(val),
            };
        }

        if self.matches(&[String]) {
            let val = match self.previous().literal() {
                Literal::String(i) => i.clone(),
                _ => {
                    panic!("error parsing Strings")
                }
            };

            return Expression::Literal {
                value: ExprLiteral::String(val),
            };
        }

        if self.matches(&[LeftParen]) {
            let expr = self.expression();
            consume(RightParen, "Expect ')' after expression.");
            return Expression::Grouping {
                expr: Box::new(expr),
            };
        }

        panic!("error parsing tokens, mismatched patterns")
    }
}

fn consume(ty: TokenType, msg: impl Display) {
    unimplemented!()
}
