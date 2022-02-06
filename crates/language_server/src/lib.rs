pub mod receive;
pub mod send;

use ast::range::Locatable;
use lexer::{result::LexErr, with_pos::WithPosExt, LexExt};
use log::warn;
use lsp::LspMessage;
use serde_json::{json, Value as JsonValue};
use token::Token;
use tokio::sync::mpsc;
use valq::query_value;

fn text_document_uri(params_obj: &JsonValue) -> Option<&JsonValue> {
    query_value!(params_obj.textDocument.uri)
}

fn lex(text: &str) -> (Vec<Token>, Vec<LexErr>) {
    let (tokens, errors): (Vec<_>, Vec<_>) = text
        .chars()
        .with_pos()
        .lex()
        .flatten()
        .partition(Result::is_ok);

    let tokens = tokens.into_iter().map(Result::unwrap).collect::<Vec<_>>();

    let errors = errors
        .into_iter()
        .map(Result::unwrap_err)
        .collect::<Vec<_>>();
    (tokens, errors)
}

fn locatable_to_json_range<L>(locatable: &L) -> JsonValue
where
    L: Locatable,
{
    let range = locatable.locate();
    let start = range.start;
    let end = range.end;
    json!({
        "start": { "line": start.line, "character": start.column },
        "end": { "line": end.line, "character": end.column },
    })
}

fn lex_err_to_diagnostic(err: &LexErr) -> JsonValue {
    let range = locatable_to_json_range(err);
    let message = err.to_string();
    json!({ "range": range, "message": message })
}

async fn lex_and_report_errs(
    responder: &mpsc::Sender<LspMessage>,
    uri: &JsonValue,
    text: &str,
) -> anyhow::Result<()> {
    let (_tokens, errors) = lex(text);

    let diagnostics: Vec<_> = errors.iter().map(lex_err_to_diagnostic).collect();

    responder
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

pub async fn handle_msg(
    responder: &mpsc::Sender<LspMessage>,
    publish_diagnostics_supported: &mut bool,
    msg: LspMessage,
) -> anyhow::Result<()> {
    match msg {
        LspMessage::Request { id, method, params } if method == "initialize" => {
            *publish_diagnostics_supported =
                query_value!(params.capabilities.textDocument.publishDiagnostics).is_some();

            responder
                .send(LspMessage::Response {
                    id,
                    result: json!({
                        "capabilities": {
                            "textDocumentSync": 1,
                        }
                    }),
                })
                .await?
        }

        LspMessage::Notification { method, params }
            if *publish_diagnostics_supported && (method == "textDocument/didOpen") =>
        {
            if let (Some(uri), Some(text)) = (
                text_document_uri(&params),
                query_value!(params.textDocument.text -> str),
            ) {
                lex_and_report_errs(responder, uri, text).await?
            } else {
                warn!("Skipped: ({:?}) {:?}", method, params)
            }
        }

        LspMessage::Notification { method, params }
            if *publish_diagnostics_supported && (method == "textDocument/didChange") =>
        {
            if let (Some(uri), Some(text)) = (
                text_document_uri(&params),
                query_value!(params.contentChanges[0].text -> str),
            ) {
                lex_and_report_errs(responder, uri, text).await?
            } else {
                warn!("Skipped: ({:?}) {:?}", method, params)
            }
        }

        LspMessage::Notification { method, params }
            if *publish_diagnostics_supported && method == "textDocument/didClose" =>
        {
            if let Some(uri) = text_document_uri(&params) {
                responder
                    .send(LspMessage::Notification {
                        method: "textDocument/publishDiagnostics".to_owned(),
                        params: json!({ "uri": uri, "diagnostics": [] }),
                    })
                    .await?
            }
        }

        _ => (),
    }

    Ok(())
}
