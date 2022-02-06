//! 字句解析の結果

use ast::{
    pos::Pos,
    range::{Locatable, Range},
};
use thiserror::Error;
use token::Token;

/// 字句解析中に発生するエラー
#[derive(Debug, Error)]
pub enum LexErr {
    #[error("({0}) Forbidden character: '{1}'")]
    ForbiddenChar(Pos, char),

    #[error("({0}) Invalid escape sequence: '{1}'")]
    InvalidEscapeSequence(Pos, char),

    #[error("({0}) Invalid keyword: \"{1}\"")]
    InvalidKeyword(Range, String),

    #[error("({0}) Missing a closing quote for string literal")]
    MissingClosingQuoteForStr(Pos),
}

impl Locatable for LexErr {
    fn locate(&self) -> Range {
        match self {
            LexErr::ForbiddenChar(pos, _)
            | LexErr::InvalidEscapeSequence(pos, _)
            | LexErr::MissingClosingQuoteForStr(pos) => pos.clone().into(),
            LexErr::InvalidKeyword(range, _) => range.clone(),
        }
    }
}

/// 字句解析中に得られたトークン、または発生したエラー
pub type LexResult = Result<Token, LexErr>;
