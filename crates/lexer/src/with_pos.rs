//! 文字のイテレータを「位置・文字ペア」のイテレータに変換するアダプタ

use token::Pos;

pub struct WithPos<I>
where
    I: Iterator<Item = char> + Sized,
{
    pos: Pos,
    chars: I,
}

impl<I> Iterator for WithPos<I>
where
    I: Iterator<Item = char>,
{
    type Item = (Pos, char);

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next().map(|c| {
            let prev_pos = self.pos.clone();

            if c == '\n' {
                self.pos.next_line();
            } else {
                self.pos.next_char();
            }

            (prev_pos, c)
        })
    }
}

pub trait WithPosExt: Iterator<Item = char> + Sized {
    fn with_pos(self) -> WithPos<Self>;
}

impl<I> WithPosExt for I
where
    I: Iterator<Item = char> + Sized,
{
    fn with_pos(self) -> WithPos<Self> {
        WithPos {
            pos: Pos::default(),
            chars: self,
        }
    }
}
