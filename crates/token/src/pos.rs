use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Pos {
    line: u32,
    character: u32,
}

impl Pos {
    pub fn next_char(&mut self) {
        self.character += 1;
    }

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
