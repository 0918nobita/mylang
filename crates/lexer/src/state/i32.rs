use mylang_token::{range, Pos, Token};

#[derive(Clone, Debug, Default)]
pub struct I32State {
    start: Pos,
    end: Pos,
    acc: String,
}

impl I32State {
    pub fn new(pos: Pos, acc: String) -> Self {
        Self {
            start: pos.clone(),
            end: pos,
            acc,
        }
    }

    pub fn append_digit_char(&self, pos: Pos, c: char) -> Self {
        Self {
            start: self.start.clone(),
            end: pos,
            acc: format!("{}{c}", self.acc),
        }
    }

    pub fn tokenize(&self) -> Token {
        let i = self.acc.parse::<i32>().unwrap();
        Token::I32(range!(self.start.clone(), self.end.clone()), i)
    }
}

#[cfg(test)]
mod tests {
    use mylang_token::pos;

    use super::I32State;

    #[test]
    fn test_i32_state() {
        let state = I32State::default();
        let state = state.append_digit_char(pos!(0;0), '1');
        let state = state.append_digit_char(pos!(0;1), '2');
        let token = state.tokenize();
        insta::assert_debug_snapshot!((state, token));
    }
}
