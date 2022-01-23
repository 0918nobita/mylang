use std::fmt::{Debug, Formatter, Result as FmtResult};

#[derive(Default)]
struct Pos {
    line: u32,
    column: u32,
}

impl Debug for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[derive(Default)]
pub struct Range {
    start: Pos,
    end: Pos,
}

impl Debug for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}-{:?}", self.start, self.end)
    }
}

#[derive(Debug)]
pub enum Expr {
    I32Lit(Range, i32),
    Add(Range, Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub enum Stmt {
    Print(Range, Expr),
}
