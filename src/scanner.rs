use crate::reporter;
use crate::token::{Literal, Token, TokenType};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut key_words = HashMap::new();

        key_words.insert("and", TokenType::And);
        key_words.insert("class", TokenType::Class);
        key_words.insert("else", TokenType::Else);
        key_words.insert("false", TokenType::False);
        key_words.insert("for", TokenType::For);
        key_words.insert("fun", TokenType::Fun);
        key_words.insert("if", TokenType::If);
        key_words.insert("nil", TokenType::Nil);
        key_words.insert("or", TokenType::Or);
        key_words.insert("print", TokenType::Print);
        key_words.insert("return", TokenType::Return);
        key_words.insert("super", TokenType::Super);
        key_words.insert("this", TokenType::This);
        key_words.insert("true", TokenType::True);
        key_words.insert("var", TokenType::Var);
        key_words.insert("while", TokenType::While);

        key_words
    };
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: impl ToString) -> Self {
        Scanner {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(&mut self) {
        loop {
            self.start = self.current;
            if let Some(c) = self.next_char() {
                match c {
                    '(' => self.add_token(TokenType::LeftParen, Literal::Null),
                    ')' => self.add_token(TokenType::RightParen, Literal::Null),
                    '{' => self.add_token(TokenType::LeftBrace, Literal::Null),
                    '}' => self.add_token(TokenType::RightBrace, Literal::Null),
                    ',' => self.add_token(TokenType::Comma, Literal::Null),
                    '.' => self.add_token(TokenType::Dot, Literal::Null),
                    '-' => self.add_token(TokenType::Minus, Literal::Null),
                    '+' => self.add_token(TokenType::Plus, Literal::Null),
                    ';' => self.add_token(TokenType::Semicolon, Literal::Null),
                    '*' => self.add_token(TokenType::Star, Literal::Null),
                    '!' => {
                        if self.next_char_matches('=') {
                            self.add_token(TokenType::Bang, Literal::Null);
                        } else {
                            self.add_token(TokenType::BangEqual, Literal::Null);
                        }
                    }
                    '=' => {
                        if self.next_char_matches('=') {
                            self.add_token(TokenType::EqualEqual, Literal::Null);
                        } else {
                            self.add_token(TokenType::Equal, Literal::Null);
                        }
                    }
                    '<' => {
                        if self.next_char_matches('=') {
                            self.add_token(TokenType::LessEqual, Literal::Null);
                        } else {
                            self.add_token(TokenType::Less, Literal::Null);
                        }
                    }
                    '>' => {
                        if self.next_char_matches('=') {
                            self.add_token(TokenType::GreaterEqual, Literal::Null);
                        } else {
                            self.add_token(TokenType::Greater, Literal::Null);
                        }
                    }
                    '/' => {
                        if self.next_char_matches('/') {
                            while self.peek().is_some() && self.peek() != Some('\n') {
                                self.next_char();
                            }
                        } else {
                            self.add_token(TokenType::Slash, Literal::Null);
                        }
                    }

                    ' ' | '\r' | '\t' => (),
                    '\n' => self.line += 1,

                    '"' => self.string(),
                    c => {
                        if Self::is_digit(c) {
                            self.number();
                        } else if Self::is_alpha(c) {
                            self.identifier();
                        } else {
                            reporter::error(self.line, "Unexpected character.");
                        }
                    }
                }
            } else {
                break;
            }
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "", Literal::Null, self.line));
    }

    #[inline]
    fn is_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    #[inline]
    fn is_alpha(c: char) -> bool {
        c.is_ascii_uppercase() || c.is_ascii_lowercase() || c == '_'
    }

    #[inline]
    fn is_alpha_numeric(c: char) -> bool {
        Self::is_alpha(c) || Self::is_digit(c)
    }

    fn identifier(&mut self) {
        while self.peek().is_some() && Self::is_alpha_numeric(self.peek().unwrap()) {
            self.next_char();
        }

        let text = &self.source[self.start..self.current];
        let token_type = KEYWORDS.get(text);

        if let Some(token_type) = token_type {
            self.add_token(*token_type, Literal::Null);
        } else {
            self.add_token(TokenType::Identifier, Literal::Null);
        }
    }

    fn number(&mut self) {
        while Self::is_digit(self.peek().unwrap()) {
            self.next_char();
        }
        if self.peek() == Some('.')
            && self.peek_next().is_some()
            && Self::is_digit(self.peek_next().unwrap())
        {
            self.next_char();
            while Self::is_digit(self.peek().unwrap()) {
                self.next_char();
            }
        }
        let val = &self.source[self.start..self.current];
        let val = val.parse::<f64>();
        if val.is_err() {
            reporter::error(self.line, "error parsing number");
        }
        self.add_token(TokenType::Number, Literal::Number(val.unwrap()))
    }

    fn string(&mut self) {
        while self.peek().is_some() && self.peek().unwrap() != '"' {
            if self.peek() == Some('\n') {
                self.line += 1
            }
            self.next_char();
        }
        if self.peek().is_none() {
            reporter::error(self.line, "Unterminated String");
            return;
        }
        self.next_char();

        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String, Literal::String(value.to_string()));
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.current);
        self.current += 1;
        c
    }

    fn next_char_matches(&mut self, c: char) -> bool {
        let e = self.peek();

        if e.is_none() || e.unwrap() != c {
            return false;
        }

        self.current += 1;

        true
    }

    #[inline]
    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    #[inline]
    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }

    #[inline]
    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    #[inline]
    fn take_tokens(self) -> Vec<Token> {
        self.tokens
    }

    #[inline]
    pub fn parse(source_code: impl ToString) -> Vec<Token> {
        let mut scanner = Scanner::new(source_code);
        scanner.scan_tokens();
        scanner.take_tokens()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenStream;

    #[test]
    fn test_1() {
        let tokens: TokenStream = Scanner::parse(
            r#"
                var i = 1; // this is a comment
                print i;
            "#,
        )
        .into();
        println!("{tokens:?}")
    }

    #[test]
    fn test_2() {
        let tokens: TokenStream = Scanner::parse(
            r#"
                var a = true; // this is a comment
                var b = 2.13;
                var c = "abc";
            "#,
        )
        .into();
        println!("{tokens:?}")
    }
}
