//! バイトコードの定義

use serde::{Deserialize, Serialize};

/// 命令
#[derive(Debug, Serialize, Deserialize)]
pub enum Inst {
    I32Const(i32),
    I32Add,
    StrConst(String),
    PrintI32,
    PrintStr,

    /// PC + 1 をスタックに push したうえで、指定された番地にジャンプする
    Call(usize),

    /// スタックトから pop した番地にジャンプする
    Return,
}
