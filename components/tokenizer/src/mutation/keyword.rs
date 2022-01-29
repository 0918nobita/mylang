use ast::{pos::Pos, range::Range};
use token::{KeywordKind, Token};

use crate::{
    mutation::{str::StrState, State},
    result::{TokenizeError, TokenizeResult},
};

#[derive(Clone)]
pub struct KeywordState {
    start: Pos,
    acc: String,
}

impl KeywordState {
    pub fn new(start: Pos, acc: String) -> Self {
        Self { start, acc }
    }
}

fn try_tokenize_keyword(start: &Pos, pos: &Pos, acc: &str) -> TokenizeResult {
    if let Some(keyword_kind) = KeywordKind::parse(acc) {
        Ok(Token::Keyword(
            Range::new(start.clone(), pos.clone()),
            keyword_kind,
        ))
    } else {
        Err(TokenizeError::InvalidKeyword(
            Range::new(start.clone(), pos.clone()),
            acc.to_string(),
        ))
    }
}

pub fn mapping_for_keyword_state(
    state: &mut State,
    KeywordState { start, acc }: KeywordState,
    (pos, c): (Pos, char),
) -> Vec<TokenizeResult> {
    match c {
        '\n' => {
            *state = State::Initial;
            vec![
                try_tokenize_keyword(&start, &pos, &acc),
                Ok(Token::Newline(pos.clone())),
            ]
        }
        '"' => {
            *state = State::Str(StrState::new(pos.clone(), false, String::new()));
            vec![try_tokenize_keyword(&start, &pos, &acc)]
        }
        '+' => {
            *state = State::Initial;
            vec![
                try_tokenize_keyword(&start, &pos, &acc),
                Ok(Token::AddOp(pos.clone())),
            ]
        }
        c if c.is_ascii_whitespace() => {
            *state = State::Initial;
            vec![try_tokenize_keyword(&start, &pos, &acc)]
        }
        c if c.is_ascii() => {
            *state = State::Keyword(KeywordState::new(start, format!("{acc}{c}")));
            vec![]
        }
        _ => {
            *state = State::Initial;
            vec![Err(TokenizeError::ForbiddenChar(pos, c))]
        }
    }
}
