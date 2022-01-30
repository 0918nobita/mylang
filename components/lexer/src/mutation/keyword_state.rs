use ast::{pos::Pos, range::Range};
use token::{KeywordKind, Token};

use crate::result::{LexErr, LexResult};

#[derive(Clone)]
pub struct KeywordState {
    start: Pos,
    acc: String,
}

impl KeywordState {
    pub fn new(start: Pos, acc: String) -> Self {
        Self { start, acc }
    }

    pub fn append_char(&self, c: char) -> Self {
        Self {
            start: self.start.clone(),
            acc: format!("{}{}", self.acc, c),
        }
    }

    pub fn try_tokenize(&self, pos: &Pos) -> LexResult {
        if let Some(keyword_kind) = KeywordKind::parse(&self.acc) {
            Ok(Token::Keyword(
                Range::new(self.start.clone(), pos.clone()),
                keyword_kind,
            ))
        } else {
            Err(LexErr::InvalidKeyword(
                Range::new(self.start.clone(), pos.clone()),
                self.acc.to_string(),
            ))
        }
    }
}
