use std::collections::HashMap;

use crate::error::{Error, Result};
use crate::token::{Literal, Token, TokenType};

#[derive(Debug, Clone)]
pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    keywords: HashMap<String, TokenType>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert(String::from("and"), TokenType::And);
        keywords.insert(String::from("class"), TokenType::Class);
        keywords.insert(String::from("else"), TokenType::Else);
        keywords.insert(String::from("false"), TokenType::False);
        keywords.insert(String::from("for"), TokenType::For);
        keywords.insert(String::from("fun"), TokenType::Fun);
        keywords.insert(String::from("if"), TokenType::If);
        keywords.insert(String::from("nil"), TokenType::Nil);
        keywords.insert(String::from("or"), TokenType::Or);
        keywords.insert(String::from("print"), TokenType::Print);
        keywords.insert(String::from("return"), TokenType::Return);
        keywords.insert(String::from("super"), TokenType::Super);
        keywords.insert(String::from("this"), TokenType::This);
        keywords.insert(String::from("true"), TokenType::True);
        keywords.insert(String::from("var"), TokenType::Var);
        keywords.insert(String::from("while"), TokenType::While);
        Self {
            source,
            tokens: vec![],
            keywords,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_owned(),
            None,
            self.line,
        ));
        Ok(&self.tokens)
    }

    fn scan_token(&mut self) -> Result<()> {
        let c = self
            .advance()
            .ok_or(Error::UnexpectedCharacter(self.line))?;
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
            // multicharacter operators
            '!' => {
                let f = self.match_char('=');
                let tt = if f {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(tt, None);
                if self.tokens.last().unwrap().token_type == TokenType::BangEqual {
                    self.increment();
                }
            }
            '=' => {
                let f = self.match_char('=');
                let tt = if f {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(tt, None);
                if self.tokens.last().unwrap().token_type == TokenType::EqualEqual {
                    self.increment();
                }
            }
            '<' => {
                let f = self.match_char('=');
                let tt = if f {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(tt, None);
                if self.tokens.last().unwrap().token_type == TokenType::LessEqual {
                    self.increment();
                }
            }
            '>' => {
                let f = self.match_char('=');
                let tt = if f {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(tt, None);
                if self.tokens.last().unwrap().token_type == TokenType::GreaterEqual {
                    self.increment();
                }
            }
            '/' => {
                let f = self.match_char('/');
                let ff = self.match_char('*');
                if f {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if ff {
                    while self.peek() != '*' && self.peek_next() != '/' && !self.is_at_end() {
                        if self.peek() == '\n' {
                            self.line += 1;
                        }
                        self.advance();
                    }
                    self.advance();
                    self.advance();
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            // skip whitespace
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string()?,

            _ => {
                if c.is_ascii_digit() {
                    self.number()?;
                } else if c.is_alphanumeric() {
                    self.indentifier()?;
                } else {
                    return Err(Error::UnexpectedCharacter(self.line));
                }
            }
        }
        Ok(())
    }

    fn increment(&mut self) {
        self.current += 1;
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.current);
        self.increment();
        c
    }

    fn add_token(&mut self, token: TokenType, literal: Option<Literal>) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(
            token,
            text.to_owned(),
            literal,
            self.line,
        ));
    }

    fn match_char(&mut self, exp: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current) != Some(exp) {
            return false;
        }
        self.increment();
        true
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn string(&mut self) -> Result<()> {
        while !self.is_at_end() {
            match self.peek() {
                '"' => break,
                '\n' => self.line += 1,
                _ => {}
            }
            self.increment();
        }
        if self.is_at_end() {
            return Err(Error::UnterminatedString(self.line));
        }
        self.advance();

        let value: String = self.source.clone()[self.start + 1..self.current - 1].to_owned();
        self.add_token(TokenType::String, Some(Literal::Str(value)));

        Ok(())
    }

    fn number(&mut self) -> Result<()> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.add_token(
            TokenType::Number,
            Some(Literal::Number(self.source.clone()[self.start..self.current].parse::<f64>().unwrap())),
        );
        Ok(())
    }

    //TODO: not rust-y
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap() // safe to unwrap becuase if statement above it
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap() // safe to unwrap because if statement above it
    }

    fn indentifier(&mut self) -> Result<()> {
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let text: String = self.source[self.start..self.current].to_owned();
        let tt = self
            .keywords
            .get(&text)
            .unwrap_or(&TokenType::Identifier)
            .to_owned();
        self.add_token(tt, Some(Literal::Identifier(text)));
        Ok(())
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_tokens_single_characters() {
        let source = "(){},.-+;*".to_string();
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_tokens().unwrap();
        let expected_tokens = vec![
            Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
            Token::new(TokenType::RightParen, ")".to_string(), None, 1),
            Token::new(TokenType::LeftBrace, "{".to_string(), None, 1),
            Token::new(TokenType::RightBrace, "}".to_string(), None, 1),
            Token::new(TokenType::Comma, ",".to_string(), None, 1),
            Token::new(TokenType::Dot, ".".to_string(), None, 1),
            Token::new(TokenType::Minus, "-".to_string(), None, 1),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Star, "*".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_eq!(result, &expected_tokens);
    }

    #[test]
    fn test_scan_tokens_operators() {
        let source = "! != == = < <= > >=".to_string();
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_tokens().unwrap();
        let expected_tokens = vec![
            Token::new(TokenType::Bang, "!".to_string(), None, 1),
            Token::new(TokenType::BangEqual, "!=".to_string(), None, 1),
            Token::new(TokenType::EqualEqual, "==".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(TokenType::Less, "<".to_string(), None, 1),
            Token::new(TokenType::LessEqual, "<=".to_string(), None, 1),
            Token::new(TokenType::Greater, ">".to_string(), None, 1),
            Token::new(
                TokenType::GreaterEqual,
                ">=".to_string(),
                None,
                1,
            ),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_eq!(result, &expected_tokens);
    }

    #[test]
    fn test_scan_tokens_string() {
        let source = "\"hello world\"".to_string();
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_tokens().unwrap();
        let expected_tokens = vec![
            Token::new(
                TokenType::String,
                "\"hello world\"".to_string(),
                Some(Literal::Str("hello world".to_owned())),
                1,
            ),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_eq!(result, &expected_tokens);
    }

    #[test]
    fn test_scan_tokens_unterminated_string() {
        let source = "\"hello world".to_string();
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_tokens();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::UnterminatedString(1));
    }

    #[test]
    fn test_scan_tokens_number() {
        let source = "12345".to_string();
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_tokens().unwrap();
        let expected_tokens = vec![
            Token::new(
                TokenType::Number,
                "12345".to_string(),
                Some(Literal::Number(12345 as f64)),
                1,
            ),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_eq!(result, &expected_tokens);
    }

    #[test]
    fn test_scan_tokens_float_number() {
        let source = "123.45".to_string();
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_tokens().unwrap();
        let expected_tokens = vec![
            Token::new(
                TokenType::Number,
                "123.45".to_string(),
                Some(Literal::Number(123.45 as f64)),
                1,
            ),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_eq!(result, &expected_tokens);
    }

    #[test]
    fn test_scan_tokens_unexpected_character() {
        let source = "$".to_string();
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_tokens();
        assert_eq!(result, Err(Error::UnexpectedCharacter(1)));
    }

    #[test]
    fn test_scan_tokens_with_whitespace() {
        let source = "   ( ) { } ".to_string();
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_tokens().unwrap();
        let expected_tokens = vec![
            Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
            Token::new(TokenType::RightParen, ")".to_string(), None, 1),
            Token::new(TokenType::LeftBrace, "{".to_string(), None, 1),
            Token::new(TokenType::RightBrace, "}".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_eq!(result, &expected_tokens);
    }

    #[test]
    fn test_scan_tokens_multiple_lines() {
        let source = "(\n)".to_string();
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_tokens().unwrap();
        let expected_tokens = vec![
            Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
            Token::new(TokenType::RightParen, ")".to_string(), None, 2),
            Token::new(TokenType::Eof, "".to_string(), None, 2),
        ];
        assert_eq!(result, &expected_tokens);
    }

    #[test]
    fn test_scan_tokens_identifiers_and_keywords() {
        let scanner = Scanner::new("".to_string());
        let mut expected_tokens = vec![];

        // Add tokens for each keyword
        for (keyword, token_type) in &scanner.keywords {
            let token_text = keyword.to_string();
            expected_tokens.push(Token::new(
                token_type.clone(),
                token_text.clone(),
                Some(Literal::Identifier(token_text.clone())),
                1,
            ));
            expected_tokens.push(Token::new(
                TokenType::Eof,
                "".to_string(),
                None,
                1,
            ));

            let mut scanner = Scanner::new(token_text);
            let result = scanner.scan_tokens().unwrap();
            assert_eq!(result, &expected_tokens);
            expected_tokens.clear(); // Clear for the next iteration
        }
    }

    #[test]
    fn test_scan_tokens_single_line_comment() {
        let source = " // This is a single-line comment\n".to_string();
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_tokens().unwrap();

        let expected_tokens = vec![Token::new(
            TokenType::Eof,
            "".to_string(),
            None,
            2,
        )];

        assert_eq!(result, &expected_tokens);
    }

    #[test]
    fn test_scan_tokens_multiline_comments() {
        let source = "/* This is a comment */".to_string();
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_tokens().unwrap();

        let expected_tokens = vec![Token::new(
            TokenType::Eof,
            "".to_string(),
            None,
            1,
        )];

        assert_eq!(result, &expected_tokens);
    }
}
