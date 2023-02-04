mod env;
mod eval;
mod expr;
mod read;

fn repl() {
    let mut buf = String::new();
    let mut env = env::Env::new(None);

    loop {
        eprint!("> ");
        let bytes_read = std::io::stdin().read_line(&mut buf).unwrap();

        if bytes_read == 0 {
            break;
        }

        match read::read(&buf) {
            Ok(expr) => match eval::eval(expr, &mut env) {
                Ok(expr) => eprintln!("{}", expr),
                Err(e) => eprintln!("error: {:?}", e),
            },
            Err(read::ParseError::UnexpectedEOF) => eprintln!("Unexpected EOF"),
            Err(read::ParseError::Empty) => (),
        }

        buf.clear();
    }
}

fn main() {
    repl();
}
