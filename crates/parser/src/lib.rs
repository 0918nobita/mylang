//! 構文解析器
//!
//! ```text
//! program ::= NEWLINE* , [ stmt , { NEWLINE+ , stmt } , NEWLINE* ] ;
//! stmt ::= ( PRINT_INT | PRINT_STR ) , expr ;
//! expr ::= term , { PLUS , term } ;
//! term ::= INT_LIT | STRING_LIT ;
//! ```

use ast::{expr::Expr, stmt::Stmt};
use itertools::{put_back, PutBack};
use thiserror::Error;
use token::{KeywordKind, Token};

/// 構文解析中に発生するエラー
#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum ParseErr {
    #[error("Unexpected token")]
    UnexpectedToken,

    #[error("Term expected, but not found")]
    TermExpected,

    #[error("Add operator expected, but not found")]
    AddOpExpected,

    #[error("Either print_int or print_str expected, but not found")]
    KeywordExpected,
}

fn term(tokens: &mut PutBack<impl Iterator<Item = Token>>) -> Result<Expr, ParseErr> {
    let tok = tokens.next().ok_or(ParseErr::TermExpected)?;

    match tok {
        Token::I32(range, n) => Ok(Expr::I32Lit(range, n)),
        Token::Str(range, s) => Ok(Expr::StrLit(range, s)),
        _ => Err(ParseErr::TermExpected),
    }
}

fn expr(tokens: &mut PutBack<impl Iterator<Item = Token>>) -> Result<Expr, ParseErr> {
    let lhs = term(tokens)?;

    match tokens.next() {
        Some(Token::AddOp(_)) => {
            let rhs = expr(tokens)?;
            Ok(Expr::Add(Box::new(lhs), Box::new(rhs)))
        }
        Some(tok) => {
            tokens.put_back(tok);
            Ok(lhs)
        }
        None => Ok(lhs),
    }
}

fn stmt(tokens: &mut PutBack<impl Iterator<Item = Token>>) -> Result<Stmt, ParseErr> {
    match tokens.next() {
        Some(Token::Keyword(range, KeywordKind::PrintI32)) => {
            let expr = expr(tokens)?;
            Ok(Stmt::PrintI32(range, expr))
        }
        Some(Token::Keyword(range, KeywordKind::PrintStr)) => {
            let expr = expr(tokens)?;
            Ok(Stmt::PrintStr(range, expr))
        }
        _ => Err(ParseErr::KeywordExpected),
    }
}

pub fn parse(tokens: impl Iterator<Item = Token>) -> Result<Vec<Stmt>, ParseErr> {
    stmt(&mut put_back(tokens)).map(|stmt| vec![stmt])
}
