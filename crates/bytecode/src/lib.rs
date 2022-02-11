//! バイトコードの定義

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Inst {
    I32Const(i32),
    I32Add,
    StrConst(String),
    PrintI32,
    PrintStr,
}
