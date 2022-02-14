//! 字句解析で取り出されるトークンとその付帯情報の定義

mod locatable;
mod pos;
mod range;

use serde::{Deserialize, Serialize};

pub use locatable::Locatable;
pub use pos::Pos;
pub use range::Range;

/// [`Pos`] を簡単に生成するためのマクロ
#[macro_export]
macro_rules! pos {
    ($line:expr ; $char:expr) => {
        mylang_token::Pos::new($line, $char)
    };
}

/// [`Range`] を簡単に生成するためのマクロ
#[macro_export]
macro_rules! range {
    ($start_line:expr ; $start_char:expr, $end_line:expr ; $end_char:expr) => {
        mylang_token::Range::new(
            mylang_token::pos!($start_line;$start_char),
            mylang_token::pos!($end_line;$end_char),
        )
    };
    ($start:expr, $end:expr) => {
        mylang_token::Range::new($start, $end)
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub enum KeywordKind {
    PrintI32,
    PrintStr,
}

impl KeywordKind {
    pub fn parse(str: &str) -> Option<Self> {
        match str {
            "print_int" => Some(KeywordKind::PrintI32),
            "print_str" => Some(KeywordKind::PrintStr),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Token {
    I32(Range, i32),
    AddOp(Pos),
    Str(Range, String),
    Keyword(Range, KeywordKind),
    Newline(Pos),
}

impl Locatable for Token {
    fn locate(&self) -> Range {
        match self {
            Token::I32(r, _) | Token::Str(r, _) | Token::Keyword(r, _) => r.clone(),

            Token::AddOp(p) | Token::Newline(p) => p.clone().into(),
        }
    }
}
