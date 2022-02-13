//! 字句解析器

mod result;
mod state;
mod transition;
mod with_pos;

use std::io::BufRead;

use token::Pos;
use utf8_chars::BufReadCharsExt;
pub use with_pos::WithPosExt;

use crate::{result::LexResult, state::State, transition::transition};
pub use result::LexErr;

/// 「文字と位置のイテレータを、トークン取得結果のイテレータに変換するアダプタ」として字句解析器を実装している
pub struct Lex<I>
where
    I: Iterator<Item = (Pos, char)> + Sized,
{
    iter: I,
    state: Option<State>,
}

impl<I> Iterator for Lex<I>
where
    I: Iterator<Item = (Pos, char)>,
{
    type Item = Vec<LexResult>;

    /// 内部イテレータと内部状態をもとに、次のトークンを取り出す
    fn next(&mut self) -> Option<Self::Item> {
        let state = self.state.as_ref()?.clone();

        if let Some((pos, c)) = self.iter.next() {
            let (next_state, results) = transition(&state, (pos, c));
            self.state = Some(next_state);
            Some(results)
        } else {
            match state {
                State::Initial => {
                    self.state = None;
                    None
                }
                State::I32(i32_state) => {
                    self.state = None;
                    Some(vec![Ok(i32_state.tokenize())])
                }
                State::Str(str_state) => {
                    self.state = None;
                    Some(vec![Err(LexErr::MissingClosingQuoteForStr(str_state.end))])
                }
                State::Keyword(keyword_state) => {
                    self.state = None;
                    Some(vec![keyword_state.try_tokenize()])
                }
            }
        }
    }
}

/// 字句解析器に変換可能なイテレータを表すトレイト
pub trait LexExt: Iterator<Item = (Pos, char)> + Sized {
    /// 位置と文字のイテレータを、字句解析器に変換する
    fn lex(self) -> Lex<Self>;
}

impl<I> LexExt for I
where
    I: Iterator<Item = (Pos, char)> + Sized,
{
    fn lex(self) -> Lex<Self> {
        Lex {
            iter: self,
            state: Some(State::Initial),
        }
    }
}

/// 字句解析を実行する
///
/// UTF-8 文字列から順番に文字を取り出し、字句解析器としての状態を持ち回りながら flatMap を行い、結果を順次流す。
///
/// ## Panics
///
/// UTF-8 文字列として不正な入力を得た場合には panic する。
pub fn lex<T: BufRead>(src: &mut T) -> impl Iterator<Item = LexResult> + '_ {
    src.chars().map(|r| r.unwrap()).with_pos().lex().flatten()
}
