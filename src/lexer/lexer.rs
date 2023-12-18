use std::{iter::Peekable, str::Chars};

use super::token::{Token, KEYWORDS};

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        Lexer {
            chars: input.chars().peekable(),
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn read_char(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn skip_whitespace(&mut self) {
        while let Some(_) = self.chars.next_if(|x| x.is_whitespace()) {}
    }

    fn keep_reading(&mut self, c: char, f: impl Fn(&char) -> bool) -> Vec<char> {
        let mut out = vec![c];
        while let Some(c) = self.chars.next_if(&f) {
            out.push(c)
        }
        out
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        loop {
            match self.read_char() {
                Some('*') => return Some(Token::Asterisk),
                Some('!') => {
                    if let Some(c) = self.peek() {
                        if *c == '=' {
                            self.read_char();
                            return Some(Token::NotEqual);
                        }
                    }
                    return Some(Token::Bang);
                }
                Some('/') => return Some(Token::Slash),
                Some('>') => return Some(Token::Gt),
                Some('<') => return Some(Token::Lt),
                Some('-') => return Some(Token::Minus),
                Some('+') => return Some(Token::Plus),
                Some(',') => return Some(Token::Comma),
                Some('=') => {
                    if let Some(c) = self.peek() {
                        if *c == '=' {
                            self.read_char();
                            return Some(Token::Equal);
                        }
                    }
                    return Some(Token::Assign);
                }
                Some(';') => return Some(Token::Semicolon),
                Some('(') => return Some(Token::Lparen),
                Some(')') => return Some(Token::Rparen),
                Some('{') => return Some(Token::Lsquirlybrace),
                Some('}') => return Some(Token::Rsquirlybrace),

                Some(c) if c.is_digit(10) => {
                    let str = self.keep_reading(c, |c| c.is_digit(10));
                    let str = str.into_iter().collect::<String>();
                    return Some(Token::Int(
                        str::parse::<usize>(&str).expect("this should always work"),
                    ));
                }

                Some(c) if c.is_ascii_alphabetic() => {
                    let ident = self.keep_reading(c, |c| c.is_ascii_alphabetic());
                    let ident = ident.into_iter().collect::<String>();

                    if let Some((_, v)) = KEYWORDS.get_entry(&ident) {
                        return Some(v.clone());
                    }
                    return Some(Token::Identifier(ident));
                }

                Some(_) => return Some(Token::Illegal),
                _ => return None,
            }
        }
    }
}
