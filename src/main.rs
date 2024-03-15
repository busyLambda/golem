mod ast;
mod lexer;
mod parser;

fn main() {
    println!("Hello, world!");
}

mod test {
    use crate::{
        ast::File, lexer::{
            token::{Token, TokenKind},
            Lexer,
        }, parser::Parser
    };

    #[test]
    fn lexer() {
        //let mut lexer = Lexer::new("fn : ((int) -> int) -> [int]");
        let mut lexer = Lexer::new("fn : () -> int\nfn = () do 3\nmain : () -> void\nmain = () do 3");
        let mut tokens = Vec::<Token>::new();

        let mut i = 1;

        loop {
            let token = lexer.next();
            if token.kind().is_type() {
                println!("{i}: {:?} -> {}", token.kind(), token.literal());
            } else {
                println!("{i}: {:?}", token.kind());
            }

            tokens.push(token.clone());

            if token.kind() == TokenKind::EOF {
                break
            }

            i += 1;
        }

        let mut parser = Parser::new(tokens);
        let mut file = parser.file(String::from("testing")).unwrap();
        File::unify_funcs(file.stmts());
        println!("{:?}", file);
    }
}
