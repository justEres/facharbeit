#[derive(Debug)]
pub enum Expr {
    Int(i64),
    Local(String),
    Binary {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug)]
pub enum Stmt {
    Let {
        name: String,
        value: Expr,
    },
    Return(Expr),
    Expr(Expr),
    If {
        cond: Expr,
        then_block: Vec<Stmt>,
        else_block: Vec<Stmt>,
    },
    While {
        cond: Expr,
        body: Vec<Stmt>,
    },
}

#[derive(Debug)]
pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
    pub return_type: Option<Type>,
}

#[derive(Debug)]
pub enum Type {
    Int,
}

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<FunctionDecl>,
}
