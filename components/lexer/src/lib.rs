mod iter;
mod lex;
mod result;
mod state;

use std::io::BufRead;

use ast::pos::Pos;
use utf8_chars::BufReadCharsExt;

use crate::{
    iter::{map_with_state::MapWithStateExt, with_pos::WithPosExt},
    lex::{
        default::default_lex,
        i32::{i32_lex, I32LexResult},
        keyword::{keyword_lex, KeywordLexResult},
        str::{str_lex, StrLexResult},
    },
    result::LexResult,
    state::State,
};

fn mapping(state: &mut State, pos_c: (Pos, char)) -> Vec<LexResult> {
    match &state {
        State::Initial => default_lex(state, &pos_c),

        State::I32(i32_state) => match i32_lex(i32_state, &pos_c) {
            I32LexResult::Continued(i32_state) => {
                *state = State::I32(i32_state);
                vec![]
            }

            I32LexResult::Interrupted(token) => {
                let mut result = vec![Ok(token)];
                result.extend(default_lex(state, &pos_c));
                result
            }
        },

        State::Str(str_state) => match str_lex(str_state, &pos_c) {
            StrLexResult::Continued(str_state) => {
                *state = State::Str(str_state);
                vec![]
            }

            StrLexResult::Interrupted(result) => {
                *state = State::Initial;
                vec![result]
            }
        },

        State::Keyword(keyword_state) => match keyword_lex(keyword_state, &pos_c) {
            KeywordLexResult::Continued(keyword_state) => {
                *state = State::Keyword(keyword_state);
                vec![]
            }

            KeywordLexResult::Interrupted(prev_result) => {
                let mut result = vec![prev_result];
                result.extend(default_lex(state, &pos_c));
                result
            }
        },
    }
}

pub fn lex<T: BufRead>(src: &mut T) -> impl Iterator<Item = LexResult> + '_ {
    src.chars()
        .map(|r| r.unwrap())
        .with_pos()
        .map_with_state(State::Initial, mapping)
        .flatten()
}
