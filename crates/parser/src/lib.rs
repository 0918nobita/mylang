//! 構文解析器
//!
//! ```text
//! program ::= NEWLINE* , [ stmt , { NEWLINE+ , stmt } , NEWLINE* ] ;
//! stmt ::= ( PRINT_INT | PRINT_STR ) , expr ;
//! expr ::= term , { PLUS , term } ;
//! term ::= INT_LIT | STRING_LIT ;
//! ```

use ast::{expr::Expr, range::Range, stmt::Stmt};
use thiserror::Error;
use token::Token;

#[allow(dead_code)]
fn term() {
    unimplemented!()
}

#[allow(dead_code)]
fn expr() {
    unimplemented!()
}

#[allow(dead_code)]
fn stmt() {
    unimplemented!()
}

#[allow(dead_code)]
fn program() {
    unimplemented!()
}

/// 構文解析中に発生するエラー
#[allow(dead_code)]
#[derive(Debug, Error)]
enum ParseErr {
    #[error("({0}) Unexpected token")]
    UnexpectedToken(Range),
}

type ParseResult = Result<Stmt, ParseErr>;

#[allow(dead_code)]
struct Parse<I>
where
    I: Iterator<Item = Token> + Sized,
{
    iter: I,
}

impl<I> Iterator for Parse<I>
where
    I: Iterator<Item = Token> + Sized,
{
    type Item = ParseResult;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

trait ParseExt: Iterator<Item = Token> + Sized {
    fn parse(self) -> Parse<Self>;
}

impl<I> ParseExt for I
where
    I: Iterator<Item = Token> + Sized,
{
    fn parse(self) -> Parse<Self> {
        Parse { iter: self }
    }
}

pub fn parse(_tokens: &[Token]) -> Vec<Stmt> {
    vec![Stmt::PrintStr(
        Range::default(),
        Expr::StrLit(Range::default(), "Hello, world!".to_owned()),
    )]
}
