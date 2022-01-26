use std::fmt;

use super::pos::Pos;

#[derive(Clone, Default)]
pub struct Range {
    pub start: Pos,
    pub end: Pos,
}

impl fmt::Debug for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}-{:?}", self.start, self.end)
    }
}

impl From<Pos> for Range {
    fn from(pos: Pos) -> Self {
        Self {
            start: pos.clone(),
            end: pos,
        }
    }
}

pub trait Locatable {
    fn get_range(&self) -> Range;
}
