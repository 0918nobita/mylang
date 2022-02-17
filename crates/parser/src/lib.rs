//! 構文解析器
//!
//! ## 構文定義
//!
//! ```text
//! program ::= NEWLINE* , [ stmt , { NEWLINE+ , stmt } , NEWLINE* ] ;
//! stmt ::= ( PRINT_INT | PRINT_STR ) , expr ;
//! expr ::= term , { PLUS , term } ;
//! term ::= INT_LIT | STRING_LIT ;
//! ```

mod parse;
mod result;

pub use parse::parse;
pub use result::{ParseErr, ParseResult};
