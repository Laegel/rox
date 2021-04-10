use std::fs;

mod scanner;
mod token;
mod errors;
mod expr;

use scanner::Scanner;

pub fn execute(source: &str) {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token.lexeme);
    }
}

pub fn execute_file(filepath: String) {
    let code = fs::read_to_string(filepath).expect("Something went wrong reading the file");
    execute(&code);
}
