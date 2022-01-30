use ast::{pos::Pos, range::Range};
use token::Token;

use crate::result::LexErr;

#[derive(Clone)]
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

    pub fn append_char(&self, pos: &Pos, c: char) -> Result<Self, LexErr> {
        if self.escape {
            match c {
                '\\' => Ok(Self {
                    start: self.start.clone(),
                    escape: false,
                    acc: format!("{}\\", self.acc),
                }),
                'n' => Ok(Self {
                    start: self.start.clone(),
                    escape: false,
                    acc: format!("{}\n", self.acc),
                }),
                _ => Err(LexErr::InvalidEscapeSequence(pos.clone(), c)),
            }
        } else if c == '\\' {
            Ok(Self {
                start: self.start.clone(),
                escape: true,
                acc: self.acc.clone(),
            })
        } else {
            Ok(Self {
                start: self.start.clone(),
                escape: self.escape,
                acc: format!("{}{c}", self.acc),
            })
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
