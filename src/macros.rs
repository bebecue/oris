macro_rules! propagate {
    ($eval:expr) => {
        match $eval {
            Ok($crate::eval::Eval::Continue(x)) => x,
            Ok($crate::eval::Eval::Return(b)) => return Ok($crate::eval::Eval::Return(b)),
            Err(err) => return Err(err),
        }
    };
}

macro_rules! args {
    ($args:ident @ $pos:ident = $($binding:ident),*) => {
        let [$($binding),*] = match <[$crate::eval::value::Value; args!(@count $($binding),*)]>::try_from($args) {
            Ok(xs) => { xs }
            Err(err) => {
                  return Err($crate::eval::Error::ArgCount {
                    pos: $pos,
                    supplied: err.len(),
                    expected: args!(@count $($binding),*),
                });
            }
        };
    };

    (@count $($binding:ident),*) => {
        0usize $( + args!(@subst $binding 1usize) )*
    };

    (@subst $from:tt $to:tt) => {
        $to
    };
}
