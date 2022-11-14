use crate::expr::Expr;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    UnexpectedEOF,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Token<'a> {
    OpenParen,
    CloseParen,
    Atom(&'a str),
}

fn tokenize<'a>(input: &'a str) -> Vec<Token<'a>> {
    let mut tokens = vec![];
    let mut atom_start_index = None;

    for (i, c) in input.chars().enumerate() {
        let (atom_ended, token) = if c.is_whitespace() {
            (true, None)
        } else if c == '(' {
            (true, Some(Token::OpenParen))
        } else if c == ')' {
            (true, Some(Token::CloseParen))
        } else {
            if atom_start_index.is_none() {
                atom_start_index = Some(i);
            }
            (false, None)
        };

        if atom_ended {
            if let Some(start) = atom_start_index {
                let atom = &input[start..i];
                atom_start_index = None;
                tokens.push(Token::Atom(atom));
            }

            if let Some(t) = token {
                tokens.push(t);
            }
        }
    }

    if let Some(start) = atom_start_index {
        let atom = &input[start..];
        tokens.push(Token::Atom(atom));
    }

    tokens
}

fn parse<'a>(tokens: &[Token<'a>]) -> Result<Expr<'a>, ParseError> {
    let mut index = None;
    let mut expr_stack: Vec<Vec<Expr<'a>>> = vec![];

    for token in tokens {
        match token {
            Token::OpenParen => {
                index = if let Some(i) = index {
                    Some(i + 1)
                } else {
                    Some(0)
                };

                expr_stack.push(vec![]);
            }
            Token::CloseParen => {
                if let Some(i) = index {
                    let expr = Expr::List(expr_stack.pop().unwrap());
                    if i >= 1 {
                        expr_stack[i - 1].push(expr);
                        index = Some(i - 1);
                    } else {
                        return Ok(expr);
                    }
                } else {
                    return Err(ParseError::UnexpectedEOF);
                }
            }
            Token::Atom(atom) => {
                let symbol = Expr::Symbol(atom);
                if let Some(i) = index {
                    expr_stack[i].push(symbol);
                } else {
                    return Ok(symbol);
                }
            }
        }
    }

    Err(ParseError::UnexpectedEOF)
}

pub fn read<'a>(input: &'a str) -> Result<Expr<'a>, ParseError> {
    let tokens = tokenize(input);
    parse(&tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize("  5 (* pi (* a b))foo bar"),
            vec![
                Token::Atom("5"),
                Token::OpenParen,
                Token::Atom("*"),
                Token::Atom("pi"),
                Token::OpenParen,
                Token::Atom("*"),
                Token::Atom("a"),
                Token::Atom("b"),
                Token::CloseParen,
                Token::CloseParen,
                Token::Atom("foo"),
                Token::Atom("bar"),
            ]
        );
    }

    #[test]
    fn test_read() {
        assert_eq!(read(""), Err(ParseError::UnexpectedEOF));

        assert_eq!(read("5"), Ok(Expr::Symbol("5")));

        assert_eq!(read("a b"), Ok(Expr::Symbol("a")));

        assert_eq!(
            read("(* x y)"),
            Ok(Expr::List(vec![
                Expr::Symbol("*"),
                Expr::Symbol("x"),
                Expr::Symbol("y"),
            ]))
        );

        assert_eq!(read("(* x y"), Err(ParseError::UnexpectedEOF));

        assert_eq!(
            read("(* (+ a b) c)"),
            Ok(Expr::List(vec![
                Expr::Symbol("*"),
                Expr::List(vec![
                    Expr::Symbol("+"),
                    Expr::Symbol("a"),
                    Expr::Symbol("b"),
                ]),
                Expr::Symbol("c"),
            ]))
        );
    }
}
