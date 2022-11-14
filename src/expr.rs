#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr<'a> {
    Symbol(&'a str),
    List(Vec<Expr<'a>>),
}
