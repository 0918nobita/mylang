use mylang_token::{range, KeywordKind, Pos, Token};

#[derive(Clone, Debug)]
pub struct SymbolState {
    start: Pos,
    end: Pos,
    acc: String,
}

impl SymbolState {
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

    pub fn tokenize(&self) -> Token {
        if let Some(keyword_kind) = KeywordKind::parse(&self.acc) {
            Token::Keyword(range!(self.start.clone(), self.end.clone()), keyword_kind)
        } else {
            Token::Ident(
                range!(self.start.clone(), self.end.clone()),
                self.acc.clone(),
            )
        }
    }
}
