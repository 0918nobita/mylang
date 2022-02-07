use token::{Pos, Token};

use crate::{result::LexErr, state::str::StrState};

pub enum StrLexResult {
    Continued(StrState),
    Completed(Token),
    Err(StrState, LexErr),
}

pub fn str_lex(str_state: &StrState, (pos, c): (Pos, char)) -> StrLexResult {
    match (str_state.escape, c) {
        (false, '"') => StrLexResult::Completed(str_state.tokenize()),

        (_, c) => match str_state.try_append_char(pos, c) {
            Ok(str_state) => StrLexResult::Continued(str_state),
            Err(e) => {
                let mut str_state = str_state.clone();
                str_state.escape = false;
                StrLexResult::Err(str_state, e)
            }
        },
    }
}
