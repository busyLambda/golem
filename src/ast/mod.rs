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
        Self {
            name,
            pos,
            context,
        }
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
    Func { name: Ident, params: Vec<FuncParam>, ret: Type},
    FuncCall { name: Ident, args: Vec<Expr>},
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
}
