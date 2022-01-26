use ast::{
    pos::Pos,
    range::{Locatable, Range},
};

#[allow(dead_code)]
#[derive(Debug)]
enum Token {
    I32(Range, i32),
    AddOp(Pos),
    Str(Range, String),
    PrintI32Keyword(Range),
    PrintStrKeyword(Range),
}

impl Locatable for Token {
    fn get_range(&self) -> Option<Range> {
        match self {
            Token::I32(r, _)
            | Token::Str(r, _)
            | Token::PrintI32Keyword(r)
            | Token::PrintStrKeyword(r) => Some(r.clone()),
            Token::AddOp(p) => Some(p.clone().into()),
        }
    }
}

fn main() {
    println!("mylang parser")
}
