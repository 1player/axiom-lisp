#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr<'a> {
    Symbol(&'a str),
    List(Vec<Expr<'a>>),
}

impl<'a> std::fmt::Display for Expr<'a> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(Expr::Symbol("a").to_string(), "a");
        assert_eq!(Expr::List(vec![Expr::Symbol("a")]).to_string(), "(a)");
        assert_eq!(
            Expr::List(vec![Expr::Symbol("a"), Expr::Symbol("b")]).to_string(),
            "(a b)"
        );
        assert_eq!(
            Expr::List(vec![
                Expr::Symbol("values"),
                Expr::Symbol("b"),
                Expr::List(vec![
                    Expr::Symbol("+"),
                    Expr::Symbol("a"),
                    Expr::Symbol("b")
                ],),
                Expr::Symbol("c")
            ])
            .to_string(),
            "(values b (+ a b) c)"
        );
    }
}
