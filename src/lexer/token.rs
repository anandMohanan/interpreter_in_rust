#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Let,
    Function,
    True,
    False,
    If,
    Else,
    Return,
    Equal,
    NotEqual,

    Illegal,
    Assign,
    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lsquirlybrace,
    Rsquirlybrace,
    Minus,

    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,

    Identifier(String),
    Int(usize),
}

pub static KEYWORDS: phf::Map<&'static str, Token> = phf::phf_map! {

 "true" => Token::True,
    "false" => Token::False,
    "fn" => Token::Function,
    "let" => Token::Let,
    "if" => Token::If,
    "else" => Token::Else,
    "return" => Token::Return,
};
