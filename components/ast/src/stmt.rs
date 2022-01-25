use super::{
    expr::Expr,
    range::{Locatable, Range},
};

type OptRange = Option<Range>;

#[derive(Debug)]
pub enum Stmt {
    PrintI32(OptRange, Expr),
    PrintStr(OptRange, Expr),
}

impl Locatable for Stmt {
    fn get_range(&self) -> Option<Range> {
        match self {
            Stmt::PrintI32(r, _) | Stmt::PrintStr(r, _) => r.clone(),
        }
    }
}
