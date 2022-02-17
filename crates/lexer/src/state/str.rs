use mylang_token::{range, Pos, Token};

use crate::{LexErr, LexResult};

#[derive(Clone, Debug, Default)]
pub struct StrState {
    start: Pos,
    pub end: Pos,
    pub escape: bool,
    acc: String,
}

impl StrState {
    pub fn new(pos: Pos) -> Self {
        Self {
            start: pos.clone(),
            end: pos,
            escape: false,
            acc: String::new(),
        }
    }

    pub fn try_append_char(&self, pos: Pos, c: char) -> LexResult<Self> {
        match (self.escape, c) {
            (_, '\n') => Err(LexErr::ForbiddenChar(pos, c)),

            (true, c @ ('n' | '\\' | '"')) => Ok(Self {
                start: self.start.clone(),
                end: pos,
                escape: false,
                acc: format!("{}{c}", self.acc),
            }),

            (true, _) => Err(LexErr::InvalidEscapeSequence(pos, c)),

            (false, '\\') => Ok(Self {
                start: self.start.clone(),
                end: pos,
                escape: true,
                acc: self.acc.clone(),
            }),

            (false, c) => Ok(Self {
                start: self.start.clone(),
                end: pos,
                escape: false,
                acc: format!("{}{c}", self.acc),
            }),
        }
    }

    pub fn tokenize(&self) -> Token {
        if self.escape {
            panic!("Illigal state")
        }
        Token::Str(
            range!(self.start.clone(), self.end.clone()),
            self.acc.clone(),
        )
    }
}

#[cfg(test)]
mod tests {
    use mylang_token::pos;

    use super::StrState;

    #[test]
    fn newline() {
        let state = StrState::default();
        let state = state.try_append_char(pos!(0;0), '\n').unwrap_err();
        insta::assert_debug_snapshot!(state);
    }

    #[test]
    fn escape() {
        let state = StrState::default();
        let state = state.try_append_char(pos!(0;0), '\\').unwrap();
        insta::assert_debug_snapshot!(state);
    }

    #[test]
    #[should_panic]
    fn illigal_state() {
        let state = StrState::default();
        let state = state.try_append_char(pos!(0;0), '\\').unwrap();
        state.tokenize();
    }
}
