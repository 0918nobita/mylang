mod locatable;
mod pos;
mod range;

use serde::{Deserialize, Serialize};

pub use locatable::Locatable;
pub use pos::Pos;
pub use range::Range;

#[macro_export]
macro_rules! pos {
    ($line:expr ; $char:expr) => {
        token::Pos::new($line, $char)
    };
}

#[macro_export]
macro_rules! range {
    ($start_line:expr ; $start_char:expr, $end_line:expr ; $end_char:expr) => {
        token::Range::new(
            token::pos!($start_line;$start_char),
            token::pos!($end_line;$end_char),
        )
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
