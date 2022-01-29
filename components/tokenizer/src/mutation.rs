mod i32;
mod i32_state;
mod initial;
mod keyword;
mod keyword_state;
pub mod state;
mod str;
mod str_state;

use ast::pos::Pos;

use self::{
    i32::mapping_for_i32_state, initial::mapping_for_initial_state,
    keyword::mapping_for_keyword_state, state::State, str::mapping_for_str_state,
};
use crate::result::TokenizeResult;

pub fn mapping(state: &mut State, pos_c: (Pos, char)) -> Vec<TokenizeResult> {
    match state.clone() {
        State::Initial => mapping_for_initial_state(state, pos_c),
        State::I32(i32_state) => mapping_for_i32_state(state, i32_state, pos_c),
        State::Str(str_state) => mapping_for_str_state(state, str_state, pos_c),
        State::Keyword(keyword_state) => mapping_for_keyword_state(state, keyword_state, pos_c),
    }
}
