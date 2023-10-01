use crate::bytecode::ByteCode;

use super::{
    lexer::Lexer,
    token::{Token, TokenType, Vars},
};

#[derive(Debug)]
pub struct Parser;

#[derive(Debug)]
pub struct Prototype {
    pub constants: Vec<Vars>,
    pub bytecode: Vec<ByteCode>,
}

impl Parser {
    pub fn parse(input: &str) -> Prototype {
        let lexer = Lexer::new(input);

        let mut constants = vec![];
        let mut bytecode = vec![];

        let mut lexer_iter = lexer.iter();

        while let Some(Token { ty, .. }) = lexer_iter.next() {
            match ty {
                TokenType::GlobalVar(global_var) => match global_var {
                    Vars::Print => {
                        constants.push(Vars::Print);
                        bytecode.push(ByteCode::GetGlobal(0, (constants.len() - 1) as u8));

                        if let Some(Token {
                            ty: TokenType::String(val),
                            ..
                        }) = lexer_iter.next()
                        {
                            constants.push(Vars::General(val));
                            bytecode.push(ByteCode::LoadK(1, (constants.len() - 1) as u8));
                            bytecode.push(ByteCode::Call(0, 1));
                        } else {
                            panic!("not implement!");
                        }
                    }
                    Vars::General(_) => panic!("not implement!"),
                },
                TokenType::String(_) => panic!("not implement!"),
                TokenType::Ident(_) => panic!("not implement!"),
                TokenType::EOF => panic!("not implement!"),
            }
        }

        Prototype {
            constants: constants,
            bytecode: bytecode,
        }
    }
}
