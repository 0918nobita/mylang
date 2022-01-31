//! 字句解析器

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

fn mapping(state: State, pos_c: (Pos, char)) -> (State, Vec<LexResult>) {
    match &state {
        State::Initial => default_lex(&pos_c),

        State::I32(i32_state) => match i32_lex(i32_state, &pos_c) {
            I32LexResult::Continued(i32_state) => (State::I32(i32_state), vec![]),

            I32LexResult::Interrupted(token) => {
                let mut results = vec![Ok(token)];
                let (next_state, next_results) = default_lex(&pos_c);
                results.extend(next_results);
                (next_state, results)
            }
        },

        State::Str(str_state) => match str_lex(str_state, &pos_c) {
            StrLexResult::Continued(str_state) => (State::Str(str_state), vec![]),

            StrLexResult::Completed(token) => (State::Initial, vec![Ok(token)]),

            StrLexResult::Err(str_state, err) => (State::Str(str_state), vec![Err(err)]),
        },

        State::Keyword(keyword_state) => match keyword_lex(keyword_state, &pos_c) {
            KeywordLexResult::Continued(keyword_state) => (State::Keyword(keyword_state), vec![]),

            KeywordLexResult::Interrupted(prev_result) => {
                let mut results = vec![prev_result];
                let (next_state, next_results) = default_lex(&pos_c);
                results.extend(next_results);
                (next_state, results)
            }
        },
    }
}

/// 字句解析を実行する
///
/// UTF-8 文字列から順番に文字を取り出し、字句解析器としての状態を持ち回りながら flatMap を行い、結果を順次流す。
///
/// ## Panics
///
/// UTF-8 文字列として不正な入力を得た場合には panic する。
pub fn lex<T: BufRead>(src: &mut T) -> impl Iterator<Item = LexResult> + '_ {
    src.chars()
        .map(|r| r.unwrap())
        .with_pos()
        // FIXME: 途中でも毎回 State をクローンして流すのは無駄にコストがデカいのでやめたい
        .map_with_state(State::Initial, mapping)
        // TODO: 最終状態を確認して、必要があれば追加で結果を流したい (そのためのアダプタを実装する)
        .map(|(_state, results)| results)
        .flatten()
}
