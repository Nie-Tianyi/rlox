mod expression;
mod parser;
mod reporter;
mod scanner;
mod token;

use crate::scanner::Scanner;
use clap::{Parser, ValueHint};
use std::io::{Read, Write};
use std::path::Path;
use std::{fs, io};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(value_parser = clap::value_parser!(String),num_args = 1,
        value_hint = ValueHint::FilePath,)]
    path: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match cli.path {
        Some(path) => run_file(path),
        None => run_prompt(),
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
    println!("run {:?}", Scanner::parse(source_code))
}
