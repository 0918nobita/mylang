use itertools::PutBack;
use mylang_ast::Stmt;
use mylang_token::{KeywordKind, Locatable, Pos, Token};

use super::expr::expr;
use crate::{ParseErr, ParseResult};

pub fn stmt(tokens: &mut PutBack<impl Iterator<Item = Token>>, pos: Pos) -> ParseResult<Stmt> {
    match tokens.next() {
        Some(Token::Keyword(range, KeywordKind::PrintI32)) => {
            let (_, expr) = expr(tokens, pos)?;
            Ok(Stmt::PrintI32(range, expr))
        }

        Some(Token::Keyword(range, KeywordKind::PrintStr)) => {
            let (_, expr) = expr(tokens, pos)?;
            Ok(Stmt::PrintStr(range, expr))
        }

        Some(tok) => Err(ParseErr::KeywordExpected(tok.locate())),

        _ => Err(ParseErr::KeywordExpected(pos.into())),
    }
}
