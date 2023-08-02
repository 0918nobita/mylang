use std::fmt;

#[derive(PartialEq, Eq, Clone)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Range {
    pub start: Location,
    pub end: Location,
}

impl fmt::Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}-{:?}", self.start, self.end)
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

pub trait Locatable<'a> {
    fn locate(&'a self) -> &'a Range;
}
