#[derive(Debug, Clone)]
pub enum Context {
    Type,
    Var,
    Import,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct FuncParam {
    name: Ident,
    d_type: Type,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
    Expr(Expr),
    Func {
        name: Ident,
        params: Vec<(Ident, Type)>,
        body: Box<Vec<Stmt>>,
        ret: Type,
    },
}

#[derive(Debug, Clone)]
pub struct File {
    name: String,
    stmts: Vec<Stmt>,
}

impl File {
    pub fn new(name: String, stmts: Vec<Stmt>) -> Self {
        Self { name, stmts }
    }
    
    pub fn stmts(&mut self) -> &mut Vec<Stmt> {
        &mut self.stmts
    }

    pub fn unify_funcs(stmts: &mut Vec<Stmt>) {
        let mut i = 0;
        while i < stmts.len() {
            if let Stmt::Decl { name, t_type } = &stmts[i] {
                if let Type::Func { params, ret } = t_type {
                    if let Some(Stmt::Asgn {
                        name: func_name,
                        expr:
                            Expr::FuncImpl {
                                stmts: func_stmts,
                                params: func_params,
                                ret: func_ret,
                            },
                    }) = stmts.get(i + 1)
                    {
                        let mut ps = Vec::<(Ident, Type)>::new();

                        let mut j = 0;

                        while j < params.len() {
                            let t = params[j].clone();
                            let n = func_params[j].clone();

                            let ntp = (n, t);

                            ps.push(ntp);

                            j += 1;
                        }

                        let func = Stmt::Func {
                            name: func_name.clone(),
                            params: ps,
                            body: func_stmts.clone(),
                            ret: func_ret.clone(),
                        };
                        stmts[i] = func;
                        stmts.remove(i + 1);
                    }
                }
            }
            i += 1;
        }
    }
}
