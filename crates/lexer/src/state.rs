//! 字句解析器の状態

pub mod i32;
pub mod keyword;
pub mod str;

use self::{i32::I32State, keyword::KeywordState, str::StrState};

/// 字句解析器内部の状態
#[derive(Clone, Debug)]
pub enum State {
    /// 初期状態
    Initial,
    I32(I32State),
    Str(StrState),
    Keyword(KeywordState),
}
