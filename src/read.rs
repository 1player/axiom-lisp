use crate::expr::Expr;

#[derive(Debug, PartialEq, Eq)]
pub enum ReadError {
    IOError,
    InvalidUTF8,
    UnbalancedParen,
    UnexpectedEOF,
    Empty,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Token {
    OpenParen,
    CloseParen,
    Atom(String),
}

fn next_token(reader: &mut impl std::io::BufRead) -> Result<Option<Token>, ReadError> {
    let mut atom_start_index = None;
    let mut bytes_read = 0;
    let mut token = None;

    let buf = reader.fill_buf().map_err(|_| ReadError::IOError)?;
    let input = std::str::from_utf8(buf).map_err(|_| ReadError::InvalidUTF8)?;

    for (i, c) in input.chars().enumerate() {
        if let Some(start) = atom_start_index {
            // If we're reading an atom, stop as soon as
            // we reach a parens or a whitespace, and don't
            // consume the input.
            if c.is_whitespace() || c == '(' || c == ')' {
                let atom = &input[start..i];
                token = Some(Token::Atom(atom.to_owned()));
                break;
            }
        }

        bytes_read += c.len_utf8();

        if c.is_whitespace() {
            continue;
        } else if c == '(' {
            token = Some(Token::OpenParen);
            break;
        } else if c == ')' {
            token = Some(Token::CloseParen);
            break;
        } else {
            if atom_start_index.is_none() {
                atom_start_index = Some(i);
            }
        }
    }

    if token.is_none() {
        if let Some(start) = atom_start_index {
            let atom = &input[start..];
            token = Some(Token::Atom(atom.to_owned()));
        }
    }

    reader.consume(bytes_read);
    return Ok(token);
}

pub fn read_buffer(reader: &mut impl std::io::BufRead) -> Result<Expr, ReadError> {
    let mut index = None;
    let mut expr_stack: Vec<Vec<Expr>> = vec![];

    while let Some(token) = next_token(reader)? {
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
                    return Err(ReadError::UnbalancedParen);
                }
            }
            Token::Atom(atom) => {
                let expr = parse_atom(&atom)?;
                if let Some(i) = index {
                    expr_stack[i].push(expr);
                } else {
                    return Ok(expr);
                }
            }
        }
    }

    if index.is_some() {
        Err(ReadError::UnexpectedEOF)
    } else {
        Err(ReadError::Empty)
    }
}

fn parse_atom(atom: &str) -> Result<Expr, ReadError> {
    // try parsing as an integer first
    match atom.parse::<isize>() {
        Ok(n) => Ok(Expr::Integer(n)),
        _ => Ok(Expr::Symbol((*atom).to_owned())),
    }
}

pub fn read_string(input: &str) -> Result<Expr, ReadError> {
    let mut reader = std::io::BufReader::new(input.as_bytes());
    read_buffer(&mut reader)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn atom(s: &str) -> Token {
        Token::Atom(s.to_owned())
    }

    #[test]
    fn test_token() {
        let s = "  5 (* pi (* a b))foo bar";
        let mut reader = std::io::BufReader::new(s.as_bytes());

        assert_eq!(next_token(&mut reader), Ok(Some(atom("5"))));
        assert_eq!(next_token(&mut reader), Ok(Some(Token::OpenParen)));
        assert_eq!(next_token(&mut reader), Ok(Some(atom("*"))));
        assert_eq!(next_token(&mut reader), Ok(Some(atom("pi"))));
        assert_eq!(next_token(&mut reader), Ok(Some(Token::OpenParen)));
        assert_eq!(next_token(&mut reader), Ok(Some(atom("*"))));
        assert_eq!(next_token(&mut reader), Ok(Some(atom("a"))));
        assert_eq!(next_token(&mut reader), Ok(Some(atom("b"))));
        assert_eq!(next_token(&mut reader), Ok(Some(Token::CloseParen)));
        assert_eq!(next_token(&mut reader), Ok(Some(Token::CloseParen)));
        assert_eq!(next_token(&mut reader), Ok(Some(atom("foo"))));
        assert_eq!(next_token(&mut reader), Ok(Some(atom("bar"))));
        assert_eq!(next_token(&mut reader), Ok(None));
    }

    #[test]
    fn test_read_buf() {
        let s = "  5 (* pi (* a b))foo bar";
        let mut reader = std::io::BufReader::new(s.as_bytes());

        assert_eq!(read_buffer(&mut reader), Ok(Expr::Integer(5)));
        assert_eq!(
            read_buffer(&mut reader),
            Ok(Expr::new_list(vec![
                Expr::Symbol("*".into()),
                Expr::Symbol("pi".into()),
                Expr::new_list(vec![
                    Expr::Symbol("*".into()),
                    Expr::Symbol("a".into()),
                    Expr::Symbol("b".into()),
                ])
            ]))
        );
        assert_eq!(read_buffer(&mut reader), Ok(Expr::Symbol("foo".into())));
        assert_eq!(read_buffer(&mut reader), Ok(Expr::Symbol("bar".into())));
        assert_eq!(read_buffer(&mut reader), Err(ReadError::Empty));
    }

    #[test]
    fn test_read_string() {
        assert_eq!(read_string(""), Err(ReadError::Empty));

        assert_eq!(read_string("5"), Ok(Expr::Integer(5)));

        assert_eq!(read_string("a b"), Ok(Expr::Symbol("a".into())));

        assert_eq!(
            read_string("(* x y)"),
            Ok(Expr::List(vec![
                Expr::Symbol("*".into()),
                Expr::Symbol("x".into()),
                Expr::Symbol("y".into()),
            ]))
        );

        assert_eq!(read_string("(* x y"), Err(ReadError::UnexpectedEOF));

        assert_eq!(
            read_string("(* (+ a b) 4)"),
            Ok(Expr::List(vec![
                Expr::Symbol("*".into()),
                Expr::List(vec![
                    Expr::Symbol("+".into()),
                    Expr::Symbol("a".into()),
                    Expr::Symbol("b".into()),
                ]),
                Expr::Integer(4),
            ]))
        );
    }
}
