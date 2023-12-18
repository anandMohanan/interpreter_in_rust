use crate::lexer::{lexer::Lexer, token::Token};

use super::ast::{Program, Statement};

pub struct Parser<'a> {
    l: Lexer<'a>,
    cur_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser<'_> {
    pub fn new(l: Lexer) -> Parser {
        let mut p: Parser = Parser {
            l,
            cur_token: None,
            peek_token: None,
        };
        p.next_token();
        p.next_token();
        p
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next();
    }

    pub fn parse_program(&self) -> Option<Program> {
        // match self.cur_token {
        //     Some(ipsa) => match ipsa {
        //         Token::Illegal => None,
        //         // _ => {
        //             // let stmt = self.parse_program();
        //             // if let Some(val) = stmt {
        //             // let p = Program{
        //             // statements:
        //             // }
        //             // }
        //         }
        //     },
        //     None => {}
        // }
    }


    pub fn parse_statement(&self){ 
    
}
    } 

