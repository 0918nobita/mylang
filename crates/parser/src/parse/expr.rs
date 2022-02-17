use itertools::PutBack;
use mylang_ast::Expr;
use mylang_token::{Pos, Token};

use super::term::term;
use crate::ParseResult;

pub fn expr(
    tokens: &mut PutBack<impl Iterator<Item = Token>>,
    pos: Pos,
) -> ParseResult<(Pos, Expr)> {
    let (pos, lhs) = term(tokens, pos)?;

    match tokens.next() {
        Some(Token::AddOp(_)) => {
            let (pos, rhs) = expr(tokens, pos)?;
            Ok(match rhs {
                Expr::Add(b, c) => (pos, Expr::Add(Box::new(Expr::Add(Box::new(lhs), b)), c)),

                _ => (pos, Expr::Add(Box::new(lhs), Box::new(rhs))),
            })
        }

        Some(tok) => {
            tokens.put_back(tok);
            Ok((pos, lhs))
        }

        None => Ok((pos, lhs)),
    }
}
