use ast::{pos::Pos, range::Range};
use token::Token;

use crate::{
    mutation::{keyword::KeywordState, str::StrState, State},
    result::{TokenizeError, TokenizeResult},
};

#[derive(Clone)]
pub struct I32State {
    start: Pos,
    acc: String,
}

impl I32State {
    pub fn new(start: Pos, acc: String) -> Self {
        Self { start, acc }
    }
}

fn tokenize_i32(I32State { start, acc }: I32State, pos: &Pos) -> Token {
    let i = acc.parse::<i32>().unwrap();
    Token::I32(Range::new(start, pos.clone()), i)
}

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
                Ok(tokenize_i32(i32_state, &pos)),
                Ok(Token::Newline(pos.clone())),
            ]
        }
        '"' => {
            *state = State::Str(StrState::new(pos.clone(), false, String::new()));
            vec![Ok(tokenize_i32(i32_state, &pos))]
        }
        '+' => vec![
            Ok(tokenize_i32(i32_state, &pos)),
            Ok(Token::AddOp(pos.clone())),
        ],
        c if c.is_ascii_digit() => {
            *state = State::I32(I32State::new(
                i32_state.start,
                format!("{}{c}", i32_state.acc),
            ));
            vec![]
        }
        c if c.is_ascii_alphabetic() => {
            *state = State::Keyword(KeywordState::new(pos.clone(), c.to_string()));
            vec![Ok(tokenize_i32(i32_state, &pos))]
        }
        c if c.is_ascii_whitespace() => {
            *state = State::Initial;
            vec![Ok(tokenize_i32(i32_state, &pos))]
        }
        _ => {
            *state = State::Initial;
            vec![Err(TokenizeError::ForbiddenChar(pos, c))]
        }
    }
}
