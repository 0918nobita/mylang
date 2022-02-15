//! 構文解析器
//!
//! ## 構文定義
//!
//! ```text
//! program ::= NEWLINE* , [ stmt , { NEWLINE+ , stmt } , NEWLINE* ] ;
//! stmt ::= ( PRINT_INT | PRINT_STR ) , expr ;
//! expr ::= term , { PLUS , term } ;
//! term ::= INT_LIT | STRING_LIT ;
//! ```

use itertools::{put_back, PutBack};
use mylang_ast::{Expr, Stmt};
use mylang_token::{KeywordKind, Locatable, Pos, Range, Token};
use thiserror::Error;

/// 構文解析中に発生するエラー
#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum ParseErr {
    #[error("Term expected, but not found")]
    TermExpected(Pos),

    #[error("Either print_int or print_str expected, but not found")]
    KeywordExpected(Range),
}

impl Locatable for ParseErr {
    fn locate(&self) -> Range {
        match self {
            ParseErr::TermExpected(pos) => pos.clone().into(),

            ParseErr::KeywordExpected(range) => range.clone(),
        }
    }
}

fn term(
    tokens: &mut PutBack<impl Iterator<Item = Token>>,
    pos: Pos,
) -> Result<(Pos, Expr), ParseErr> {
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

fn expr(
    tokens: &mut PutBack<impl Iterator<Item = Token>>,
    pos: Pos,
) -> Result<(Pos, Expr), ParseErr> {
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

fn stmt(tokens: &mut PutBack<impl Iterator<Item = Token>>, pos: Pos) -> Result<Stmt, ParseErr> {
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

fn program(tokens: &mut PutBack<impl Iterator<Item = Token>>) -> Vec<Result<Stmt, ParseErr>> {
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

pub fn parse(tokens: impl Iterator<Item = Token>) -> Vec<Result<Stmt, ParseErr>> {
    program(&mut put_back(tokens))
}
