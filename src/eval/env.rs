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

    pub(super) fn find_similar_symbol<'a>(&'a self, sym: &str) -> Option<&'a std::rc::Rc<str>> {
        self.frames
            .last()
            .and_then(|frame| find_closest_symbol(sym, frame.keys()))
            .or_else(|| find_closest_symbol(sym, self.global.keys()))
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

fn find_closest_symbol<'a, I>(
    undefined_symbol: &str,
    defined_symbols: I,
) -> Option<&'a std::rc::Rc<str>>
where
    I: Iterator<Item = &'a std::rc::Rc<str>>,
{
    defined_symbols
        .map(|defined| (defined, compute_edit_distance(undefined_symbol, defined)))
        .min_by_key(|(_, dis)| *dis)
        .map(|(defined, _)| defined)
}

fn compute_edit_distance(a: &str, b: &str) -> usize {
    let mut row = (0..b.chars().count() + 1).into_iter().collect::<Vec<_>>();

    for (ai, ac) in a.chars().enumerate() {
        let mut left = ai + 1;
        let mut diagonal = std::mem::replace(&mut row[0], left);

        for (bi, bc) in b.chars().enumerate() {
            left = if ac == bc {
                diagonal
            } else {
                1 + diagonal.min(row[bi]).min(row[bi + 1])
            };

            diagonal = std::mem::replace(&mut row[bi + 1], left);
        }
    }

    *row.last().unwrap()
}

#[test]
fn test_edit_distance() {
    assert_eq!(compute_edit_distance("kitten", "sitting"), 3);
    assert_eq!(compute_edit_distance("self", "self"), 0);
    assert_eq!(compute_edit_distance("some", ""), 4);
    assert_eq!(compute_edit_distance("", "none"), 4);
}
