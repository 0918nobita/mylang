use ast::pos::Pos;
use token::Token;

use super::keyword_state::KeywordState;
use crate::{
    mutation::{str_state::StrState, State},
    result::{TokenizeError, TokenizeResult},
};

pub fn mapping_for_keyword_state(
    state: &mut State,
    keyword_state: KeywordState,
    (pos, c): (Pos, char),
) -> Vec<TokenizeResult> {
    match c {
        '\n' => {
            *state = State::Initial;
            vec![
                keyword_state.try_tokenize(&pos),
                Ok(Token::Newline(pos.clone())),
            ]
        }
        '"' => {
            *state = State::Str(StrState::new(pos.clone(), false, String::new()));
            vec![keyword_state.try_tokenize(&pos)]
        }
        '+' => {
            *state = State::Initial;
            vec![
                keyword_state.try_tokenize(&pos),
                Ok(Token::AddOp(pos.clone())),
            ]
        }
        c if c.is_ascii_whitespace() => {
            *state = State::Initial;
            vec![keyword_state.try_tokenize(&pos)]
        }
        c if c.is_ascii() => {
            *state = State::Keyword(keyword_state.append_char(c));
            vec![]
        }
        _ => {
            *state = State::Initial;
            vec![Err(TokenizeError::ForbiddenChar(pos, c))]
        }
    }
}
