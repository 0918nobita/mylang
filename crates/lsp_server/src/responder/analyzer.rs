use actix::Addr;
use itertools::Itertools;
use lexer::{LexErr, LexExt, WithPosExt};
use serde_json::{json, Value as JsonValue};
use token::Token;

use crate::{message::LspMessage, sender::Sender};

use super::diagnostic::{lex_err_to_diagnostic, parse_err_to_diagnostic};

fn lex(text: &str) -> (Vec<Token>, Vec<LexErr>) {
    text.chars().with_pos().lex().flatten().partition_result()
}

pub async fn analyze_src(sender: Addr<Sender>, uri: &JsonValue, text: &str) -> anyhow::Result<()> {
    let (tokens, errors) = lex(text);

    let mut diagnostics: Vec<_> = errors.iter().map(lex_err_to_diagnostic).collect();

    let (_stmts, errors): (Vec<_>, Vec<_>) = parser::parse(tokens.into_iter())
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
