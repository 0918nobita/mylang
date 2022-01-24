use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Inst {
    I32Const(i32),
    I32Add,
    Print,
}
