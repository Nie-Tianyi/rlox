use std::fmt::Display;

#[inline]
pub fn error(line: usize, message: impl Display) {
    report(line, "", message)
}

#[inline]
pub fn report(line: usize, wheres: impl Display, message: impl Display) {
    println!("[line {line}] Error {wheres}: {message}");
}