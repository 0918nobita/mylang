use ast::pos::Pos;

#[derive(Clone)]
pub struct I32State {
    pub start: Pos,
    pub acc: String,
}

impl I32State {
    pub fn new(start: Pos, acc: String) -> Self {
        Self { start, acc }
    }
}

#[derive(Clone)]
pub struct StrState {
    pub start: Pos,
    pub escape: bool,
    pub acc: String,
}

impl StrState {
    pub fn new(start: Pos, escape: bool, acc: String) -> Self {
        Self { start, escape, acc }
    }
}

#[derive(Clone)]
pub struct KeywordState {
    pub start: Pos,
    pub acc: String,
}

impl KeywordState {
    pub fn new(start: Pos, acc: String) -> Self {
        Self { start, acc }
    }
}

#[derive(Clone)]
pub enum State {
    Initial,
    I32(I32State),
    Str(StrState),
    Keyword(KeywordState),
}
