use ast::pos::Pos;
use token::Token;

use crate::{
    result::{LexErr, LexResult},
    state::{i32::I32State, keyword::KeywordState, str::StrState, State},
};

pub fn default_lex((pos, c): &(Pos, char)) -> (State, Vec<LexResult>) {
    match c {
        '\n' => (State::Initial, vec![Ok(Token::Newline(pos.clone()))]),

        '"' => (State::Str(StrState::new(pos.clone())), vec![]),

        '+' => (State::Initial, vec![Ok(Token::AddOp(pos.clone()))]),

        c if c.is_ascii_whitespace() => (State::Initial, vec![]),

        c if c.is_ascii_digit() => (
            State::I32(I32State::new(pos.clone(), c.to_string())),
            vec![],
        ),

        c if c.is_ascii_alphabetic() => (
            State::Keyword(KeywordState::new(pos.clone(), c.to_string())),
            vec![],
        ),

        _ => (
            State::Initial,
            vec![Err(LexErr::ForbiddenChar(pos.clone(), *c))],
        ),
    }
}
