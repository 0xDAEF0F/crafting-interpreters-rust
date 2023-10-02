use derive_more::Display;
use std::{env, fs, io, process};

#[derive(Display, Debug)]
enum TokenType {
    #[display(fmt = "(")]
    LeftParen,
    #[display(fmt = ")")]
    RightParen,
    #[display(fmt = "{{")]
    LeftBrace,
    #[display(fmt = "}}")]
    RightBrace,
    #[display(fmt = ",")]
    Comma,
    #[display(fmt = ".")]
    Dot,
    #[display(fmt = "-")]
    Minus,
    #[display(fmt = "+")]
    Plus,
    #[display(fmt = ";")]
    Semicolon,
    #[display(fmt = "/")]
    Slash,
    #[display(fmt = "*")]
    Star,
    #[display(fmt = "!")]
    Bang,
    #[display(fmt = "!=")]
    BangEqual,
    #[display(fmt = "=")]
    Equal,
    #[display(fmt = "==")]
    EqualEqual,
    #[display(fmt = ">")]
    Greater,
    #[display(fmt = ">=")]
    GreaterEqual,
    #[display(fmt = "<")]
    Less,
    #[display(fmt = "<=")]
    LessEqual,
    Identifier,
    String,
    Number(i64),
    #[display(fmt = "&&")]
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    #[display(fmt = "if")]
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
    EOF,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: u64,
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u64,
}

impl Scanner {
    fn build(str: String) -> Self {
        Scanner {
            source: str,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;

            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            literal: "".to_string(),
            line: self.line,
        });

        self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&self) {
        // let c = self.
    }
}

impl Token {
    fn to_string(&self) -> String {
        format!("{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

fn main() {
    let mut args = env::args().into_iter();

    if args.len() > 2 {
        eprintln!("Usage: jlox [script]");
        process::exit(1);
    } else if args.len() == 2 {
        args.next();
        let file_path = args.next().unwrap();
        run_file(file_path);
    } else {
        run_prompt();
    }
}

fn run_file(_a: String) {
    let _contents = fs::read_to_string(_a);
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

fn run(_a: String) {
    let scanner = Scanner::build(_a);
    let tokens: Vec<Token> = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

fn error(line: u64, message: String) {
    report(line, "".to_string(), message);
}

fn report(line: u64, wher: String, message: String) {
    eprintln!("[line {}] Error{}: {}", line, wher, message);
}
