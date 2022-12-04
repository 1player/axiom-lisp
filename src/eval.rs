use crate::env::Env;
use crate::expr::Expr;

#[derive(Debug)]
pub enum EvalError {
    UndefinedSymbol(String),
    SyntaxError,
}

fn eval_list(first: Expr, rest: &[Expr], env: &mut Env) -> Result<Expr, EvalError> {
    match first {
        Expr::Symbol(s) if s == "define" => {
            if let [name_e, value_e] = rest {
                let name = name_e.expect_symbol().ok_or(EvalError::SyntaxError)?;
                env.set(name, value_e.clone());
                Ok(name_e.clone())
            } else {
                Err(EvalError::SyntaxError)
            }
        }
        _ => todo!(),
    }
}

pub fn eval(expr: Expr, env: &mut Env) -> Result<Expr, EvalError> {
    match expr {
        Expr::Symbol(s) => env
            .get(&s)
            .map(|e| e.clone())
            .ok_or(EvalError::UndefinedSymbol(s.to_owned())),
        Expr::List(ref exprs) => {
            if exprs.len() > 0 {
                eval_list(exprs[0].clone(), &exprs[1..], env)
            } else {
                // empty list
                Ok(expr)
            }
        }
    }
}
