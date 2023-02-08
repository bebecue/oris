#[macro_use]
mod macros;

mod eval;
mod lex;
mod parse;

fn main() {
    if let Some(file) = std::env::args().nth(1) {
        run(&file);
    } else {
        repl();
    }
}

fn run(file: &str) {
    let code = match std::fs::read(file) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("can't open file '{}': {}", file, err);
            std::process::exit(1);
        }
    };

    let mut env = eval::Env::with_builtin();

    match eval::entry(&mut env, code) {
        Ok(eval::Value::Unit) => {}
        Ok(other) => {
            println!("{:?}", other);
        }
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    }
}

fn repl() {
    let mut env = eval::Env::with_builtin();

    let mut stdin = std::io::stdin().lock();
    loop {
        print!(">> ");
        use std::io::Write;
        std::io::stdout().flush().unwrap();

        let mut line = String::new();

        use std::io::BufRead;
        stdin.read_line(&mut line).unwrap();
        if line.is_empty() {
            break;
        }

        match eval::entry(&mut env, line.into_bytes()) {
            Ok(result) => {
                println!("{:?}", result);
            }
            Err(err) => {
                eprintln!("error: {}", err);
            }
        }
    }
}
