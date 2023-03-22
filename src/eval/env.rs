use crate::{eval::Value, parse::ast::Ident};

pub(crate) struct Env {
    global: Storage,
    frames: Vec<Storage>,
    cached: Vec<Storage>,
}

pub(crate) type Storage = std::collections::HashMap<std::rc::Rc<str>, (Option<usize>, Value)>;

impl Env {
    pub(crate) fn new(global: Storage) -> Self {
        Self {
            global,
            frames: Default::default(),
            cached: Default::default(),
        }
    }

    pub(super) fn get(&self, sym: &str) -> Option<&Value> {
        self.frames
            .last()
            .and_then(|frame| frame.get(sym))
            .or_else(|| self.global.get(sym)) // FIXME: filter out values in global but defined after the closure?
            .map(|(_, value)| value)
    }

    pub(super) fn set(&mut self, ident: Ident, value: Value) {
        self.frames
            .last_mut()
            .unwrap_or(&mut self.global)
            .insert(ident.sym_rc_str().clone(), (Some(ident.pos()), value));
    }

    pub(super) fn enclosed<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Env) -> T,
    {
        let new = self.cached.pop().unwrap_or_default();
        self.frames.push(new);

        let result = f(self);

        let mut old = self.frames.pop().unwrap();
        old.clear();
        self.cached.push(old);

        result
    }
}
