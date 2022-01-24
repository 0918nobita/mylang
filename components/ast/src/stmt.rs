use super::{
    expr::Expr,
    range::{Locatable, Range},
};

type OptRange = Option<Range>;

#[derive(Debug)]
pub enum Stmt {
    Print(OptRange, Expr),
}

impl Locatable for Stmt {
    fn get_range(&self) -> Option<Range> {
        match self {
            Stmt::Print(r, _) => r.clone(),
        }
    }
}
