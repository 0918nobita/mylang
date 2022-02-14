use actix::Addr;
use itertools::Itertools;
use mylang_lexer::{lex, LexErr};
use mylang_token::Token;
use serde_json::{json, Value as JsonValue};

use crate::{message::LspMessage, sender::Sender};

use super::diagnostic::{lex_err_to_diagnostic, parse_err_to_diagnostic};

fn lex_all(text: &str) -> (Vec<Token>, Vec<LexErr>) {
    lex(text.chars()).partition_result()
}

pub async fn analyze_src(sender: Addr<Sender>, uri: &JsonValue, text: &str) -> anyhow::Result<()> {
    let (tokens, errors) = lex_all(text);

    let mut diagnostics: Vec<_> = errors.iter().map(lex_err_to_diagnostic).collect();

    let (_stmts, errors): (Vec<_>, Vec<_>) = mylang_parser::parse(tokens.into_iter())
        .into_iter()
        .partition_result();

    let diagnostics_from_parser = errors
        .into_iter()
        .map(parse_err_to_diagnostic)
        .collect::<Vec<_>>();

    diagnostics.extend(diagnostics_from_parser);

    sender
        .send(LspMessage::Notification {
            method: "textDocument/publishDiagnostics".to_owned(),
            params: json!({
                "uri": uri,
                "diagnostics": diagnostics,
            }),
        })
        .await?;

    Ok(())
}
