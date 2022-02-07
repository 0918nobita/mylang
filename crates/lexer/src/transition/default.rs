//! 初期状態のとき、または他の状態で想定外の文字が出現したときに行う状態遷移

use token::{Pos, Token};

use crate::{
    result::{LexErr, LexResult},
    state::{i32::I32State, keyword::KeywordState, str::StrState, State},
};

pub fn default_transition((pos, c): (Pos, char)) -> (State, Vec<LexResult>) {
    match c {
        '\n' => (State::Initial, vec![Ok(Token::Newline(pos))]),

        '"' => (State::Str(StrState::new(pos)), vec![]),

        '+' => (State::Initial, vec![Ok(Token::AddOp(pos))]),

        c if c.is_ascii_whitespace() => (State::Initial, vec![]),

        c if c.is_ascii_digit() => (State::I32(I32State::new(pos, c.to_string())), vec![]),

        c if c.is_ascii_alphabetic() => (
            State::Keyword(KeywordState::new(pos, c.to_string())),
            vec![],
        ),

        _ => (State::Initial, vec![Err(LexErr::ForbiddenChar(pos, c))]),
    }
}
