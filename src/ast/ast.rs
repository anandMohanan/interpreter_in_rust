use crate::lexer::token::Token;

pub trait Node {
    fn token_literal(&self) -> Token;
}

pub trait Statement: Node {
    fn statement_node(&self);
}
pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

pub struct Identifier<'a> {
    value: &'a str,
    token: Token,
}

impl Node for Identifier<'_> {
    fn token_literal(&self) -> Token {
        self.token.clone()
    }
}

impl Statement for Identifier<'_> {
    fn statement_node(&self) {}
}
pub struct Let<'a> {
    token: Token,
    value: Box<dyn Expression>,
    name: Box<Identifier<'a>>,
}

impl Node for Let<'_> {
    fn token_literal(&self) -> Token {
        self.token.clone()
    }
}

impl Statement for Let<'_> {
    fn statement_node(&self) {}
}
