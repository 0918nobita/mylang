use itertools::PutBack;
use mylang_ast::Stmt;
use mylang_token::{Locatable, Token};

use super::stmt::stmt;
use crate::result::{ParseErr, ParseResult};

pub fn program(tokens: &mut PutBack<impl Iterator<Item = Token>>) -> Vec<ParseResult<Stmt>> {
    let mut results = Vec::<Result<Stmt, ParseErr>>::new();

    while let Some(tok) = tokens.next() {
        match tok {
            Token::Newline(_) => (),

            _ => {
                let pos = tok.locate().end();
                tokens.put_back(tok);
                results.push(stmt(tokens, pos));
            }
        }
    }

    results
}
