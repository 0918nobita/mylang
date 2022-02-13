use mylang_token::{Locatable, Range};
use serde::{Deserialize, Serialize};

use super::expr::Expr;

/// 文を表す抽象構文木
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
