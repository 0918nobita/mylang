use serde::{Deserialize, Serialize};
use token::{Locatable, Range};

#[derive(Debug, Serialize, Deserialize)]
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
                let mut range = lhs.locate();
                let rhs_range = rhs.locate();
                range.concat(rhs_range);
                range
            }
        }
    }
}
