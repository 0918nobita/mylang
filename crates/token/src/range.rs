use std::fmt;

use serde::{Deserialize, Serialize};

use super::pos::Pos;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Range {
    start: Pos,
    end: Pos,
}

impl Range {
    pub fn new(start: Pos, end: Pos) -> Self {
        Range { start, end }
    }

    pub fn end(&self) -> Pos {
        self.end.clone()
    }

    pub fn concat(&self, other: Range) -> Range {
        Range {
            start: self.start.clone(),
            end: other.end,
        }
    }
}

impl fmt::Debug for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}-{:?}", self.start, self.end)
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
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
