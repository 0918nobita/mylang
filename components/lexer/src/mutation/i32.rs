use ast::pos::Pos;
use token::Token;

use crate::{
    mutation::{keyword_state::KeywordState, state::State, str_state::StrState},
    result::{LexErr, LexResult},
};

use super::i32_state::I32State;

pub fn mapping_for_i32_state(
    state: &mut State,
    i32_state: I32State,
    (pos, c): (Pos, char),
) -> Vec<LexResult> {
    match c {
        '_' => vec![],

        // FIXME: Newline 関連の処理をここに書かない
        '\n' => {
            *state = State::Initial;
            vec![
                Ok(i32_state.tokenize(&pos)),
                Ok(Token::Newline(pos.clone())),
            ]
        }

        // FIXME: Str 関連の処理をここに書かない
        '"' => {
            *state = State::Str(StrState::new(pos.clone()));
            vec![Ok(i32_state.tokenize(&pos))]
        }

        // FIXME: AddOp 関連の処理をここに書かない
        '+' => vec![Ok(i32_state.tokenize(&pos)), Ok(Token::AddOp(pos.clone()))],

        c if c.is_ascii_digit() => {
            *state = State::I32(i32_state.append_digit_char(c));
            vec![]
        }

        // FIXME: Keyword 関連の処理をここに書かない
        c if c.is_ascii_alphabetic() => {
            *state = State::Keyword(KeywordState::new(pos.clone(), c.to_string()));
            vec![Ok(i32_state.tokenize(&pos))]
        }

        // FIXME: Initial state 関連の処理をここに書かない
        c if c.is_ascii_whitespace() => {
            *state = State::Initial;
            vec![Ok(i32_state.tokenize(&pos))]
        }

        // FIXME: 異常系の処理をここに書かない
        _ => {
            *state = State::Initial;
            vec![Err(LexErr::ForbiddenChar(pos, c))]
        }
    }
}
