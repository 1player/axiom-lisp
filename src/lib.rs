pub mod builtin;
pub mod env;
pub mod eval;
pub mod expr;
pub mod read;

pub fn create_toplevel() -> env::Env {
    let mut env = env::Env::new(None);

    use builtin::*;

    env.set("car", expr::Expr::Builtin(("car", builtin_car)));
    env.set("cdr", expr::Expr::Builtin(("cdr", builtin_cdr)));

    env.set("+", expr::Expr::Builtin(("+", builtin_add)));
    env.set("-", expr::Expr::Builtin(("-", builtin_sub)));
    env.set("/", expr::Expr::Builtin(("/", builtin_div)));
    env.set("*", expr::Expr::Builtin(("*", builtin_mul)));

    env
}
