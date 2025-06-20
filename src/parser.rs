use crate::expression::{ExprLiteral, Expression};
use crate::reporter::error_at_token;
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
pub struct Parser {
    tokens: Vec<Token>,
    current: RefCell<usize>,
}

#[derive(Debug)]
pub struct ParseError;

pub type ParseResult<T> = Result<T, ParseError>;

// basic methods
impl Parser {
    #[inline]
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: RefCell::new(0),
        }
    }

    #[inline]
    fn parse_tokens(&self) -> Expression {
        self.expression().unwrap_or(Expression::Literal {
            value: ExprLiteral::Nil,
        })
    }

    #[inline]
    pub fn parse(tokens: Vec<Token>) -> Expression {
        let parser = Self::new(tokens);
        parser.parse_tokens()
    }

    #[inline]
    fn matches(&self, types: &[TokenType]) -> bool {
        for tt in types {
            if self.check(*tt) {
                self.advance();
                return true;
            }
        }

        false
    }

    #[inline]
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type() == token_type
    }

    #[inline]
    fn advance(&self) -> &Token {
        if !self.is_at_end() {
            *self.current.borrow_mut() += 1;
        }

        self.previous()
    }

    #[inline]
    fn is_at_end(&self) -> bool {
        self.peek().token_type() == EOF
    }

    #[inline]
    fn peek(&self) -> &Token {
        &self.tokens[*self.current.borrow()]
    }

    #[inline]
    fn previous(&self) -> &Token {
        &self.tokens[*self.current.borrow() - 1]
    }

    #[inline]
    fn consume(&self, ty: TokenType, msg: impl Display) -> Result<&Token, ParseError> {
        if self.check(ty) {
            return Ok(self.advance());
        }

        Err(Self::error(self.peek(), msg))
    }

    #[inline]
    fn error(t: &Token, msg: impl Display) -> ParseError {
        error_at_token(t, msg);
        ParseError
    }
}
// methods for constructing AST
impl Parser {
    fn expression(&self) -> ParseResult<Expression> {
        self.equality()
    }

    fn equality(&self) -> ParseResult<Expression> {
        let mut expr = self.comparison()?;

        while self.matches(&[BangEqual, EqualEqual]) {
            let token_operator = self.previous();
            let right = self.comparison()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: token_operator.clone(),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn comparison(&self) -> ParseResult<Expression> {
        let mut expr = self.term()?;

        while self.matches(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn term(&self) -> ParseResult<Expression> {
        let mut expr = self.factor()?;

        while self.matches(&[Minus, Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn factor(&self) -> ParseResult<Expression> {
        let mut expr = self.unary()?;
        while self.matches(&[Slash, Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn unary(&self) -> ParseResult<Expression> {
        if self.matches(&[Bang, Minus]) {
            let op = self.previous();
            let right = self.unary()?;

            return Ok(Expression::Unary {
                operator: op.clone(),
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&self) -> ParseResult<Expression> {
        if self.matches(&[False]) {
            return Ok(Expression::Literal {
                value: ExprLiteral::Bool(false),
            });
        }

        if self.matches(&[True]) {
            return Ok(Expression::Literal {
                value: ExprLiteral::Bool(true),
            });
        }

        if self.matches(&[Nil]) {
            return Ok(Expression::Literal {
                value: ExprLiteral::Nil,
            });
        }

        if self.matches(&[Number]) {
            let val = match self.previous().literal() {
                Literal::Number(i) => *i,
                _ => {
                    return Err(Self::error(self.peek(), "error parsing Number"));
                }
            };

            return Ok(Expression::Literal {
                value: ExprLiteral::Number(val),
            });
        }

        if self.matches(&[String]) {
            let val = match self.previous().literal() {
                Literal::String(i) => i.clone(),
                _ => {
                    return Err(Self::error(self.peek(), "error parsing Strings"));
                }
            };

            return Ok(Expression::Literal {
                value: ExprLiteral::String(val),
            });
        }

        if self.matches(&[LeftParen]) {
            let expr = self.expression()?;
            self.consume(RightParen, "Expect ')' after expression.")?;
            return Ok(Expression::Grouping {
                expr: Box::new(expr),
            });
        }

        Err(Self::error(self.peek(), "unexpected token"))
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::ast_printer::AstPrinter;
    use crate::parser::Parser;
    use crate::scanner::Scanner;

    fn compile_to_ast(source_code: &str) -> String {
        let tokens = Scanner::parse(source_code);
        let expr = Parser::parse(tokens);
        expr.accept(&AstPrinter)
    }

    #[test]
    fn test_1() {
        // 现在语法分析其只支持以下这些运算
        // == != > >= < <=
        // + - * / ! -
        // number string nil true false ()
        assert_eq!(compile_to_ast("2 + 2;"), "(+ 2 2)");
        assert_eq!(
            compile_to_ast("3.14 * (2 + 2);"),
            "(* 3.14 (group (+ 2 2)))"
        );
        assert_eq!(compile_to_ast("3.14 * 2 + 2;"), "(+ (* 3.14 2) 2)");

        println!("{}", compile_to_ast("123 + \"123\" != \"123123\";"));
    }
}
