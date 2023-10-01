use super::token::{Token, TokenType, Vars};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
}

#[derive(Debug)]

pub struct LexerIter<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new<T: Into<&'a str>>(input: T) -> Self {
        Lexer {
            input: input.into(),
        }
    }

    pub fn iter(&self) -> LexerIter {
        LexerIter {
            input: self.input,
            pos: 0,
        }
    }
}

impl<'a> Iterator for LexerIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.input.len() {
            return None;
        }

        let bytes = self.input.as_bytes();

        loop {
            self.trim();

            if self.pos >= self.input.len() {
                return None;
            }

            match bytes[self.pos] {
                b'a'..b'z' | b'A'..b'Z' | b'_' => return Some(self.find_ident()),
                b'"' | b'\'' => return Some(self.find_string()),
                _ => {
                    return Some(Token {
                        ty: TokenType::EOF,
                        literal: "".into(),
                    })
                }
            }
        }
    }
}

impl<'a> LexerIter<'a> {
    fn find_ident(&mut self) -> Token {
        let bytes = self.input.as_bytes();
        let mut res = String::new();

        loop {
            res.push(bytes[self.pos] as char);

            self.pos += 1;

            if self.pos >= self.input.len() {
                break;
            }

            match bytes[self.pos] {
                b'a'..b'z' | b'A'..b'Z' | b'_' => {}
                _ => break,
            }
        }

        if let Some(global_var) = Self::find_global_var(&res) {
            return global_var;
        }

        Token {
            ty: TokenType::Ident(res.clone()),
            literal: res,
        }
    }

    fn find_global_var(ident: &str) -> Option<Token> {
        match ident {
            "print" => Some(Token {
                ty: TokenType::GlobalVar(Vars::Print),
                literal: ident.into(),
            }),
            _ => None,
        }
    }

    fn find_string(&mut self) -> Token {
        let bytes = self.input.as_bytes();
        let start = bytes[self.pos];
        let mut res = String::new();

        self.pos += 1;

        loop {
            if self.pos >= self.input.len() {
                break;
            }

            match bytes[self.pos] {
                _end if _end == start => {
                    self.pos += 1;
                    break;
                }
                cr => res.push(cr as char),
            }

            self.pos += 1;
        }

        Token {
            ty: TokenType::String(res.clone()),
            literal: res,
        }
    }

    fn trim(&mut self) {
        let bytes = self.input.as_bytes();

        loop {
            if self.pos >= self.input.len() {
                break;
            }

            match bytes[self.pos] {
                b' ' | b'\t' | b'\n' => self.skip_one(),
                b';' => self.skip_one(),
                _ => break,
            }
        }
    }

    fn skip_one(&mut self) {
        self.pos += 1;
    }
}

#[cfg(test)]
mod test_lexer {
    use super::*;

    #[test]
    fn test_lua_print_hello_world() {
        let input = r#"print "hello, world""#;
        let lexer = Lexer::new(input);
        let res = [
            Token {
                ty: TokenType::GlobalVar(Vars::Print),
                literal: "print".into(),
            },
            Token {
                ty: TokenType::String("hello, world".into()),
                literal: "hello, world".into(),
            },
        ];

        let mut count = 0;
        for (i, token) in lexer.iter().enumerate() {
            count += 1;
            assert_eq!(res[i], token);
        }

        assert_eq!(count, res.len());
    }
}
