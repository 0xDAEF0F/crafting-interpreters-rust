use std::{env, io, process};

fn main() {
    let mut args = env::args().into_iter();

    if args.len() > 1 {
        eprintln!("Not supported");
        process::exit(1);
    }

    run_prompt();
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

fn run(line: String) {
    let tokens = Scanner::build(line).scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}
