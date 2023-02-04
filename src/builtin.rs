use crate::eval::EvalError;
use crate::expr::Expr;

pub fn builtin_car(args: &[Expr]) -> Result<Expr, EvalError> {
    match args[0] {
        Expr::List(ref list) => Ok(list[0].clone()),
        _ => Err(EvalError::ArgumentError),
    }
}

pub fn builtin_cdr(args: &[Expr]) -> Result<Expr, EvalError> {
    match args[0] {
        Expr::List(ref list) => Ok(Expr::List(list.clone().split_off(1))),
        _ => Err(EvalError::ArgumentError),
    }
}
