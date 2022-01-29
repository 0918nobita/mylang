mod iter;
mod mutation;
mod result;

use std::io::BufRead;

use mutation::{mapping, State};
use result::TokenizeResult;
use utf8_chars::BufReadCharsExt;

use crate::iter::{map_with_state::MapWithStateExt, with_pos::WithPosExt};

pub fn tokenize<T: BufRead>(src: &mut T) -> impl Iterator<Item = TokenizeResult> + '_ {
    src.chars()
        .map(|r| r.unwrap())
        .with_pos()
        .map_with_state(State::Initial, mapping)
        .flatten()
}
