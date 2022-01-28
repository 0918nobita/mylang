use ast::pos::Pos;

pub struct WithPos<I>
where
    I: Iterator<Item = char>,
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
                self.pos.line += 1;
                self.pos.column = 0;
            } else {
                self.pos.column += 1;
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
