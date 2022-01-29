use ast::pos::Pos;
use token::Token;

use crate::{
    mutation::{keyword_state::KeywordState, state::State, str_state::StrState},
    result::{TokenizeError, TokenizeResult},
};

use super::i32_state::I32State;

pub fn mapping_for_i32_state(
    state: &mut State,
    i32_state: I32State,
    (pos, c): (Pos, char),
) -> Vec<TokenizeResult> {
    match c {
        '_' => vec![],
        '\n' => {
            *state = State::Initial;
            vec![
                Ok(i32_state.tokenize(&pos)),
                Ok(Token::Newline(pos.clone())),
            ]
        }
        '"' => {
            *state = State::Str(StrState::new(pos.clone(), false, String::new()));
            vec![Ok(i32_state.tokenize(&pos))]
        }
        '+' => vec![Ok(i32_state.tokenize(&pos)), Ok(Token::AddOp(pos.clone()))],
        c if c.is_ascii_digit() => {
            *state = State::I32(i32_state.append_digit_char(c));
            vec![]
        }
        c if c.is_ascii_alphabetic() => {
            *state = State::Keyword(KeywordState::new(pos.clone(), c.to_string()));
            vec![Ok(i32_state.tokenize(&pos))]
        }
        c if c.is_ascii_whitespace() => {
            *state = State::Initial;
            vec![Ok(i32_state.tokenize(&pos))]
        }
        _ => {
            *state = State::Initial;
            vec![Err(TokenizeError::ForbiddenChar(pos, c))]
        }
    }
}
