use super::{
    expr::Expr,
    range::{Locatable, Range},
};

#[derive(Debug)]
pub enum Stmt {
    PrintI32(Range, Expr),
    PrintStr(Range, Expr),
}

impl Locatable for Stmt {
    fn get_range(&self) -> Range {
        match self {
            Stmt::PrintI32(r, _) | Stmt::PrintStr(r, _) => r.clone(),
        }
    }
}
