mod lexer;
mod ast;
mod parser;

fn main() {
    println!("Hello, world!");
}

mod test {
    use crate::{lexer::{
        token::{Token, TokenKind},
        Lexer,
    }, parser::Parser};

    #[test]
    fn lexer() {
        let mut lexer = Lexer::new("fn : ((int) -> int) -> [int]");
        let mut tokens = Vec::<Token>::new();
        
        let mut i = 1;

        loop {
            let token = lexer.next();
            if token.kind() == TokenKind::EOF {
                break;
            } else if token.kind().is_type() {
                println!("{i}: {:?} -> {}", token.kind(), token.literal());
            } else {
                println!("{i}: {:?}", token.kind());
            }
            
            tokens.push(token);
            i+=1;
        }
        
        let mut parser = Parser::new(tokens);
        let func = parser.decl().unwrap();
        println!("{:?}", func);
    }
}
