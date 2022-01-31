use ast::{pos::Pos, range::Range};
use token::Token;

#[derive(Clone, Debug)]
pub struct I32State {
    start: Pos,
    acc: String,
}

impl I32State {
    pub fn new(start: Pos, acc: String) -> Self {
        Self { start, acc }
    }

    pub fn append_digit_char(&self, c: char) -> Self {
        Self {
            start: self.start.clone(),
            acc: format!("{}{c}", self.acc),
        }
    }

    pub fn tokenize(&self, pos: &Pos) -> Token {
        let i = self.acc.parse::<i32>().unwrap();
        Token::I32(Range::new(self.start.clone(), pos.clone()), i)
    }
}
