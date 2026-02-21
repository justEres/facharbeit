/// Expression nodes in the language AST.
#[derive(Debug)]
pub enum Expr {
    /// Integer literal.
    Int(i64),
    /// Read from a local variable.
    Local(String),
    /// Binary operation expression.
    Binary {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    /// Function call expression.
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

/// Binary operators supported by the parser and code generator.
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

/// Statement nodes in function bodies.
#[derive(Debug)]
pub enum Stmt {
    /// Local declaration with initializer.
    Let {
        name: String,
        value: Expr,
    },
    /// Return statement with optional value.
    Return(Option<Expr>),
    /// Standalone expression statement.
    Expr(Expr),
    /// Conditional branch.
    If {
        cond: Expr,
        then_block: Vec<Stmt>,
        else_block: Vec<Stmt>,
    },
    /// While loop.
    While {
        cond: Expr,
        body: Vec<Stmt>,
    },
}

/// Function declaration in the AST.
#[derive(Debug)]
pub struct FunctionDecl {
    /// Function name.
    pub name: String,
    /// Parameter names (currently all map to i64 in codegen).
    pub params: Vec<String>,
    /// Function body statements.
    pub body: Vec<Stmt>,
    /// Optional return type annotation.
    pub return_type: Option<Type>,
}

/// Type nodes currently supported by the language.
#[derive(Debug)]
pub enum Type {
    Int,
}

/// Root AST node for a whole source file.
#[derive(Debug)]
pub struct Program {
    pub functions: Vec<FunctionDecl>,
}
