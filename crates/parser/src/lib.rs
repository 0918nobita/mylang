//! 構文解析器

use token::Token;

use ast::{expr::Expr, range::Range, stmt::Stmt};

pub fn parse(_tokens: &[Token]) -> Vec<Stmt> {
    vec![Stmt::PrintStr(
        Range::default(),
        Expr::StrLit(Range::default(), "Hello, world!".to_owned()),
    )]
}
