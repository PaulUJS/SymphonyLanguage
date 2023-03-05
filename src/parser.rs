use crate::scanner::{Token, TokenType::*, TokenType};
use crate::expression::{Expression};

macro_rules! matchtokens {
    ($parser:ident, $($token:ident),+) => {{
        let mut result = false;
        $(
            result |= $parser.match_token($token);
        )*
        result
    }}
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            current: 0,
        }
    }

    fn expression(&mut self) -> Expression {
        self.equality()
    }

    fn equality(&mut self) -> Expression {
        let mut expr = self.comparison();

        while matchtokens!(self, BangEqual, EqualEqual) {
            expr = Expression::Binary{ left: Box::from(expr), operator: self.previous(), right: Box::from(self.comparison()) }; 
        };
        expr
    }

    fn comparison(&self) -> Expression {
        todo!()
    }

    pub fn match_token(&self, t: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == t
        }
    }

    fn advance(&mut self) -> Token {
        if self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == Eof
    }

}