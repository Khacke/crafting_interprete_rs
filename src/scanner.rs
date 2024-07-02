use crate::token::{Token, TokenType};
use crate::error::{Result, Error};

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self { source, tokens: vec![] ,start: 0, current: 0, line: 1 }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>> {
        while !self.is_at_end(){
            self.start = self.current;
            self.scan_token()?;
        }
        self.tokens.push(Token::new(TokenType:: Eof, "".to_owned(), "".to_owned(), self.line));
        Ok(&self.tokens)
    }

    fn scan_token(&mut self) -> Result<()> {
        let c = self.advance().ok_or(Error::ErrorUnexpectedCharacter(self.line))?;
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                self.increment();
                self.add_token(if self.match_char('=') {TokenType::BangEqual} else {TokenType::Bang}, None);
            },
            '=' => {
                self.increment();
                self.add_token(if self.match_char('=') {TokenType::EqualEqual} else {TokenType::Equal}, None);
            },
            '<' => {
                self.increment();
                self.add_token(if self.match_char('=') {TokenType::LessEqual} else {TokenType::Less}, None);
            }
            '>' => {
                self.increment();
                self.add_token(if self.match_char('=') {TokenType::GreaterEqual} else {TokenType::Greater}, None);
            }
            _   => return Err(Error::ErrorUnexpectedCharacter(self.line))
        }
        Ok(())
    }

    fn increment(&mut self) {
        self.current += 1;
    }

    fn advance(&mut self) -> Option<char> {
        self.increment();
        self.source.chars().nth(self.current)
    }

    fn add_token(&mut self, token: TokenType, literal: Option<String>) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token, text.to_owned(), literal.unwrap(), self.line));
    }

    fn match_char(&self, exp: char) -> bool {
        let mut iter = self.source.chars().skip(self.current).peekable();
        if let Some(c) = iter.peek() {
            if c.eq(&exp) {
                return true;
            }
        }
        false
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}