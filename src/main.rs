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

    let mut env = oris::Env::new();

    match oris::entry(&mut env, &code) {
        Ok(result) => {
            if !result.is_unit() {
                println!("{:?}", result);
            }
        }
        Err(err) => {
            let (line, column) = err.line_column(&code);
            eprintln!("[error] {}\n  at {}:{}:{}", err, file, line + 1, column + 1);
            std::process::exit(1);
        }
    }
}

fn repl() {
    let mut env = oris::Env::new();

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

        match oris::entry(&mut env, line.as_bytes()) {
            Ok(result) => {
                println!("{:?}", result);
            }
            Err(err) => {
                eprintln!("error: {}", err);
            }
        }
    }
}
