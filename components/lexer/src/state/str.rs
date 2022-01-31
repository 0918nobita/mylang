use ast::{pos::Pos, range::Range};
use token::Token;

use crate::result::LexErr;

#[derive(Clone, Debug)]
pub struct StrState {
    start: Pos,
    pub escape: bool,
    acc: String,
}

impl StrState {
    pub fn new(start: Pos) -> Self {
        Self {
            start,
            escape: false,
            acc: String::new(),
        }
    }

    pub fn try_append_char(&self, pos: &Pos, c: char) -> Result<Self, LexErr> {
        match (self.escape, c) {
            (_, '\n') => Err(LexErr::ForbiddenChar(pos.clone(), c)),

            (true, c @ ('\\' | 'n')) => Ok(Self {
                start: self.start.clone(),
                escape: false,
                acc: format!("{}{c}", self.acc),
            }),

            (true, _) => Err(LexErr::InvalidEscapeSequence(pos.clone(), c)),

            (false, '\\') => Ok(Self {
                start: self.start.clone(),
                escape: true,
                acc: self.acc.clone(),
            }),

            (false, c) => Ok(Self {
                start: self.start.clone(),
                escape: false,
                acc: format!("{}{c}", self.acc),
            }),
        }
    }

    pub fn tokenize(&self, pos: &Pos) -> Token {
        if self.escape {
            panic!("Illigal state")
        }
        Token::Str(
            Range::new(self.start.clone(), pos.clone()),
            self.acc.clone(),
        )
    }
}
