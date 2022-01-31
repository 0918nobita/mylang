use ast::pos::Pos;

use crate::{result::LexResult, state::str::StrState};

pub enum StrLexResult {
    Continued(StrState),
    Interrupted(LexResult),
}

pub fn str_lex(str_state: &StrState, (pos, c): &(Pos, char)) -> StrLexResult {
    match (str_state.escape, c) {
        (false, '"') => StrLexResult::Interrupted(Ok(str_state.tokenize(pos))),

        (_, c) => match str_state.try_append_char(pos, *c) {
            Ok(str_state) => StrLexResult::Continued(str_state),
            Err(e) => StrLexResult::Interrupted(Err(e)),
        },
    }
}
