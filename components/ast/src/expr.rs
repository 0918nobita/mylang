use super::range::{Locatable, Range};

#[derive(Debug)]
pub enum Expr {
    I32Lit(Range, i32),
    StrLit(Range, String),
    Add(Box<Expr>, Box<Expr>),
}

impl Locatable for Expr {
    fn locate(&self) -> Range {
        match self {
            Expr::I32Lit(r, _) | Expr::StrLit(r, _) => r.clone(),

            Expr::Add(lhs, rhs) => {
                let lhs_range = lhs.locate();
                let rhs_range = rhs.locate();
                Range {
                    start: lhs_range.start,
                    end: rhs_range.end,
                }
            }
        }
    }
}
