use crate::eval::{expect_args, expect_integer, expect_list, EvalError};
use crate::expr::Expr;

pub fn builtin_car(args: &[Expr]) -> Result<Expr, EvalError> {
    expect_args(args, 1)?;
    let list = expect_list(&args[0])?;

    Ok(list[0].clone())
}

pub fn builtin_cdr(args: &[Expr]) -> Result<Expr, EvalError> {
    expect_args(args, 1)?;
    let list = expect_list(&args[0])?;

    Ok(Expr::new_list(&list[1..]))
}

pub fn builtin_add(args: &[Expr]) -> Result<Expr, EvalError> {
    expect_args(args, 2)?;
    let x = expect_integer(&args[0])?;
    let y = expect_integer(&args[1])?;

    Ok(Expr::new_integer(x + y))
}

pub fn builtin_sub(args: &[Expr]) -> Result<Expr, EvalError> {
    expect_args(args, 2)?;
    let x = expect_integer(&args[0])?;
    let y = expect_integer(&args[1])?;

    Ok(Expr::new_integer(x - y))
}

pub fn builtin_mul(args: &[Expr]) -> Result<Expr, EvalError> {
    expect_args(args, 2)?;
    let x = expect_integer(&args[0])?;
    let y = expect_integer(&args[1])?;

    Ok(Expr::new_integer(x * y))
}

pub fn builtin_div(args: &[Expr]) -> Result<Expr, EvalError> {
    expect_args(args, 2)?;
    let x = expect_integer(&args[0])?;
    let y = expect_integer(&args[1])?;

    Ok(Expr::new_integer(x / y))
}
