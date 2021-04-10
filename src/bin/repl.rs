use loxlib::{execute, execute_file};
use std::env;
use std::io::stdin;

fn replit() {
    loop {
        print!("> ");
        let mut line = String::new();

        stdin()
            .read_line(&mut line)
            .expect("Failed to read from stdin");

        if line.is_empty() || line == "\n" {
            break;
        }
        execute(&line);
    }
}

fn main() {
    let args = env::args();

    match args.len() {
        2 => execute_file(args.last().expect("Missing filepath")),
        _ => replit(),
    }
}
