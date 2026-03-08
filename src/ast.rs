/// Type nodes in the language AST.
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// Signed 64-bit integer.
    Int,
    /// 64-bit floating-point number.
    Float,
    /// Boolean value.
    Bool,
    /// Named type (struct/enum/reference aliases).
    Named(String),
    /// Reference type, explicit via `&T`.
    Ref(Box<Type>),
    /// Homogeneous list type, implemented via `List<T>`.
    List(Box<Type>),
    /// Fixed-size tuple types, e.g. `(Int, Float)`.
    Tuple(Vec<Type>),
    /// Function type `fn(...) -> T`.
    Function(Vec<Type>, Box<Type>),
    /// Empty tuple-like return type.
    Unit,
}

/// Function parameters.
#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}

/// Expression AST nodes.
#[derive(Debug, Clone)]
pub enum Expr {
    /// Integer literal.
    Int(i64),
    /// Floating-point literal.
    Float(f64),
    /// Boolean literal.
    Bool(bool),
    /// Local variable reference.
    Local(String),
    /// Unary reference operator (`&expr`).
    Ref(Box<Expr>),
    /// Unary dereference operator (`*expr`).
    Deref(Box<Expr>),
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
    /// Struct construction (`Point { x: 1, y: 2 }`).
    StructInit {
        name: String,
        fields: Vec<(String, Expr)>,
    },
    /// Enum variant construction (`Status::Done`, `Status::Value(1)`).
    EnumInit {
        enum_name: String,
        variant: String,
        payload: Vec<Expr>,
    },
    /// Pattern matching (`match expr { ... }`).
    Match {
        subject: Box<Expr>,
        arms: Vec<MatchArm>,
    },
    /// List construction (`[1, 2, 3]`).
    ListLiteral(Vec<Expr>),
    /// Tuple construction (`(1, true, 3)`).
    TupleLiteral(Vec<Expr>),
    /// Indexed access (`expr[index]`).
    Index {
        base: Box<Expr>,
        index: Box<Expr>,
    },
    /// Method call (`expr.method(arg1, ...)`).
    MethodCall {
        receiver: Box<Expr>,
        name: String,
        args: Vec<Expr>,
    },
}

/// Match arm.
#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Expr,
}

/// Pattern used in match arms.
#[derive(Debug, Clone)]
pub enum Pattern {
    /// `A`
    UnitVariant(String),
    /// `A(x)`
    TupleVariant(String, Vec<String>),
    /// `A { x, y }`
    StructVariant(String, Vec<String>),
}

/// Binary operators.
#[derive(Debug, Clone)]
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

/// Statements in function bodies.
#[derive(Debug, Clone)]
pub enum Stmt {
    /// Local declaration with optional annotation and initializer.
    Let {
        name: String,
        ty: Option<Type>,
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

/// Struct declaration.
#[derive(Debug, Clone)]
pub struct StructDecl {
    pub name: String,
    pub fields: Vec<(String, Type)>,
}

/// Enum declaration.
#[derive(Debug, Clone)]
pub struct EnumDecl {
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

/// Enum variants.
#[derive(Debug, Clone)]
pub enum EnumVariant {
    /// `A`
    Unit(String),
    /// `A(T)`
    Tuple(String, Type),
    /// `A { x: T }`
    Struct(String, Vec<(String, Type)>),
}

/// Function declaration.
#[derive(Debug, Clone)]
pub struct FunctionDecl {
    /// Function name.
    pub name: String,
    /// Function parameter names and types.
    pub params: Vec<Param>,
    /// Function body statements.
    pub body: Vec<Stmt>,
    /// Function return type annotation.
    pub return_type: Type,
}

/// Top-level AST items.
#[derive(Debug, Clone)]
pub enum TopLevelDecl {
    Struct(StructDecl),
    Enum(EnumDecl),
    Function(FunctionDecl),
}

/// Root AST node for a whole source file.
#[derive(Debug, Clone)]
pub struct Program {
    pub items: Vec<TopLevelDecl>,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "Int"),
            Type::Float => write!(f, "Float"),
            Type::Bool => write!(f, "Bool"),
            Type::Named(name) => write!(f, "{}", name),
            Type::Ref(inner) => write!(f, "&{}", inner),
            Type::List(inner) => write!(f, "List<{}>", inner),
            Type::Tuple(elements) => {
                write!(f, "(")?;
                for (idx, elem) in elements.iter().enumerate() {
                    if idx > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", elem)?;
                }
                write!(f, ")")
            }
            Type::Function(params, ret) => {
                write!(f, "fn(")?;
                for (idx, p) in params.iter().enumerate() {
                    if idx > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ") -> {}", ret)
            }
            Type::Unit => write!(f, "Unit"),
        }
    }
}
