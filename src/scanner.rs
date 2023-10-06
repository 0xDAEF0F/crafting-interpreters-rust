use crate::token::{Token, TokenType};
use std::collections::HashMap;

pub struct Scanner {
    tokens: Vec<Token>,
    source: String,
    start: usize,
    current: usize,
    line: usize,
    hm: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(str: String) -> Self {
        let hm: HashMap<String, TokenType> = vec![
            ("and".to_string(), TokenType::And),
            ("class".to_string(), TokenType::Class),
            ("else".to_string(), TokenType::Else),
            ("false".to_string(), TokenType::False),
            ("for".to_string(), TokenType::For),
            ("fun".to_string(), TokenType::Fun),
            ("if".to_string(), TokenType::If),
            ("nil".to_string(), TokenType::Nil),
            ("or".to_string(), TokenType::Or),
            ("print".to_string(), TokenType::Print),
            ("return".to_string(), TokenType::Return),
            ("super".to_string(), TokenType::Super),
            ("this".to_string(), TokenType::This),
            ("true".to_string(), TokenType::True),
            ("var".to_string(), TokenType::Var),
            ("while".to_string(), TokenType::While),
        ]
        .into_iter()
        .collect();

        Scanner {
            source: str,
            hm,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), self.line));

        self.tokens
    }

    fn scan_token(&mut self) {
        let next_char = &self.advance();

        match next_char {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => self.string(),
            c => {
                if c.is_digit(10) {
                    self.number();
                } else if c.is_alphabetic() || *c == '_' {
                    self.identifier();
                } else {
                    panic!("Unexpected character: {c}.");
                }
            }
        }
    }

    // NOTE: Not sure about the literal stuff
    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = &self.source[self.start..self.current];
        let token = Token::new(token_type, lexeme.to_string(), self.line);
        self.tokens.push(token)
    }

    fn match_char(&mut self, ch: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        let next_char = self.source.chars().nth(self.current).unwrap();

        if next_char != ch {
            return false;
        }

        self.current += 1;

        true
    }

    fn is_at_end(&self) -> bool {
        let is_end = self.current >= self.source.len() - 1;
        is_end
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            let a = self.source.chars().nth(self.current).unwrap();
            a
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() - 1 {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    fn advance(&mut self) -> char {
        let current = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        current
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        // TODO: Handle error
        if self.is_at_end() {
            panic!("unterminated string.")
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];

        self.add_token(TokenType::String(value.to_string()));
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let num = &self.source[self.start..self.current]
            .parse::<f64>()
            .unwrap();

        self.add_token(TokenType::Number(*num))
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text = &self.source[self.start..self.current];

        if let Some(token_type) = self.hm.get(text) {
            self.add_token(token_type.clone());
        } else {
            self.add_token(TokenType::Identifier);
        }
    }
}
