use std::fmt;

use serde::{Deserialize, Serialize};

use super::pos::Pos;

/// ソースコード中の範囲
#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Range {
    /// 始端
    start: Pos,
    /// 終端
    end: Pos,
}

impl Range {
    /// 新しい範囲を生成して返す
    pub fn new(start: Pos, end: Pos) -> Self {
        Range { start, end }
    }

    /// 終端の参照を返す
    pub fn end_ref(&self) -> &Pos {
        &self.end
    }

    /// 所有権をムーブして終端を返す
    pub fn end(self) -> Pos {
        self.end
    }

    /// 終端を別の範囲の終端に設定する
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
    /// 始端と終端が同じ位置の範囲に変換する
    fn from(pos: Pos) -> Self {
        Self {
            start: pos.clone(),
            end: pos,
        }
    }
}
