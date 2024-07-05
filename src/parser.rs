use crate::{
    error::{Error, Result},
    expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr},
    report,
    token::{Literal, Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn binary_helper<F>(&mut self, mut f: F, matchers: &[TokenType]) -> Result<Expr>
    where
        F: FnMut(&mut Self) -> Result<Expr>,
    {
        let mut expr = f(self)?;
        while self.match_type(matchers) {
            let operator = self.previous();
            let right = f(self)?;
            expr = Expr::Binary(BinaryExpr::new(expr, operator, right));
        }
        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expr> {
        let mut expr = self.equality()?;

        while self.match_type(&[TokenType::Comma]) {
            expr = self.equality()?;
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr> {
        self.binary_helper(
            |p| p.comparison(),
            &[TokenType::BangEqual, TokenType::EqualEqual],
        )
    }

    fn comparison(&mut self) -> Result<Expr> {
        self.binary_helper(
            |p| p.term(),
            &[
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
            ],
        )
    }

    fn match_type(&mut self, types: &[TokenType]) -> bool {
        for ttype in types {
            if self.check(ttype) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, ttype: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == ttype.to_owned()
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens
            .get(self.current)
            .expect("Tokens were accessed out of bounds")
            .to_owned()
    }

    fn previous(&self) -> Token {
        self.tokens
            .get(self.current - 1)
            .expect("Prev error")
            .to_owned()
    }

    fn term(&mut self) -> Result<Expr> {
        self.binary_helper(|p| p.factor(), &[TokenType::Plus, TokenType::Minus])
    }

    fn factor(&mut self) -> Result<Expr> {
        self.binary_helper(|p| p.unary(), &[TokenType::Slash, TokenType::Star])
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.match_type(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr::new(operator, right)));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr> {
        if self.match_type(&[TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr::new(Some(Literal::Identifier(
                "False".to_string(),
            )))));
        }
        if self.match_type(&[TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr::new(Some(Literal::Identifier(
                "True".to_string(),
            )))));
        }
        if self.match_type(&[TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr::new(Some(Literal::Identifier(
                "null".to_string(),
            )))));
        }
        if self.match_type(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(LiteralExpr::new(Some(
                self.previous().literal.unwrap(),
            ))));
        }

        if self.match_type(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect `)` after expression.");
            return Ok(Expr::Grouping(GroupingExpr::new(expr)));
        }

        let token = self.peek();
        report(format!("{:?} at end. Expect expression", token.line));
        return Err(Error::ParseError(format!(
            "{:?} at end. Expect Expression",
            token.line
        )));
    }

    fn consume(&mut self, ttype: TokenType, err_msg: &str) -> Result<Token> {
        if self.check(&ttype) {
            return Ok(self.advance());
        }

        let token = self.peek();
        if token.token_type == TokenType::Eof {
            report(format!("{:?} at end. {err_msg}", token.line));
            return Err(Error::ParseError(format!(
                "{:?} at end. {err_msg}",
                token.line
            )));
        }
        report(format!("{} at `{}`. {err_msg}", token.line, token.lexeme));
        Err(Error::ParseError(format!(
            "{} at `{}`. {err_msg}",
            token.line, token.lexeme
        )))
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }
            self.advance();
        }
    }

    pub fn parse(&mut self) -> Result<Expr> {
        self.expression()
    }
}
