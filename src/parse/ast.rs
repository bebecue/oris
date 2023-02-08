use std::rc::Rc;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) enum Node {
    Expr(Expr),
    Stmt(Stmt),
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) enum Stmt {
    // let <ident> = <expr>;
    Let(Ident, Expr),

    // return;
    // return <expr>;
    Return(Option<Expr>),
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) enum Expr {
    Int(i32),

    Bool(bool),

    Str(Rc<str>),

    // `[<element>, ...]`
    Seq(Box<[Expr]>),

    // `{<key>: <value>, ...}`
    Map(Box<[(Expr, Expr)]>),

    Ident(Ident),

    // `<target>[<index>]`
    Index(Box<Expr>, Box<Expr>),

    // `<op> <expr>`
    Unary(UnaryOp, Box<Expr>),

    // `<expr> <op> <expr>`
    Binary(Box<Expr>, BinaryOp, Box<Expr>),

    // `fn() { <body> }`
    Closure(Rc<Closure>),

    // `<ident>(<arg>, ...)`
    Call(Box<Expr>, Box<[Expr]>),

    // `if (<condition>) { <consequence> } else { <alternative> }`
    If(Box<If>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Ident(Rc<str>);

#[derive(Clone, Copy, Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) enum UnaryOp {
    /// `-`
    Neg,

    /// `!`
    Not,
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) enum BinaryOp {
    // arithmetic
    Add,
    Sub,
    Mul,
    Div,

    // comparison
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) struct Closure {
    pub(crate) parameters: Box<[Ident]>,
    pub(crate) body: Block,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) struct Block {
    pub(crate) nodes: Box<[Node]>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) struct If {
    pub(crate) condition: Expr,
    pub(crate) consequence: Block,
    pub(crate) alternative: Option<Block>,
}

impl Ident {
    pub(crate) fn new(name: Rc<str>) -> Self {
        Self(name)
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;

        f.write_char(match self {
            Self::Neg => '-',
            Self::Not => '!',
        })
    }
}

impl std::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Lt => "<",
            Self::Le => "<=",
            Self::Gt => ">",
            Self::Ge => ">=",
            Self::Eq => "==",
            Self::Ne => "!=",
        })
    }
}
