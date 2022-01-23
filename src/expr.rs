use std::fmt;

use super::range::Range;

#[derive(Debug)]
pub enum ExprAst {
    I32Lit(i32),
    Add(Box<Expr>, Box<Expr>),
}

pub struct Expr {
    range: Range,
    ast: ExprAst,
}

impl Expr {
    pub fn new(ast: ExprAst) -> Self {
        Self {
            range: Range::default(),
            ast,
        }
    }

    pub fn new_with_range(range: Range, ast: ExprAst) -> Self {
        Self { range, ast }
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?} {:?}]", self.range, self.ast)
    }
}
