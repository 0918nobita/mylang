use std::str::Chars;

use ast::{
    pos::Pos,
    range::{Locatable, Range},
};

pub struct CharsWithPos<'a> {
    pos: Pos,
    chars: Chars<'a>,
}

impl<'a> From<Chars<'a>> for CharsWithPos<'a> {
    fn from(chars: Chars<'a>) -> Self {
        Self {
            pos: Pos::default(),
            chars,
        }
    }
}

impl Iterator for CharsWithPos<'_> {
    type Item = (Pos, char);

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next().map(|c| {
            let prev_pos = self.pos.clone();

            if c == '\n' {
                self.pos.line += 1;
                self.pos.column = 0;
            } else {
                self.pos.column += 1;
            }

            (prev_pos, c)
        })
    }
}

#[derive(Debug)]
pub enum Token {
    I32(Range, i32),
    AddOp(Pos),
    Str(Range, String),
    PrintI32Keyword(Range),
    PrintStrKeyword(Range),
}

impl Locatable for Token {
    fn locate(&self) -> Range {
        match self {
            Token::I32(r, _)
            | Token::Str(r, _)
            | Token::PrintI32Keyword(r)
            | Token::PrintStrKeyword(r) => r.clone(),

            Token::AddOp(p) => p.clone().into(),
        }
    }
}
