use crate::scanner::{Token, TokenType};
use crate::expression::Expression;

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

    fn expression(self: &mut Self) -> Expression {
        self.equality()
    }

    fn equality(self: &mut Self) -> Expression {
        let mut expr = self.comparison();

        while self.match_token(TokenType::BangEqual, TokenType::EqualEqual) {
            expr = Expression::Binary{ left: Box::from(expr), operator: self.previous(), right: Box::from(self.comparison()) }; 
        };
        expr
    }

    fn comparison(self: &Self) -> Expression {
        todo!()
    }

    fn match_token(self: &Self, bang: TokenType, equal: TokenType) -> bool {
        todo!()
    }

    fn previous(self: &Self) -> Token {
        todo!()
    }
}