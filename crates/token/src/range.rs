use std::fmt;

use serde::{Deserialize, Serialize};

use super::pos::Pos;

#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
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

    /// 末尾を別の範囲の末尾に設定する
    ///
    /// ```
    /// use token::range;
    ///
    /// let mut former = range!(0;0, 0;5);
    ///
    /// let latter = range!(0;7, 0;10);
    /// former.concat(latter);
    ///
    /// let expected = range!(0;0, 0;10);
    /// assert_eq!(former, expected);
    /// ```
    pub fn concat(&mut self, other: Range) {
        self.end = other.end;
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
