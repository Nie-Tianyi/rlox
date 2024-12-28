use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    #[allow(clippy::upper_case_acronyms)]
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    Null,
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::String(s) => {
                write!(f, "\"{}\"", s)
            }
            Literal::Number(fl) => {
                write!(f, "\"{}\"", fl)
            }
            Literal::Null => {
                write!(f, "None")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: impl ToString,
        literal: Literal,
        line: usize,
    ) -> Self {
        Token {
            token_type,
            lexeme: lexeme.to_string(),
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{:?}-{}-{}>",
            self.token_type, self.lexeme, self.literal
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let token = Token::new(
            TokenType::String,
            "String".to_string(),
            Literal::String("Hello World".to_string()),
            12,
        );
        println!("{token}")
    }
}
