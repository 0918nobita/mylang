mod iter;
pub mod token;

use std::io::BufRead;

use token::Token;

use ast::{expr::Expr, range::Range, stmt::Stmt};
use iter::CharsWithPosExt;
use utf8_chars::BufReadCharsExt;

pub fn tokenize<T: BufRead>(src: &mut T) -> Vec<Token> {
    for (pos, c) in src.chars().map(|r| r.unwrap()).with_pos() {
        println!("{:?} {:?}", pos, c);
    }
    vec![]
}

pub fn parse(_tokens: &[Token]) -> Vec<Stmt> {
    vec![Stmt::PrintStr(
        Range::default(),
        Expr::StrLit(Range::default(), "Hello, world!".to_owned()),
    )]
}
