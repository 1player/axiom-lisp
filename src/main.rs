mod env;
mod eval;
mod expr;
mod read;

fn repl() {
    let mut buf = String::new();
    let mut env = env::Env::new(None);

    loop {
        eprint!("> ");
        std::io::stdin().read_line(&mut buf).unwrap();

        match read::read(&buf) {
            Ok(expr) => match eval::eval(expr, &mut env) {
                Ok(expr) => eprintln!("{}", expr),
                Err(e) => eprintln!("error: {:?}", e),
            },
            Err(read::ParseError::UnexpectedEOF) => eprintln!("Unexpected EOF"),
        }

        buf.clear();
    }
}

fn main() {
    repl();
}
