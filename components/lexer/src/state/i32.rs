use ast::{pos::Pos, range::Range};
use token::Token;

#[derive(Clone, Debug)]
pub struct I32State {
    start: Pos,
    end: Pos,
    acc: String,
}

impl I32State {
    pub fn new(pos: Pos, acc: String) -> Self {
        Self {
            start: pos.clone(),
            end: pos,
            acc,
        }
    }

    pub fn append_digit_char(&self, pos: Pos, c: char) -> Self {
        Self {
            start: self.start.clone(),
            end: pos,
            acc: format!("{}{c}", self.acc),
        }
    }

    pub fn tokenize(&self) -> Token {
        let i = self.acc.parse::<i32>().unwrap();
        Token::I32(Range::new(self.start.clone(), self.end.clone()), i)
    }
}
