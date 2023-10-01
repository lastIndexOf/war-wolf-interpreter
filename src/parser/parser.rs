use crate::{bytecode::ByteCode, common::GLOBAL_VAR_PRINT, value::Value};

use super::{
    lexer::Lexer,
    token::{GlobalVar, Token, TokenType},
};

#[derive(Debug)]
pub struct Parser;

#[derive(Debug)]
pub struct Prototype {
    pub constants: Vec<Value>,
    pub bytecode: Vec<ByteCode>,
}

impl Parser {
    pub fn parse(&self, input: &'static str) -> Prototype {
        let lexer = Lexer::new(input);

        let mut constants = vec![];
        let mut bytecode = vec![];

        let mut lexer_iter = lexer.iter();

        while let Some(Token { ty, .. }) = lexer_iter.next() {
            match ty {
                TokenType::GlobalVar(global_var) => match global_var {
                    GlobalVar::Print => {
                        constants.push(Value::String(GLOBAL_VAR_PRINT.into()));
                        bytecode.push(ByteCode::GetGlobal(0, (constants.len() - 1) as u8));

                        if let Some(Token {
                            ty: TokenType::String(val),
                            ..
                        }) = lexer_iter.next()
                        {
                            constants.push(Value::String(val));
                            bytecode.push(ByteCode::LoadK(1, (constants.len() - 1) as u8));
                            bytecode.push(ByteCode::Call(0, 1));
                        } else {
                            panic!("not implement!");
                        }
                    }
                    GlobalVar::General(_) => panic!("not implement!"),
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
