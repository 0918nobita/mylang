use std::fmt;

#[derive(Clone, Default)]
pub struct Pos {
    line: u32,
    column: u32,
}

impl fmt::Debug for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
