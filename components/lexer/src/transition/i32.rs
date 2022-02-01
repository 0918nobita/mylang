use ast::pos::Pos;
use token::Token;

use crate::state::i32::I32State;

pub enum I32LexResult {
    Continued(I32State),
    Interrupted(Token),
}

pub fn i32_lex(i32_state: &I32State, (pos, c): (Pos, char)) -> I32LexResult {
    match c {
        '_' => I32LexResult::Continued(i32_state.clone()),

        c if c.is_ascii_digit() => I32LexResult::Continued(i32_state.append_digit_char(pos, c)),

        _ => I32LexResult::Interrupted(i32_state.tokenize()),
    }
}
