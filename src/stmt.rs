use std::fmt;

use super::{expr::Expr, range::Range};

#[derive(Debug)]
pub enum StmtAst {
    Print(Expr),
}

pub struct Stmt {
    range: Range,
    ast: StmtAst,
}

impl Stmt {
    pub fn new(ast: StmtAst) -> Self {
        Self {
            range: Range::default(),
            ast,
        }
    }

    pub fn new_with_range(ast: StmtAst, range: Range) -> Self {
        Self { range, ast }
    }
}

impl fmt::Debug for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?} {:?}]", self.range, self.ast)
    }
}
