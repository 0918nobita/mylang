use ast::pos::Pos;
use token::Token;

use crate::{
    mutation::{i32_state::I32State, keyword_state::KeywordState, str_state::StrState, State},
    result::{LexErr, LexResult},
};

pub fn mapping_for_initial_state(state: &mut State, (pos, c): (Pos, char)) -> Vec<LexResult> {
    match c {
        '\n' => vec![Ok(Token::Newline(pos))],
        '"' => {
            *state = State::Str(StrState::new(pos));
            vec![]
        }
        '+' => vec![Ok(Token::AddOp(pos))],
        c if c.is_ascii_whitespace() => vec![],
        c if c.is_ascii_digit() => {
            *state = State::I32(I32State::new(pos, c.to_string()));
            vec![]
        }
        c if c.is_ascii_alphabetic() => {
            *state = State::Keyword(KeywordState::new(pos, c.to_string()));
            vec![]
        }
        _ => {
            *state = State::Initial;
            vec![Err(LexErr::ForbiddenChar(pos, c))]
        }
    }
}
