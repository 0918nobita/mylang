mod i32;
mod initial;
mod keyword;
mod str;

use ast::pos::Pos;

use self::{
    i32::{mapping_for_i32_state, I32State},
    initial::mapping_for_initial_state,
    keyword::{mapping_for_keyword_state, KeywordState},
    str::{mapping_for_str_state, StrState},
};
use crate::result::TokenizeResult;

#[derive(Clone)]
pub enum State {
    Initial,
    I32(I32State),
    Str(StrState),
    Keyword(KeywordState),
}

pub fn mapping(state: &mut State, pos_c: (Pos, char)) -> Vec<TokenizeResult> {
    match state.clone() {
        State::Initial => mapping_for_initial_state(state, pos_c),
        State::I32(i32_state) => mapping_for_i32_state(state, i32_state, pos_c),
        State::Str(str_state) => mapping_for_str_state(state, str_state, pos_c),
        State::Keyword(keyword_state) => mapping_for_keyword_state(state, keyword_state, pos_c),
    }
}
