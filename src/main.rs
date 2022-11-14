use std::io;

#[derive(Clone, Debug)]
enum Expr {
    Symbol(String),
    Number(usize),
    List(Vec<Expr>),
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

fn read() -> io::Result<Expr> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    Ok(Expr::Number(10))
}

fn repl() {
    eprint!("> ");
    let expr = read().unwrap();
    eprintln!("{:?}", expr);
}

fn main() {
    repl();
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
}
