#[derive(Debug, PartialEq)]
pub struct Token {
    pub ty: TokenType,
    pub literal: String,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Name(String),
    String(String),
}
