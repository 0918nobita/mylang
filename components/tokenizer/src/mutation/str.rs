use ast::{pos::Pos, range::Range};
use token::Token;

use crate::{
    mutation::State,
    result::{TokenizeError, TokenizeResult},
};

#[derive(Clone)]
pub struct StrState {
    start: Pos,
    escape: bool,
    acc: String,
}

impl StrState {
    pub fn new(start: Pos, escape: bool, acc: String) -> Self {
        Self { start, escape, acc }
    }
}

pub fn mapping_for_str_state(
    state: &mut State,
    StrState { start, escape, acc }: StrState,
    (pos, c): (Pos, char),
) -> Vec<TokenizeResult> {
    match (escape, c) {
        (_, '\n') => {
            *state = State::Initial;
            vec![Err(TokenizeError::MissingClosingQuoteForStr(pos))]
        }
        (false, '\\') => {
            *state = State::Str(StrState::new(start, true, acc));
            vec![]
        }
        (true, '"') => {
            *state = State::Str(StrState::new(start, false, format!("{acc}\"")));
            vec![]
        }
        (true, 'n') => {
            *state = State::Str(StrState::new(start, false, format!("{acc}\n")));
            vec![]
        }
        (true, c) => {
            *state = State::Initial;
            vec![Err(TokenizeError::InvalidEscapeSequence(pos, c))]
        }
        (false, '"') => {
            *state = State::Initial;
            vec![Ok(Token::Str(Range::new(start, pos), acc))]
        }
        (false, c) => {
            *state = State::Str(StrState::new(start, false, format!("{acc}{c}")));
            vec![]
        }
    }
}
