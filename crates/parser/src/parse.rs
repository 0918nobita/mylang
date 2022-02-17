mod expr;
mod program;
mod stmt;
mod term;

use itertools::put_back;
use mylang_ast::Stmt;
use mylang_token::Token;

use self::program::program;
use crate::result::ParseResult;

pub fn parse(tokens: impl Iterator<Item = Token>) -> Vec<ParseResult<Stmt>> {
    program(&mut put_back(tokens))
}
