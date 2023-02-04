use crate::env::Env;
use crate::expr::Expr;

#[derive(Debug)]
pub enum EvalError {
    UndefinedSymbol(String),
    SyntaxError,
    ArgumentError,
    TypeError,
}

pub fn eval(expr: &Expr, env: &mut Env) -> Result<Expr, EvalError> {
    match expr {
        Expr::Integer(_) => Ok(expr.clone()),
        Expr::Symbol(s) => eval_symbol(s, env),
        Expr::List(exprs) => eval_list(exprs, env),
    }
}

fn eval_symbol(s: &str, env: &mut Env) -> Result<Expr, EvalError> {
    env.get(&s).ok_or(EvalError::UndefinedSymbol(s.to_owned()))
}

fn eval_list(args: &[Expr], env: &mut Env) -> Result<Expr, EvalError> {
    let (op, rest) = args.split_at(1);

    if let Expr::Symbol(ref sym) = op[0] {
        match sym.as_str() {
            "define" => eval_define(rest, env),
            "quote" => eval_quote(rest, env),
            _ => Err(EvalError::SyntaxError),
        }
    } else {
        Err(EvalError::SyntaxError)
    }
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

fn expect_symbol(e: &Expr) -> Result<&String, EvalError> {
    if let Expr::Symbol(s) = e {
        Ok(s)
    } else {
        Err(EvalError::TypeError)
    }
}

fn expect_list(e: &Expr) -> Result<&Vec<Expr>, EvalError> {
    if let Expr::List(l) = e {
        Ok(l)
    } else {
        Err(EvalError::TypeError)
    }
}
