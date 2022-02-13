use mylang_lexer::LexErr;
use mylang_parser::ParseErr;
use serde_json::{json, Value as JsonValue};

use super::range::locatable_to_lsp_range;

pub fn lex_err_to_diagnostic(err: &LexErr) -> JsonValue {
    let range = locatable_to_lsp_range(err);
    let message = format!("(Tokenization Error) {}", err);
    json!({ "range": range, "message": message })
}

pub fn parse_err_to_diagnostic(err: ParseErr) -> JsonValue {
    let range = locatable_to_lsp_range(&err);
    let message = format!("(Syntax Error) {}", err);
    json!({ "range": range, "message": message })
}
