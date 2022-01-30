use ast::pos::Pos;
use token::Token;

use super::keyword_state::KeywordState;
use crate::{
    mutation::{str_state::StrState, State},
    result::{LexErr, LexResult},
};

pub fn mapping_for_keyword_state(
    state: &mut State,
    keyword_state: KeywordState,
    (pos, c): (Pos, char),
) -> Vec<LexResult> {
    match c {
        // FIXME: Newline 関連の処理をここに書かない
        '\n' => {
            *state = State::Initial;
            vec![
                keyword_state.try_tokenize(&pos),
                Ok(Token::Newline(pos.clone())),
            ]
        }

        // FIXME: Str 関連の処理をここに書かない
        '"' => {
            *state = State::Str(StrState::new(pos.clone()));
            vec![keyword_state.try_tokenize(&pos)]
        }

        // FIXME: AddOp 関連の処理をここに書かない
        '+' => {
            *state = State::Initial;
            vec![
                keyword_state.try_tokenize(&pos),
                Ok(Token::AddOp(pos.clone())),
            ]
        }

        // FIXME: Initial state 関連の処理をここに書かない
        c if c.is_ascii_whitespace() => {
            *state = State::Initial;
            vec![keyword_state.try_tokenize(&pos)]
        }

        c if c.is_ascii() => {
            *state = State::Keyword(keyword_state.append_char(c));
            vec![]
        }

        // FIXME: 異常系の処理をここに書かない
        _ => {
            *state = State::Initial;
            vec![Err(LexErr::ForbiddenChar(pos, c))]
        }
    }
}
