use mylang_token::Pos;

use crate::{result::LexResult, state::keyword::KeywordState};

pub enum KeywordLexResult {
    Continued(KeywordState),
    Interrupted(LexResult),
}

pub fn keyword_lex(keyword_state: &KeywordState, (pos, c): (Pos, char)) -> KeywordLexResult {
    match c {
        c if c.is_ascii() && !c.is_ascii_whitespace() => {
            KeywordLexResult::Continued(keyword_state.append_char(pos, c))
        }

        _ => KeywordLexResult::Interrupted(keyword_state.try_tokenize()),
    }
}
