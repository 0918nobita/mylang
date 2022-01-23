use std::fmt;

use super::pos::Pos;

#[derive(Default)]
pub struct Range {
    start: Pos,
    end: Pos,
}

impl fmt::Debug for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}-{:?}", self.start, self.end)
    }
}
