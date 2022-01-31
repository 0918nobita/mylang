pub mod i32;
pub mod keyword;
pub mod str;

use self::{i32::I32State, keyword::KeywordState, str::StrState};

#[derive(Clone, Debug)]
pub enum State {
    Initial,
    I32(I32State),
    Str(StrState),
    Keyword(KeywordState),
}
