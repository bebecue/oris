use crate::{eval::Value, parse::ast::Ident};

pub(crate) struct Env {
    global: Storage,
    scopes: Vec<Storage>,
    cached: Vec<Storage>,
}

type Storage = std::collections::HashMap<Ident, Value>;

impl Env {
    pub(crate) fn with_builtin() -> Self {
        let mut global = Storage::default();

        for (k, v) in crate::eval::value::builtin::all_() {
            global.insert(Ident::new(std::rc::Rc::from(k)), Value::Builtin(v));
        }

        Self {
            global,
            scopes: Default::default(),
            cached: Default::default(),
        }
    }

    pub(super) fn get(&self, ident: &Ident) -> Option<&Value> {
        self.scopes
            .last()
            .and_then(|scope| scope.get(ident))
            .or_else(|| self.global.get(ident))
    }

    pub(super) fn set(&mut self, ident: Ident, value: Value) {
        self.scopes
            .last_mut()
            .unwrap_or(&mut self.global)
            .insert(ident, value);
    }

    pub(super) fn enclosed<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Env) -> T,
    {
        let new = self.cached.pop().unwrap_or_default();
        self.scopes.push(new);

        let result = f(self);

        let mut old = self.scopes.pop().unwrap();
        old.clear();
        self.cached.push(old);

        result
    }
}
