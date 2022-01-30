use ast::{pos::Pos, range::Range};
use thiserror::Error;
use token::Token;

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

pub type LexResult = Result<Token, LexErr>;
