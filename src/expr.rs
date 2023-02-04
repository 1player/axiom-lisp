use crate::eval::EvalError;

type BuiltinFn = fn(&[Expr]) -> Result<Expr, EvalError>;

#[derive(Clone)]
pub enum Expr {
    Builtin((&'static str, BuiltinFn)),
    Integer(isize),
    Symbol(String),
    List(Vec<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Builtin((name, _)) => write!(f, "builtin:{}", name),
            Expr::Symbol(s) => write!(f, "{}", s),
            Expr::Integer(n) => write!(f, "{}", n),
            Expr::List(exprs) => {
                let inner: Vec<_> = exprs.iter().map(|e| format!("{}", e)).collect();
                write!(f, "({})", inner.join(" "))
            }
        }
    }
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Builtin((name, _)) => write!(f, "Builtin({:?})", name),
            Expr::Symbol(s) => write!(f, "Symbol({:?})", s),
            Expr::Integer(n) => write!(f, "Integer({:?})", n),
            Expr::List(exprs) => {
                write!(f, "List({:?})", exprs)
            }
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Expr::Builtin((ref a, _)), &Expr::Builtin((ref b, _))) => a == b,
            (&Expr::Symbol(ref a), &Expr::Symbol(ref b)) => a == b,
            (&Expr::Integer(ref a), &Expr::Integer(ref b)) => a == b,
            (&Expr::List(ref a), &Expr::List(ref b)) => a == b,
            _ => false,
        }
    }
}

impl Expr {
    pub fn new_symbol(s: &str) -> Expr {
        Expr::Symbol(s.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(Expr::Symbol("a".into()).to_string(), "a");
        assert_eq!(
            Expr::List(vec![Expr::Symbol("a".into())]).to_string(),
            "(a)"
        );
        assert_eq!(
            Expr::List(vec![Expr::Symbol("a".into()), Expr::Symbol("b".into())]).to_string(),
            "(a b)"
        );
        assert_eq!(
            Expr::List(vec![
                Expr::Symbol("values".into()),
                Expr::Symbol("b".into()),
                Expr::List(vec![
                    Expr::Symbol("+".into()),
                    Expr::Integer(1),
                    Expr::Symbol("b".into())
                ],),
                Expr::Symbol("c".into())
            ])
            .to_string(),
            "(values b (+ 1 b) c)"
        );
    }
}
