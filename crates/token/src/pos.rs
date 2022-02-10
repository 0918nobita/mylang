use std::fmt;

use serde::{Deserialize, Serialize};

/// ソースコード上の位置
#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Pos {
    /// 行番号
    line: u32,
    /// 列番号
    character: u32,
}

impl Pos {
    /// 新しい位置を生成して返す
    pub fn new(line: u32, character: u32) -> Self {
        Self { line, character }
    }

    /// 次の文字位置に移動する (改行ではない文字が続くことを想定している)
    ///
    /// ```
    /// use token::pos;
    ///
    /// let mut pos = pos!(3;7);
    /// pos.next_char();
    ///
    /// let expected = pos!(3;8);
    /// assert_eq!(pos, expected);
    /// ```
    pub fn next_char(&mut self) {
        self.character += 1;
    }

    /// 次の行の先頭に移動する
    ///
    /// ```
    /// use token::pos;
    ///
    /// let mut pos = pos!(1;8);
    /// pos.next_line();
    ///
    /// let expected = pos!(2;0);
    /// assert_eq!(pos, expected);
    /// ```
    pub fn next_line(&mut self) {
        self.line += 1;
        self.character = 0;
    }
}

impl fmt::Debug for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.character)
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line + 1, self.character + 1)
    }
}
