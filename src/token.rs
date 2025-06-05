use std::fmt::{Debug, Display, Formatter};

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

#[derive(Clone, PartialEq)]
pub enum Literal {
    String(String), // string literal
    Number(f64),    // 所有的number内部使用f64储存
    None,           // 用于占位，表示该Token没有Literal
}

impl Debug for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::String(s) => {
                write!(f, "string:\"{}\"", s)
            }
            Literal::Number(fl) => {
                write!(f, "number:\"{}\"", fl)
            }
            Literal::None => {
                write!(f, "None")
            }
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::String(s) => {
                write!(f, "{}", s)
            }
            Literal::Number(fl) => {
                write!(f, "{}", fl)
            }
            Literal::None => {
                write!(f, "None")
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Token {
    token_type: TokenType, // token的类型
    lexeme: String,        // token的源代码中的表示
    literal: Literal, // 当token为String或者Number时，这里记录String或者Number的具体内容，其他的为Null
    line: usize,      // token在源码的第几行
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

    pub fn token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn literal(&self) -> &Literal {
        &self.literal
    }

    pub fn line(&self) -> usize {
        self.line
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{:?}-{:?}-{:?}>",
            self.token_type, self.lexeme, self.literal
        )
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.token_type {
            TokenType::EOF => {
                write!(f, "EOF")?;
            }
            TokenType::String => {
                write!(f, "{}", self.literal)?;
            }
            TokenType::Number => {
                write!(f, "{}", self.literal)?;
            }
            _ => {
                write!(f, "{}", self.lexeme)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct TokenStream(Vec<Token>);

impl From<Vec<Token>> for TokenStream {
    fn from(value: Vec<Token>) -> Self {
        TokenStream(value)
    }
}

impl Display for TokenStream {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut tokens = self.0.iter();
        if let Some(first) = tokens.next() {
            write!(f, "{}", first)?;
            for token in tokens {
                write!(f, " {}", token)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let token = Token::new(
            TokenType::String,
            "String",
            Literal::String("Hello World".to_string()),
            12,
        );
        println!("{token:?}");
        println!("{token}");
    }
}
