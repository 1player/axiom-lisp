use axiom::*;

fn repl() {
    let mut is_multiline = false;
    let mut buf = String::new();
    let mut env = create_toplevel();

    loop {
        if is_multiline {
            eprint!(". ");
            is_multiline = false;
        } else {
            eprint!("> ");
        }

        let bytes_read = std::io::stdin().read_line(&mut buf).unwrap();
        if bytes_read == 0 {
            break;
        }

        match read::read_string(&buf) {
            Ok(expr) => match eval::eval(&expr, &mut env) {
                Ok(expr) => eprintln!("{}", expr),
                Err(e) => eprintln!("error: {:?}", e),
            },
            Err(read::ReadError::UnexpectedEOF) => is_multiline = true,
            Err(read::ReadError::UnbalancedParen) => {
                eprintln!("error: Unbalanced closing parenthesis")
            }
            Err(read::ReadError::InvalidUTF8) => eprintln!("error: Invalid UTF-8 input"),
            Err(read::ReadError::IOError) => eprintln!("error: IO error reading input"),
            Err(read::ReadError::Empty) => (),
        }

        if !is_multiline {
            buf.clear();
        }
    }
}

fn main() {
    repl();
}
