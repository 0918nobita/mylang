mod iter;
mod mutation;
mod result;

use std::io::BufRead;

use mutation::{mapping, state::State};
use result::LexResult;
use utf8_chars::BufReadCharsExt;

use crate::iter::{map_with_state::MapWithStateExt, with_pos::WithPosExt};

pub fn lex<T: BufRead>(src: &mut T) -> impl Iterator<Item = LexResult> + '_ {
    src.chars()
        .map(|r| r.unwrap())
        .with_pos()
        .map_with_state(State::Initial, mapping)
        .flatten()
}
