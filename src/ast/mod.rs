#[derive(Debug)]
pub enum Context {
    Type,
    Var,
    Import,
}

#[derive(Debug)]
pub struct Ident {
    name: String,
    pos: (usize, usize),
    context: Context,
}

impl Ident {
    pub fn new(name: String, pos: (usize, usize), context: Context) -> Self {
        Self { name, pos, context }
    }
}

#[derive(Debug)]
pub struct FuncParam {
    name: Ident,
    d_type: Type,
}

#[derive(Debug)]
pub enum Expr {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Char(char),
    Var(Ident),
    FuncImpl {
        stmts: Box<Vec<Stmt>>,
        params: Vec<Ident>,
        ret: Type,
    },
    FuncCall {
        name: Ident,
        args: Vec<Expr>,
    },
}

#[derive(Debug)]
pub enum Type {
    Func {
        params: Box<Vec<Type>>,
        ret: Box<Type>,
    },
    Int,
    Float,
    String,
    Bool,
    Void,
    Char,
    Array(Box<Type>),
}

#[derive(Debug)]
pub enum Stmt {
    Decl {
        name: Ident,
        t_type: Type,
    },
    Asgn {
        name: Ident,
        expr: Expr,
    },
    DeAs {
        name: Ident,
        d_type: Option<Type>,
        expr: Expr,
    },
    Expr(Expr)
}

#[derive(Debug)]
pub struct File {
    name: String,
    stmts: Vec<Stmt>,
}

impl File {
    pub fn new(name: String, stmts: Vec<Stmt>) -> Self {
        Self {
            name,
            stmts,
        }
    }
}