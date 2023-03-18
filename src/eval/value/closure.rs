use std::{collections::HashSet, rc::Rc};

use crate::{
    eval::Value,
    parse::ast::{self, Ident},
};

pub(crate) struct Closure {
    pub(crate) f: Rc<ast::Closure>,
    pub(crate) captured: Box<[(ast::Ident, Value)]>,
    pub(crate) undefined: Vec<ast::Ident>,
    pub(crate) recursive: Option<Ident>,
}

impl Closure {
    pub(crate) fn new(f: Rc<ast::Closure>, env: &crate::eval::Env) -> Self {
        let unbounded = analyze_unbounded(&f);

        let mut undefined = Vec::new();
        let mut captured = Vec::<(Ident, Value)>::with_capacity(unbounded.len());
        for ident in unbounded {
            if let Some(value) = env.get(ident.sym()) {
                captured.push((ident.clone(), value.clone()));
            } else {
                undefined.push(ident.clone());
            }
        }

        Self {
            f,
            captured: captured.into_boxed_slice(),
            undefined,
            recursive: None,
        }
    }
}

fn analyze_unbounded(f: &ast::Closure) -> Vec<&Ident> {
    let mut env = AnalyzeEnv {
        scopes: vec![f.parameters.iter().map(|ident| ident.sym()).collect()],
        unbounded: Default::default(),
    };

    env.walk_block(&f.body);

    env.unbounded
}

struct AnalyzeEnv<'a> {
    scopes: Vec<HashSet<&'a str>>,
    unbounded: Vec<&'a Ident>,
}

impl<'a> AnalyzeEnv<'a> {
    fn with<F>(&mut self, f: F)
    where
        F: FnOnce(&mut AnalyzeEnv<'a>),
    {
        self.scopes.push(Default::default());

        f(self);

        assert!(self.scopes.pop().is_some());
    }

    fn create_ident(&mut self, ident: &'a Ident) {
        self.scopes.last_mut().unwrap().insert(ident.sym());
    }

    fn access_ident(&mut self, ident: &'a Ident) {
        if !self.has_ident(ident) {
            self.unbounded.push(ident);
        }
    }

    fn has_ident(&self, ident: &Ident) -> bool {
        for scope in self.scopes.iter().rev() {
            if scope.contains(ident.sym()) {
                return true;
            }
        }

        false
    }
}

impl<'a> AnalyzeEnv<'a> {
    fn walk_block(&mut self, block: &'a ast::Block) {
        self.with(|env| {
            for node in block.nodes.iter() {
                match node {
                    ast::Node::Expr(expr) => env.walk_expr(expr),
                    ast::Node::Stmt(stmt) => env.walk_stmt(stmt),
                }
            }
        })
    }

    fn walk_stmt(&mut self, stmt: &'a ast::Stmt) {
        match stmt {
            ast::Stmt::Return(return_) => {
                if let Some(ref expr) = return_.value {
                    self.walk_expr(expr)
                }
            }
            ast::Stmt::Let(let_) => {
                self.walk_expr(&let_.value);
                self.create_ident(&let_.ident);
            }
        }
    }

    fn walk_expr(&mut self, expr: &'a ast::Expr) {
        match expr {
            ast::Expr::Int(_) | ast::Expr::Bool(_) | ast::Expr::Str(_) | ast::Expr::Closure(_) => {}
            ast::Expr::Seq(seq) => {
                for expr in seq.elements.iter() {
                    self.walk_expr(expr);
                }
            }
            ast::Expr::Ident(ident) => self.access_ident(ident),
            ast::Expr::Unary(expr) => self.walk_expr(&expr.value),
            ast::Expr::Binary(expr) => {
                self.walk_expr(&expr.left);
                self.walk_expr(&expr.right);
            }
            ast::Expr::If(expr) => {
                self.walk_expr(&expr.condition);
                self.walk_block(&expr.consequence);
                if let Some(ref alternative) = expr.alternative {
                    self.walk_block(alternative);
                }
            }
            ast::Expr::Call(call) => {
                self.walk_expr(&call.target);

                for arg in call.args.iter() {
                    self.walk_expr(arg);
                }
            }
            ast::Expr::Map(map) => {
                for (k, v) in map.entries.iter() {
                    self.walk_expr(k);
                    self.walk_expr(v);
                }
            }
            ast::Expr::Index(index) => {
                self.walk_expr(&index.base);
                self.walk_expr(&index.subscript);
            }
        }
    }
}

#[test]
fn unbounded() {
    let input = "
fn(y) {
    let a = 12;
    if y {
        let b = a + 3;
        z + b
    } else {
        x * 2
    }
}
";

    let lexer = crate::lex::Lexer::new(input.as_bytes().into());
    let mut parser = crate::parse::Parser::new(lexer);
    let f = parser.next().unwrap().unwrap();
    assert!(parser.next().is_none());

    match f {
        ast::Node::Expr(ast::Expr::Closure(closure)) => {
            let unbounded = analyze_unbounded(&closure);
            assert_eq!(unbounded.len(), 2);
            assert_eq!(unbounded[0].sym(), "z");
            assert_eq!(unbounded[1].sym(), "x");
        }
        _ => unreachable!(),
    }
}
