mod expression;
mod parser;
mod reporter;
mod scanner;
mod token;

use crate::expression::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use std::io::{Read, Write};
use std::path::Path;
use std::{env, fs, io};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => run_file(&args[1]),
        1 => run_prompt(),
        _ => {
            eprintln!("Usage: {} [file_path]", args[0]);
            std::process::exit(1);
        }
    }
}

fn run_prompt() {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush().expect("fail to flush");

        stdin
            .read_line(&mut input)
            .expect("fail to read from terminal");

        if input == "\n" {
            break;
        }

        run(input.clone());
        input.clear();
    }
}

fn run_file(path: impl AsRef<Path>) {
    let mut file = fs::File::open(path).expect("fail to find given file");
    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("fail to read given file");

    run(content);
}

fn run(source_code: String) {
    let tokens = Scanner::parse(source_code);
    let expr = Parser::parse(tokens);
    Interpreter::interpret(&expr);
}
