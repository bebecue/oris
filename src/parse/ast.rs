use std::rc::Rc;

#[derive(Debug)]
pub(crate) enum Node {
    Expr(Expr),
    Stmt(Stmt),
}

#[derive(Debug)]
pub(crate) enum Stmt {
    // let <ident> = <expr>;
    Let(Let),

    // return;
    // return <expr>;
    Return(Return),
}

#[derive(Debug)]
pub(crate) struct Let {
    // position to the let keyword
    //
    // let <ident> = <expr>;
    // ^
    pub(crate) pos: usize,

    pub(crate) ident: Ident,

    pub(crate) value: Expr,
}

#[derive(Debug)]
pub(crate) struct Return {
    // position to the return keyword
    //
    // return;
    // ^
    //
    // return <expr>;
    // ^
    pub(crate) pos: usize,

    pub(crate) value: Option<Expr>,
}

#[derive(Debug)]
pub(crate) enum Expr {
    Int(Int),

    Bool(Bool),

    Str(Str),

    // `[<element>, ...]`
    Seq(Seq),

    // `{<key>: <value>, ...}`
    Map(Map),

    Ident(Ident),

    // `<base>[<subscript>]`
    Index(Box<Index>),

    // `<op> <expr>`
    Unary(Box<Unary>),

    // `<expr> <op> <expr>`
    Binary(Box<Binary>),

    // `fn() { <body> }`
    Closure(Rc<Closure>),

    // `<ident>(<arg>, ...)`
    Call(Box<Call>),

    // `if (<condition>) { <consequence> } else { <alternative> }`
    If(Box<If>),
}

#[derive(Debug)]
pub(crate) struct Int {
    // position to first digit
    //
    // 123
    // ^
    pub(crate) pos: usize,
    pub(crate) value: i32,
}

#[derive(Debug)]
pub(crate) struct Bool {
    // position to first character
    //
    // true
    // ^
    //
    // false
    // ^
    pub(crate) pos: usize,
    pub(crate) value: bool,
}

#[derive(Debug)]
pub(crate) struct Str {
    // position to left quotation mark
    //
    // "foobar"
    // ^
    pos: usize,
    value: Rc<str>,
}

#[derive(Debug)]
pub(crate) struct Seq {
    // position to left bracket token
    //
    // [<element>, ...]
    // ^
    pub(crate) pos: usize,
    pub(crate) elements: Box<[Expr]>,
}

#[derive(Debug)]
pub(crate) struct Map {
    // position to left brace token
    //
    // { <key>: <value> ... }
    // ^
    pub(crate) pos: usize,
    pub(crate) entries: Box<[(Expr, Expr)]>,
}

#[derive(Clone, Debug)]
pub(crate) struct Ident {
    // position to first character
    //
    // foobar
    // ^
    pos: usize,
    sym: Rc<str>,
}

#[derive(Debug)]
pub(crate) struct Index {
    // position to the left bracket token
    //
    // base[subscript]
    //     ^
    pub(crate) pos: usize,

    pub(crate) base: Expr,

    pub(crate) subscript: Expr,
}

#[derive(Debug)]
pub(crate) struct Unary {
    // position to the unary operator
    //
    // <op> <expr>
    // ^
    pub(crate) pos: usize,

    pub(crate) op: UnaryOp,

    pub(crate) value: Expr,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum UnaryOp {
    /// `-`
    Neg,

    /// `!`
    Not,
}

#[derive(Debug)]
pub(crate) struct Binary {
    // position to the binary operator
    //
    // <left> <op> <right>
    //        ^
    pub(crate) pos: usize,

    pub(crate) left: Expr,

    pub(crate) op: BinaryOp,

    pub(crate) right: Expr,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub(crate) struct Closure {
    // position to the fn keyword
    //
    // fn() { ... }
    // ^
    pub(crate) pos: usize,
    pub(crate) parameters: Box<[Ident]>,
    pub(crate) body: Block,
}

#[derive(Debug)]
pub(crate) struct Block {
    // position to the left brace
    //
    // { ... }
    // ^
    pub(crate) pos: usize,

    pub(crate) nodes: Box<[Node]>,
}

#[derive(Debug)]
pub(crate) struct Call {
    // position to the left parenthesis
    //
    // foo(<arg>...)
    //    ^
    pub(crate) pos: usize,
    pub(crate) target: Expr,
    pub(crate) args: Box<[Expr]>,
}

#[derive(Debug)]
pub(crate) struct If {
    // position to the if keyword
    //
    // if <expr> { ... }
    // ^
    pub(crate) pos: usize,

    pub(crate) condition: Expr,
    pub(crate) consequence: Block,
    pub(crate) alternative: Option<Block>,
}

impl Expr {
    pub(crate) fn pos(&self) -> usize {
        match self {
            Self::Int(expr) => expr.pos,
            Self::Bool(expr) => expr.pos,
            Self::Str(expr) => expr.pos,
            Self::Seq(expr) => expr.pos,
            Self::Map(expr) => expr.pos,
            Self::Ident(expr) => expr.pos,
            Self::Index(expr) => expr.pos,
            Self::Unary(expr) => expr.pos,
            Self::Binary(expr) => expr.pos,
            Self::Closure(expr) => expr.pos,
            Self::Call(expr) => expr.pos,
            Self::If(expr) => expr.pos,
        }
    }
}

impl Ident {
    pub(crate) fn from_str(pos: usize, sym: &str) -> Self {
        Self {
            pos,
            sym: sym.into(),
        }
    }

    #[cfg(test)]
    pub(crate) fn test(sym: &str) -> Self {
        Self {
            pos: 0,
            sym: sym.into(),
        }
    }

    pub(crate) fn pos(&self) -> usize {
        self.pos
    }

    pub(crate) fn sym(&self) -> &str {
        &self.sym
    }

    pub(crate) fn sym_rc_str(&self) -> &Rc<str> {
        &self.sym
    }
}

impl Str {
    pub(crate) fn from_src(pos: usize, value: &str) -> Self {
        Self {
            pos,
            value: value.into(),
        }
    }

    pub(crate) fn value(&self) -> &str {
        &self.value
    }

    pub(crate) fn value_rc_str(&self) -> &Rc<str> {
        &self.value
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.sym.fmt(f)
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
