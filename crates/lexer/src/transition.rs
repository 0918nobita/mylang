//! 字句解析器内部の状態遷移の実装

mod i32;
mod initial;
mod str;
mod symbol;

use mylang_token::{Pos, Token};

use crate::{
    state::State,
    transition::{
        i32::{i32_lex, I32LexResult},
        initial::initial_lex,
        str::{str_lex, StrLexResult},
        symbol::{symbol_lex, SymbolLexResult},
    },
    LexResult,
};

pub fn transition(state: &State, pos_c: (Pos, char)) -> (State, Vec<LexResult<Token>>) {
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

        State::Symbol(symbol_state) => match symbol_lex(symbol_state, pos_c.clone()) {
            SymbolLexResult::Continued(symbol_state) => (State::Symbol(symbol_state), vec![]),

            SymbolLexResult::Interrupted(prev_token) => {
                let mut results = vec![Ok(prev_token)];
                let (next_state, next_results) = initial_lex(pos_c);
                results.extend(next_results);
                (next_state, results)
            }
        },
    }
}
