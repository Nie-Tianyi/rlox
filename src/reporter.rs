use crate::expression::interpreter::RuntimeError;
use crate::token::{Token, TokenType};
use std::fmt::Display;
use std::process;

#[inline]
pub fn error_at_line(line: usize, message: impl Display) {
    report(line, "", message)
}

#[inline]
pub fn error_at_token(token: &Token, message: impl Display) {
    if token.token_type() == TokenType::EOF {
        report(token.line(), " at end", message);
    } else {
        report(token.line(), format!(" at '{}'", token.lexeme()), message);
    }
}

#[inline]
pub fn runtime_error(error: RuntimeError) {
    println!("{}\n[line {}]", error.msg, error.token.line());
    process::exit(70);
}

#[inline]
pub fn report(line: usize, wheres: impl Display, message: impl Display) {
    println!("[line {line}] Error {wheres}: {message}");
    process::exit(65);
}
