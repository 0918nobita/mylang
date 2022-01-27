use ast::{
    pos::Pos,
    range::{Locatable, Range},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Token {
    I32(Range, i32),
    AddOp(Pos),
    Str(Range, String),
    PrintI32Keyword(Range),
    PrintStrKeyword(Range),
}

impl Locatable for Token {
    fn locate(&self) -> Range {
        match self {
            Token::I32(r, _)
            | Token::Str(r, _)
            | Token::PrintI32Keyword(r)
            | Token::PrintStrKeyword(r) => r.clone(),

            Token::AddOp(p) => p.clone().into(),
        }
    }
}
