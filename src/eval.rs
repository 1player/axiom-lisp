use crate::env::Env;
use crate::expr::Expr;

#[derive(Debug)]
pub enum EvalError {
    UndefinedSymbol(String),
    ArgumentError,
    TypeError,
}

pub fn eval(expr: &Expr, env: &mut Env) -> Result<Expr, EvalError> {
    match expr {
        Expr::Symbol(s) => eval_symbol(s, env),
        Expr::List(exprs) => eval_list(exprs, env),
        _ => Ok(expr.clone()),
    }
}

fn eval_symbol(s: &str, env: &mut Env) -> Result<Expr, EvalError> {
    env.get(&s).ok_or(EvalError::UndefinedSymbol(s.to_owned()))
}

fn eval_list(args: &[Expr], env: &mut Env) -> Result<Expr, EvalError> {
    let (op, rest) = args.split_at(1);
    let op = &op[0];

    if let Expr::Symbol(ref sym) = op {
        match sym.as_str() {
            "define" => return eval_define(rest, env),
            "quote" => return eval_quote(rest, env),
            _ => (),
        }
    }

    // Evaluate all arguments
    let op = eval(op, env)?;
    let rest = rest
        .iter()
        .map(|arg| eval(arg, env))
        .collect::<Result<Vec<_>, EvalError>>()?;

    apply(&op, &rest)
}

fn eval_define(args: &[Expr], env: &mut Env) -> Result<Expr, EvalError> {
    if let [ref name, ref binding] = args[..] {
        let val = eval(binding, env)?;
        env.set(&expect_symbol(name)?, val);
        Ok(name.clone())
    } else {
        Err(EvalError::ArgumentError)
    }
}

fn eval_quote(args: &[Expr], _env: &mut Env) -> Result<Expr, EvalError> {
    if let [ref arg] = args[..] {
        Ok(arg.clone())
    } else {
        Err(EvalError::ArgumentError)
    }
}

fn apply(op: &Expr, args: &[Expr]) -> Result<Expr, EvalError> {
    if let Expr::Builtin((_, fun)) = op {
        fun(args)
    } else {
        Err(EvalError::TypeError)
    }
}

fn expect_symbol(e: &Expr) -> Result<&String, EvalError> {
    if let Expr::Symbol(s) = e {
        Ok(s)
    } else {
        Err(EvalError::TypeError)
    }
}
