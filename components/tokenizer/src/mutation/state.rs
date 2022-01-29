use super::{i32_state::I32State, keyword_state::KeywordState, str_state::StrState};

#[derive(Clone)]
pub enum State {
    Initial,
    I32(I32State),
    Str(StrState),
    Keyword(KeywordState),
}
