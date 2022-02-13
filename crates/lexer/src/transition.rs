//! 字句解析器内部の状態遷移の実装

mod i32;
mod initial;
mod keyword;
mod str;

use mylang_token::Pos;

use crate::{
    result::LexResult,
    state::State,
    transition::{
        i32::{i32_lex, I32LexResult},
        initial::initial_lex,
        keyword::{keyword_lex, KeywordLexResult},
        str::{str_lex, StrLexResult},
    },
};

pub fn transition(state: &State, pos_c: (Pos, char)) -> (State, Vec<LexResult>) {
    match state {
        State::Initial => initial_lex(pos_c),

        State::I32(i32_state) => match i32_lex(i32_state, pos_c.clone()) {
            I32LexResult::Continued(i32_state) => (State::I32(i32_state), vec![]),

            I32LexResult::Interrupted(token) => {
                let mut results = vec![Ok(token)];
                let (next_state, next_results) = initial_lex(pos_c);
                results.extend(next_results);
                (next_state, results)
            }
        },

        State::Str(str_state) => match str_lex(str_state, pos_c) {
            StrLexResult::Continued(str_state) => (State::Str(str_state), vec![]),

            StrLexResult::Completed(token) => (State::Initial, vec![Ok(token)]),

            StrLexResult::Err(str_state, err) => (State::Str(str_state), vec![Err(err)]),
        },

        State::Keyword(keyword_state) => match keyword_lex(keyword_state, pos_c.clone()) {
            KeywordLexResult::Continued(keyword_state) => (State::Keyword(keyword_state), vec![]),

            KeywordLexResult::Interrupted(prev_result) => {
                let mut results = vec![prev_result];
                let (next_state, next_results) = initial_lex(pos_c);
                results.extend(next_results);
                (next_state, results)
            }
        },
    }
}
