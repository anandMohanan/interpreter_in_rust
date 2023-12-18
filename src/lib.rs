pub mod lexer;
pub mod repl;
pub mod ast;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod test {
    use crate::lexer::{lexer::Lexer, token::Token};

    #[test]
    fn test_lexer() {
        let input = "==+;";
        let lexer = Lexer::new(input);
        println!("{:#?}", lexer.into_iter().collect::<Vec<Token>>());
    }
    #[test]
    fn test_lexer_2() {
        let input = "let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        !-/*5;
            5 < 10 > 5;
            if (5 < 10) {
            return true;
            } else {
            return false;
            }
            10 == 10;
            10 != 9;";
        let lexer = Lexer::new(input);
        println!("{:#?}", lexer.into_iter().collect::<Vec<Token>>());
    }
}
