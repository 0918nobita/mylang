mod iter;

use std::io::BufRead;

use token::Token;
use utf8_chars::BufReadCharsExt;

use iter::WithPosExt;

pub fn tokenize<T: BufRead>(src: &mut T) -> Vec<Token> {
    for (pos, c) in src.chars().map(|r| r.unwrap()).with_pos() {
        println!("{:?} {:?}", pos, c);
    }
    vec![]
}
