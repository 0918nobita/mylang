use token::{KeywordKind, Pos, Range, Token};

use crate::result::{LexErr, LexResult};

#[derive(Clone, Debug)]
pub struct KeywordState {
    start: Pos,
    end: Pos,
    acc: String,
}

impl KeywordState {
    pub fn new(pos: Pos, acc: String) -> Self {
        Self {
            start: pos.clone(),
            end: pos,
            acc,
        }
    }

    pub fn append_char(&self, pos: Pos, c: char) -> Self {
        Self {
            start: self.start.clone(),
            end: pos,
            acc: format!("{}{c}", self.acc),
        }
    }

    pub fn try_tokenize(&self) -> LexResult {
        if let Some(keyword_kind) = KeywordKind::parse(&self.acc) {
            Ok(Token::Keyword(
                Range::new(self.start.clone(), self.end.clone()),
                keyword_kind,
            ))
        } else {
            Err(LexErr::InvalidKeyword(
                Range::new(self.start.clone(), self.end.clone()),
                self.acc.to_string(),
            ))
        }
    }
}
