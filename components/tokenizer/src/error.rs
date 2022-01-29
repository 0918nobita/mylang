use ast::{pos::Pos, range::Range};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenizeError {
    #[error("({0}) Forbidden character: '{1}'")]
    ForbiddenChar(Pos, char),

    #[error("({0}) Invalid escape sequence: '{1}'")]
    InvalidEscapeSequence(Pos, char),

    #[error("({0}) Invalid keyword: \"{1}\"")]
    InvalidKeyword(Range, String),

    #[error("({0}) Missing a closing quote for string literal")]
    MissingClosingQuoteForStr(Pos),
}
