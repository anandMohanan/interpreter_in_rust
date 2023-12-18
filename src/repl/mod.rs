use crate::lexer::{lexer::Lexer, token::Token};

pub struct Repl {}

impl Repl {
    pub fn new() -> Repl {
        Repl {}
    }

    pub fn line(&self, line: &str) -> Vec<Token> {
        let lexer = Lexer::new(line);
        let mut out = vec![];
        for token in lexer.into_iter() {
            out.push(token)
        }
        out
    }
}
