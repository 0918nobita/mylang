mod error;
mod iter;
mod state;

use std::io::BufRead;

use ast::{pos::Pos, range::Range};
use error::TokenizeError;
use state::State;
use token::{KeywordKind, Token};
use utf8_chars::BufReadCharsExt;

use iter::{map_with_state::MapWithStateExt, with_pos::WithPosExt};
use state::{I32State, KeywordState, StrState};

type TokenizeResult = Result<Token, TokenizeError>;

fn tokenize_i32(I32State { start, acc }: I32State, pos: &Pos) -> Token {
    let i = acc.parse::<i32>().unwrap();
    Token::I32(Range::new(start.clone(), pos.clone()), i)
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

fn mapping_for_initial_state(state: &mut State, (pos, c): (Pos, char)) -> Vec<TokenizeResult> {
    match c {
        '\n' => vec![Ok(Token::Newline(pos))],
        '"' => {
            *state = State::Str(StrState::new(pos, false, String::new()));
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
            vec![Err(TokenizeError::ForbiddenChar(pos, c))]
        }
    }
}

fn mapping_for_i32_state(
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

fn mapping_for_str_state(
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

fn mapping_for_keyword_state(
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

fn mapping(state: &mut State, pos_c: (Pos, char)) -> Vec<TokenizeResult> {
    match state.clone() {
        State::Initial => mapping_for_initial_state(state, pos_c),
        State::I32(i32_state) => mapping_for_i32_state(state, i32_state, pos_c),
        State::Str(str_state) => mapping_for_str_state(state, str_state, pos_c),
        State::Keyword(keyword_state) => mapping_for_keyword_state(state, keyword_state, pos_c),
    }
}

pub fn tokenize<T: BufRead>(src: &mut T) -> impl Iterator<Item = TokenizeResult> + '_ {
    src.chars()
        .map(|r| r.unwrap())
        .with_pos()
        .map_with_state(State::Initial, mapping)
        .flatten()
}
