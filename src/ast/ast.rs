pub enum Node {
    TokenLiteral(String),
}

pub enum Statement {
    Node,
    StatementNode,
}
pub enum Expression {
    Node,
    ExpressionNode,
}

pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    fn token_literal(&mut self) -> &str {
        if self.statements.len() > 0 {
            self.statements[0]
        }
    }
}
