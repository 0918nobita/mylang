use mylang_token::{Locatable, Pos, Range};
use thiserror::Error;

/// 構文解析中に発生するエラー
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

pub type ParseResult<T> = Result<T, ParseErr>;
