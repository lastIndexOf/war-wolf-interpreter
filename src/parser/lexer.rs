use super::token::{Token, TokenType};

pub type Source = &'static str;

#[derive(Debug)]
pub struct Lexer {
    input: Source,
}

pub struct LexerIter {}

impl Lexer {
    pub fn new<T: Into<Source>>(input: T) -> Self {
        Lexer {
            input: input.into(),
        }
    }

    pub fn iter(&self) -> LexerIter {
        todo!()
    }
}

impl Iterator for LexerIter {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod test_lexer {
    use super::*;

    #[test]
    fn test_lexer_basic() {
        let input = r#"print "hello, world""#;
        let lexer = Lexer::new(input);
        let res = [
            Token {
                ty: TokenType::Name("print".into()),
                literal: "print".into(),
            },
            Token {
                ty: TokenType::String("hello, world".into()),
                literal: "hello, world".into(),
            },
        ];

        for (i, token) in lexer.iter().enumerate() {
            assert_eq!(res[i], token);
        }
    }
}
