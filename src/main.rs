mod builtin;
mod env;
mod eval;
mod expr;
mod read;

use builtin::*;

fn create_toplevel() -> env::Env {
    let mut env = env::Env::new(None);

    env.set("car", expr::Expr::Builtin(("car", builtin_car)));
    env.set("cdr", expr::Expr::Builtin(("cdr", builtin_cdr)));

    env.set("+", expr::Expr::Builtin(("+", builtin_add)));
    env.set("-", expr::Expr::Builtin(("-", builtin_sub)));
    env.set("/", expr::Expr::Builtin(("/", builtin_div)));
    env.set("*", expr::Expr::Builtin(("*", builtin_mul)));

    env
}

fn repl() {
    let mut buf = String::new();
    let mut env = create_toplevel();

    loop {
        eprint!("> ");
        let bytes_read = std::io::stdin().read_line(&mut buf).unwrap();

        if bytes_read == 0 {
            break;
        }

        match read::read(&buf) {
            Ok(expr) => match eval::eval(&expr, &mut env) {
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
