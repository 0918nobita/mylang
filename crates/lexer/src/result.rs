//! 字句解析の結果

use mylang_token::{Locatable, Pos, Range};
use thiserror::Error;

/// 字句解析中に発生するエラー
#[derive(Debug, Error)]
pub enum LexErr {
    #[error("Forbidden character: '{1}'")]
    ForbiddenChar(Pos, char),

    #[error("Invalid escape sequence: '{1}'")]
    InvalidEscapeSequence(Pos, char),

    #[error("Invalid keyword: \"{1}\"")]
    InvalidKeyword(Range, String),

    #[error("Missing a closing quote for string literal")]
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

/// 字句解析の結果
pub type LexResult<T> = Result<T, LexErr>;
