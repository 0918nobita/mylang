use ast::pos::Pos;

use super::str_state::StrState;
use crate::{
    mutation::State,
    result::{LexErr, LexResult},
};

pub fn mapping_for_str_state(
    state: &mut State,
    str_state: StrState,
    (pos, c): (Pos, char),
) -> Vec<LexResult> {
    match (str_state.escape, c) {
        (_, '\n') => {
            *state = State::Initial;
            vec![Err(LexErr::MissingClosingQuoteForStr(pos))]
        }

        (false, '"') => {
            *state = State::Initial;
            vec![Ok(str_state.tokenize(&pos))]
        }

        (_, c) => match str_state.try_append_char(&pos, c) {
            Ok(str_state) => {
                *state = State::Str(str_state);
                vec![]
            }
            Err(e) => vec![Err(e)],
        },
    }
}
