use crate::{
    ast::{Context, Expr, Ident, Stmt, Type},
    lexer::token::{self, Token, TokenKind},
};

use super::{ParseError, ParseResult, Parser};

impl Parser {
    // RET_TYPE = -> ~ Type
    fn ret_t(&mut self) -> ParseResult<Type> {
        let token = self.peek().clone();
        let kind = token.kind();

        match kind {
            TokenKind::RightArrow => {
                self.advance();
                self.eaw();

                let ret_t = self.t_type()?;

                Ok(ret_t)
            }
            TokenKind::OpenCurly | TokenKind::KwDo => Ok(Type::Void),
            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }

    // ARRAY_TYPE = [ ~ Type ~ ]
    fn array_type(&mut self) -> ParseResult<Type> {
        let token = self.peek().clone();
        let kind = token.kind();

        if kind == TokenKind::OpenBracket {
            self.advance();
            self.eaw();

            let t_type = self.t_type()?;

            self.eaw();

            let token = self.peek().clone();
            let kind = token.kind();

            if kind != TokenKind::ClosedBracket {
                return Err(ParseError::UnexpectedToken(token));
            }

            self.advance();

            Ok(Type::Array(Box::new(t_type)))
        } else {
            Err(ParseError::UnexpectedToken(token))
        }
    }

    fn decl_params(&mut self) -> ParseResult<Vec<Type>> {
        let mut params = Vec::<Type>::new();

        loop {
            let token = self.peek().clone();
            let kind = token.kind();

            if kind == TokenKind::ClosedParen {
                break;
            }

            let t_type = self.t_type()?;

            self.eaw();

            params.push(t_type);
        }

        Ok(params)
    }

    fn func_type(&mut self) -> ParseResult<Type> {
        let mut params = Vec::<Type>::new();
        let mut ret: Type = Type::Void;

        if self.is_match(TokenKind::OpenParen) {
            self.advance();
            self.eaw();
            params = self.decl_params()?;
            self.advance();

            self.eaw();
            ret = self.ret_t()?;
        }

        Ok(Type::Func {
            params: Box::new(params),
            ret: Box::new(ret),
        })
    }

    fn t_type(&mut self) -> ParseResult<Type> {
        let token = self.peek().clone();
        let kind = token.kind();

        match kind {
            TokenKind::Tint => {
                self.advance();
                Ok(Type::Int)
            }
            TokenKind::OpenBracket => self.array_type(),
            TokenKind::OpenParen => self.func_type(),
            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }

    fn expr(&mut self) -> ParseResult<Expr> {
        let token = self.peek().clone();
        let kind = token.kind();

        match kind {
            TokenKind::Integer => {
                let value = match token.literal().parse::<i64>() {
                    Ok(v) => v,
                    Err(e) => return Err(ParseError::UnexpectedToken(token)),
                };
                Ok(Expr::Int(value))
            }
            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }

    // ASGN | DECL
    fn asgn_or_decl(&mut self, ident: Token) -> ParseResult<Stmt> {
        let token = self.peek().clone();
        let kind = token.kind();

        match kind {
            // DECL = IDENT ~ : ~ TYPE
            TokenKind::Column => {
                self.advance();
                self.eaw();

                let t_type = self.t_type()?;
                let name = Ident::new(token.literal(), token.pos(), Context::Var);

                Ok(Stmt::Decl { name, t_type })
            }
            // ASGN = IDENT ~ = ~ EXPR
            TokenKind::Eq => {
                self.advance();
                self.eaw();

                let name = Ident::new(token.literal(), token.pos(), Context::Var);
                let expr = self.expr()?;

                Ok(Stmt::Asgn { name, expr })
            }
            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }

    pub fn stmt(&mut self) -> ParseResult<Stmt> {
        let token = self.peek().clone();
        let kind = token.kind();

        match kind {
            TokenKind::Identifier => {
                self.advance();
                self.eaw();
                self.asgn_or_decl(token)
            }
            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }
}
