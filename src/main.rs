mod expr;
mod read;

fn repl() {
    let mut buf = String::new();

    loop {
        eprint!("> ");
        std::io::stdin().read_line(&mut buf).unwrap();

        match read::read(&buf) {
            Ok(expr) => eprintln!("{:?}", expr),
            Err(read::ParseError::UnexpectedEOF) => eprintln!("Unexpected EOF"),
        }

        buf.clear();
    }
}

fn main() {
    repl();
}
