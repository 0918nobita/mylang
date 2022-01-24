use super::range::{Locatable, Range};

type OptRange = Option<Range>;

#[derive(Debug)]
pub enum Expr {
    I32Lit(OptRange, i32),
    StrLit(OptRange, String),
    Add(OptRange, Box<Expr>, Box<Expr>),
}

impl Locatable for Expr {
    fn get_range(&self) -> Option<Range> {
        match self {
            Expr::I32Lit(r, _) | Expr::Add(r, _, _) | Expr::StrLit(r, _) => r.clone(),
        }
    }
}
