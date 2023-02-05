use std::borrow::Cow;

use crate::eval::EvalError;

pub type Builtin = fn(&[Expr]) -> Result<Expr, EvalError>;
pub type Integer = isize;
pub type Symbol = String;
pub type List = Vec<Expr>;

#[derive(Clone)]
pub enum Expr {
    Builtin((&'static str, Builtin)),
    Integer(Integer),
    Symbol(Symbol),
    List(List),
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
    pub fn new_symbol<'a, T: Into<Cow<'a, str>>>(s: T) -> Expr {
        Expr::Symbol(s.into().into_owned())
    }

    pub fn new_list<'a, T: Into<Cow<'a, [Expr]>>>(l: T) -> Expr {
        Expr::List(l.into().into_owned())
    }

    pub fn new_integer(n: Integer) -> Expr {
        Expr::Integer(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(Expr::new_symbol("a").to_string(), "a");
        assert_eq!(
            Expr::new_list(vec![Expr::new_symbol("a")]).to_string(),
            "(a)"
        );
        assert_eq!(
            Expr::new_list(vec![Expr::new_symbol("a"), Expr::new_symbol("b")]).to_string(),
            "(a b)"
        );
        assert_eq!(
            Expr::List(vec![
                Expr::new_symbol("values"),
                Expr::new_symbol("b"),
                Expr::new_list(vec![
                    Expr::new_symbol("+"),
                    Expr::new_integer(1),
                    Expr::new_symbol("b")
                ],),
                Expr::new_symbol("c")
            ])
            .to_string(),
            "(values b (+ 1 b) c)"
        );
    }
}
