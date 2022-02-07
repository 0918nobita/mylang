use token::{Pos, Range, Token};

use crate::result::LexErr;

#[derive(Clone, Debug)]
pub struct StrState {
    start: Pos,
    pub end: Pos,
    pub escape: bool,
    acc: String,
}

impl StrState {
    pub fn new(pos: Pos) -> Self {
        Self {
            start: pos.clone(),
            end: pos,
            escape: false,
            acc: String::new(),
        }
    }

    pub fn try_append_char(&self, pos: Pos, c: char) -> Result<Self, LexErr> {
        match (self.escape, c) {
            (_, '\n') => Err(LexErr::ForbiddenChar(pos, c)),

            (true, c @ ('\\' | 'n')) => Ok(Self {
                start: self.start.clone(),
                end: pos,
                escape: false,
                acc: format!("{}{c}", self.acc),
            }),

            (true, _) => Err(LexErr::InvalidEscapeSequence(pos, c)),

            (false, '\\') => Ok(Self {
                start: self.start.clone(),
                end: pos,
                escape: true,
                acc: self.acc.clone(),
            }),

            (false, c) => Ok(Self {
                start: self.start.clone(),
                end: pos,
                escape: false,
                acc: format!("{}{c}", self.acc),
            }),
        }
    }

    pub fn tokenize(&self) -> Token {
        if self.escape {
            panic!("Illigal state")
        }
        Token::Str(
            Range::new(self.start.clone(), self.end.clone()),
            self.acc.clone(),
        )
    }
}
