use crate::eval::{expect_args, expect_list, EvalError};
use crate::expr::Expr;

pub fn builtin_car(args: &[Expr]) -> Result<Expr, EvalError> {
    expect_args(args, 1)?;
    let list = expect_list(&args[0])?;

    Ok(list[0].clone())
}

pub fn builtin_cdr(args: &[Expr]) -> Result<Expr, EvalError> {
    expect_args(args, 1)?;
    let list = expect_list(&args[0])?;

    Ok(Expr::new_list(list.clone().split_off(1)))
}
