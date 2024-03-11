use super::{
    token::{Span, Token, TokenKind},
    Lexer,
};

impl Lexer {
    pub fn next(&mut self) -> Token {
        if self.is_eof() {
            return Token::new(TokenKind::EOF, Span::new(0, 0, String::new()));
        }

        let c = self.eat();

        let kind = match c {
            c if c.is_alphabetic() => self.ident_or_kw_or_type(),
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::ClosedParen,
            '{' => TokenKind::OpenCurly,
            '}' => TokenKind::ClosedCurly,
            ':' => self.col_or_coleq(),
            '=' => self.eq_or_eqeq(),
            '-' => self.right_arrow(),
            '[' => TokenKind::OpenBracket,
            ']' => TokenKind::ClosedBracket,
            c if c.is_whitespace() => self.whitespace(),
            _ => TokenKind::Unknown,
        };

        let span = self.span();

        let token = Token::new(kind, span);
        self.reset_pos_within_tok();

        token
    }

    fn ident_or_kw_or_type(&mut self) -> TokenKind {
        self.eat_while(is_ident_cont);

        match self.tok_str.as_str() {
            "do" => TokenKind::KwDo,
            "bool" => TokenKind::Tbool,
            "int" => TokenKind::Tint,
            "float" => TokenKind::Tfloat,
            "char" => TokenKind::Tchar,
            "string" => TokenKind::Tstring,
            _ => TokenKind::Identifier,
        }
    }

    fn col_or_coleq(&mut self) -> TokenKind {
        if let Some(c) = self.peek() {
            if c == '=' {
                self.eat();
                return TokenKind::Coleq;
            }
        }
        TokenKind::Column
    }

    fn eq_or_eqeq(&mut self) -> TokenKind {
        if let Some(c) = self.peek() {
            if c == '=' {
                self.eat();
                return TokenKind::EqEq;
            }
        }
        TokenKind::Eq
    }

    fn whitespace(&mut self) -> TokenKind {
        self.eat_while(is_whitespace);
        TokenKind::Whitespace
    }
    
    fn right_arrow(&mut self) -> TokenKind {
        if let Some(c) = self.peek() {
            if c == '>' {
                self.eat();
                return TokenKind::RightArrow;
            }
        }
        TokenKind::Unknown
    }
}

fn is_ident_cont(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}
