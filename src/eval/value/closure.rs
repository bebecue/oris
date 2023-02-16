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
        let mut captured = Vec::with_capacity(unbounded.len());
        for ident in unbounded {
            if let Some(value) = env.get(ident) {
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

fn analyze_unbounded(f: &ast::Closure) -> HashSet<&Ident> {
    let mut env = AnalyzeEnv {
        scopes: vec![f.parameters.iter().collect()],
        unbounded: Default::default(),
    };

    env.walk_block(&f.body);

    env.unbounded
}

struct AnalyzeEnv<'a> {
    scopes: Vec<HashSet<&'a Ident>>,
    unbounded: HashSet<&'a Ident>,
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
        self.scopes.last_mut().unwrap().insert(ident);
    }

    fn access_ident(&mut self, ident: &'a Ident) {
        if !self.has_ident(ident) {
            self.unbounded.insert(ident);
        }
    }

    fn has_ident(&self, ident: &Ident) -> bool {
        for scope in self.scopes.iter().rev() {
            if scope.contains(ident) {
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
            ast::Stmt::Return(expr) => {
                if let Some(ref expr) = expr {
                    self.walk_expr(expr)
                }
            }
            ast::Stmt::Let(ident, value) => {
                self.walk_expr(value);
                self.create_ident(ident);
            }
        }
    }

    fn walk_expr(&mut self, expr: &'a ast::Expr) {
        match expr {
            ast::Expr::Int(_) | ast::Expr::Bool(_) | ast::Expr::Str(_) | ast::Expr::Closure(_) => {}
            ast::Expr::Seq(seq) => {
                for expr in seq.iter() {
                    self.walk_expr(expr);
                }
            }
            ast::Expr::Ident(ident) => self.access_ident(ident),
            ast::Expr::Unary(_, expr) => self.walk_expr(expr),
            ast::Expr::Binary(left, _, right) => {
                self.walk_expr(left);
                self.walk_expr(right);
            }
            ast::Expr::If(expr) => {
                self.walk_expr(&expr.condition);
                self.walk_block(&expr.consequence);
                if let Some(ref alternative) = expr.alternative {
                    self.walk_block(alternative);
                }
            }
            ast::Expr::Call(target, args) => {
                self.walk_expr(target);

                for arg in args.iter() {
                    self.walk_expr(arg);
                }
            }
            ast::Expr::Map(entries) => {
                for (k, v) in entries.iter() {
                    self.walk_expr(k);
                    self.walk_expr(v);
                }
            }
            ast::Expr::Index(target, key) => {
                self.walk_expr(target);
                self.walk_expr(key);
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
    assert_eq!(parser.next(), None);

    match f {
        ast::Node::Expr(ast::Expr::Closure(closure)) => {
            assert_eq!(
                analyze_unbounded(&closure),
                HashSet::from([&Ident::new("x".into()), &Ident::new("z".into())])
            );
        }
        _ => unreachable!(),
    }
}
