use ast::{
    pos::Pos,
    range::{Locatable, Range},
};

pub struct CharsWithPos<'a> {
    pos: Pos,
    chars: Box<dyn Iterator<Item = char> + 'a>,
}

pub trait CharsWithPosExt<'a>: Iterator<Item = char> + 'a {
    fn with_pos(&'a mut self) -> CharsWithPos<'a> {
        CharsWithPos {
            pos: Pos::default(),
            chars: Box::new(self),
        }
    }
}

impl<'a, I> CharsWithPosExt<'a> for I where I: Iterator<Item = char> + 'a {}

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
