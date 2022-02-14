use mylang_token::{range, Pos, Token};

#[derive(Clone, Debug)]
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
    use mylang_token::{pos, Token};
    use proptest::prelude::proptest;

    use super::I32State;

    proptest! {
        #[test]
        fn test_i32_state(line1: u32, char1: u32, line2: u32, char2: u32) {
            let state = I32State::new(pos!(line1;char1), String::new());
            let state = state.append_digit_char(pos!(line2;char2), '1');
            let token = state.tokenize();
            assert!(matches!(
                token,
                Token::I32(range, 1)
                    if *range.start_ref() == pos!(line1;char1) && *range.end_ref() == pos!(line2;char2)
            ));
        }
    }
}
