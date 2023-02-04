#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Symbol(String),
    List(Vec<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Symbol(s) => write!(f, "{}", s),
            Expr::List(exprs) => {
                let inner: Vec<_> = exprs.iter().map(|e| format!("{}", e)).collect();
                write!(f, "({})", inner.join(" "))
            }
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
                    Expr::Symbol("a".into()),
                    Expr::Symbol("b".into())
                ],),
                Expr::Symbol("c".into())
            ])
            .to_string(),
            "(values b (+ a b) c)"
        );
    }
}
