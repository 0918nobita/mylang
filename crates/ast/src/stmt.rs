use serde::{Deserialize, Serialize};

use super::{
    expr::Expr,
    range::{Locatable, Range},
};

#[derive(Debug, Serialize, Deserialize)]
pub enum Stmt {
    PrintI32(Range, Expr),
    PrintStr(Range, Expr),
}

impl Locatable for Stmt {
    fn locate(&self) -> Range {
        match self {
            Stmt::PrintI32(r, _) | Stmt::PrintStr(r, _) => r.clone(),
        }
    }
}