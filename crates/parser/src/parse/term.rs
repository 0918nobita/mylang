use itertools::PutBack;
use mylang_ast::Expr;
use mylang_token::{Locatable, Pos, Token};

use crate::result::{ParseErr, ParseResult};

pub fn term(
    tokens: &mut PutBack<impl Iterator<Item = Token>>,
    pos: Pos,
) -> ParseResult<(Pos, Expr)> {
    if let Some(tok) = tokens.next() {
        match tok {
            Token::I32(range, n) => Ok((range.end_ref().clone(), Expr::I32Lit(range, n))),

            Token::Str(range, s) => Ok((range.end_ref().clone(), Expr::StrLit(range, s))),

            _ => Err(ParseErr::TermExpected(tok.locate().end())),
        }
    } else {
        Err(ParseErr::TermExpected(pos))
    }
}
