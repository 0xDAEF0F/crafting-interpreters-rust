use crafting_interpreters::{scanner::Scanner, token::Token};
use std::{env, fs, io, process};

#[allow(dead_code)]
struct Lox {
    had_error: bool,
}

fn main() {
    let mut args = env::args().into_iter();

    if args.len() > 2 {
        eprintln!("Usage: jlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        args.next();
        run_file(args.next().unwrap());
    } else {
        run_prompt();
    }
}

fn run_file(path: String) {
    let contents = fs::read_to_string(path).unwrap();
    let is_error = run(contents);
    if is_error {
        process::exit(65);
    }
}

fn run_prompt() {
    loop {
        let mut line = String::new();

        println!("> ");
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line.");

        if line.is_empty() {
            break;
        }

        run(line);
    }
}

// NOTE: returns `true` if an error ocurred.
fn run(line: String) -> bool {
    let scanner = Scanner::new(line);
    let tokens: Vec<Token> = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }

    false
}

fn error(line: usize, message: String) {
    report(line, "".to_string(), message)
}

fn report(line: usize, where_: String, message: String) {
    eprintln!("[line {}] Error{}: {}", line, where_, message);
}
