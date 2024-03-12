use core::fmt;

use crate::lexer::token::{Token, TokenKind};

pub mod parsing;

pub struct Parser {
    inp: Vec<Token>,
    pos: usize,
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEOF,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken(token) => write!(f, "Unexpected token: {:?}", token),
            ParseError::UnexpectedEOF => write!(f, "Unexpected EOF"),
        }
    }
}

type ParseResult<T> = Result<T, ParseError>;

impl Parser {
    pub fn new(inp: Vec<Token>) -> Self {
        Self { inp, pos: 0 }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.inp.len()
    }

    fn peek(&self) -> &Token {
        &self.inp[self.pos]
    }

    fn is_match(&self, kind: TokenKind) -> bool {
        !self.is_eof() && self.peek().kind() == kind
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    pub fn eaw(&mut self) {
        loop {
            if self.is_match(TokenKind::Whitespace) {
                self.advance();
            } else {
                break;
            }
        }
    }
}
