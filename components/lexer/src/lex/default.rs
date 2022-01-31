use ast::pos::Pos;
use token::Token;

use crate::{
    result::{LexErr, LexResult},
    state::{i32::I32State, keyword::KeywordState, str::StrState, State},
};

pub fn default_lex(state: &mut State, (pos, c): &(Pos, char)) -> Vec<LexResult> {
    match c {
        '\n' => {
            *state = State::Initial;
            vec![Ok(Token::Newline(pos.clone()))]
        }

        '"' => {
            *state = State::Str(StrState::new(pos.clone()));
            vec![]
        }

        '+' => {
            *state = State::Initial;
            vec![Ok(Token::AddOp(pos.clone()))]
        }

        c if c.is_ascii_whitespace() => {
            *state = State::Initial;
            vec![]
        }

        c if c.is_ascii_digit() => {
            *state = State::I32(I32State::new(pos.clone(), c.to_string()));
            vec![]
        }

        c if c.is_ascii_alphabetic() => {
            *state = State::Keyword(KeywordState::new(pos.clone(), c.to_string()));
            vec![]
        }

        _ => {
            *state = State::Initial;
            vec![Err(LexErr::ForbiddenChar(pos.clone(), *c))]
        }
    }
}
