//! 字句解析で取り出されるトークンとその付帯情報の定義

mod locatable;
mod pos;
mod range;

use std::str::FromStr;

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
    Let,
    PrintI32,
    PrintStr,
}

impl FromStr for KeywordKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "let" => Ok(Self::Let),
            "print_int" => Ok(Self::PrintI32),
            "print_str" => Ok(Self::PrintStr),
            _ => Err(format!("Invalid keyword: {s}")),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Token {
    I32(Range, i32),
    AddOp(Pos),
    Equal(Pos),
    Str(Range, String),
    Keyword(Range, KeywordKind),
    Ident(Range, String),
    Newline(Pos),
}

impl Locatable for Token {
    fn locate(&self) -> Range {
        match self {
            Self::I32(r, _) | Self::Str(r, _) | Self::Keyword(r, _) | Self::Ident(r, _) => {
                r.clone()
            }

            Self::AddOp(p) | Self::Equal(p) | Self::Newline(p) => p.clone().into(),
        }
    }
}
