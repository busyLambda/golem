use crate::{
    ast::{Context, Expr, File, Ident, Stmt, Type},
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
            },
            TokenKind::Tvoid => {
                self.advance();
                Ok(Type::Void)
            }
            TokenKind::OpenBracket => self.array_type(),
            TokenKind::OpenParen => self.func_type(),
            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }
    
    fn func_body(&mut self) -> ParseResult<Vec<Stmt>> {
        let token = self.peek().clone();
        let kind = token.kind();
        
        let mut stmts = Vec::<Stmt>::new();

        match kind {
            TokenKind::KwDo => {
                self.advance();
                self.eaw();
                
                let stmt = self.stmt()?;
                
                stmts.push(stmt);

                self.eaw();
                
                Ok(stmts)
            }
            TokenKind::OpenCurly => {
                todo!()
            }
            _ => Err(ParseError::UnexpectedToken(token))
        }
    }

    fn func_expr(&mut self) -> ParseResult<Expr> {
        if self.is_match(TokenKind::OpenParen) {
            self.advance();
            self.eaw();

            let mut params = Vec::<Ident>::new();
            let mut stmts = Box::new(Vec::<Stmt>::new());

            loop {
                let ttoken = self.peek().clone();
                let kind = ttoken.kind();

                match kind {
                    TokenKind::Identifier => {
                        self.advance();
                        self.eaw();

                        let ident = Ident::new(ttoken.literal(), ttoken.pos(), Context::Var);
                        params.push(ident);
                    }
                    TokenKind::ClosedParen => {
                        self.advance();
                        self.eaw();

                        stmts = Box::new(self.func_body()?);

                        break;
                    }
                    _ => return { Err(ParseError::UnexpectedToken(ttoken)) },
                }
            }

            Ok(Expr::FuncImpl {
                stmts,
                params,
                ret: Type::Int,
            })
        } else {
            todo!()
        }
    }

    fn expr(&mut self) -> ParseResult<Expr> {
        let token = self.peek().clone();
        let kind = token.kind();

        match kind {
            TokenKind::Integer => {
                let value = match token.literal().parse::<i64>() {
                    Ok(v) => v,
                    Err(_) => return Err(ParseError::UnexpectedToken(token)),
                };
                self.advance();
                Ok(Expr::Int(value))
            }
            TokenKind::OpenParen => self.func_expr(),
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
                let name = Ident::new(ident.literal(), ident.pos(), Context::Var);

                Ok(Stmt::Decl { name, t_type })
            }
            // ASGN = IDENT ~ = ~ EXPR
            TokenKind::Eq => {
                self.advance();
                self.eaw();

                let name = Ident::new(ident.literal(), ident.pos(), Context::Var);
                let expr = self.expr()?;

                Ok(Stmt::Asgn { name, expr })
            }
            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }

    fn stmt(&mut self) -> ParseResult<Stmt> {
        let token = self.peek().clone();
        let kind = token.kind();

        match kind {
            TokenKind::Identifier => {
                self.advance();
                self.eaw();
                self.asgn_or_decl(token)
            }
            TokenKind::Integer => {
                Ok(Stmt::Expr(self.expr()?))
            }
            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }

    // FILE = { ~ STMT ~ }* EOF
    pub fn file(&mut self, name: String) -> ParseResult<File> {
        let mut stmts = Vec::<Stmt>::new();

        loop {
            self.eaw();

            if self.is_match(TokenKind::EOF) {
                break;
            }

            let stmt = self.stmt()?;

            stmts.push(stmt);
        }

        Ok(File::new(name, stmts))
    }
}
