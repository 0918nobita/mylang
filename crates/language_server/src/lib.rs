pub mod message;
pub mod receive;
pub mod send;

use ast::range::Locatable;
use lexer::{result::LexErr, with_pos::WithPosExt, LexExt};
use log::warn;
use serde_json::{json, Value as JsonValue};
use token::Token;
use tokio::sync::mpsc;

use crate::message::Message;

fn publish_diagnostics(params_obj: &JsonValue) -> Option<&JsonValue> {
    let params_obj = params_obj.as_object()?;
    let capabilities = params_obj.get("capabilities")?.as_object()?;
    let text_document = capabilities.get("textDocument")?.as_object()?;
    text_document.get("publishDiagnostics")
}

fn text_document_uri(params_obj: &JsonValue) -> Option<&JsonValue> {
    let params_obj = params_obj.as_object()?;
    let text_document = params_obj.get("textDocument")?.as_object()?;
    text_document.get("uri")
}

fn text_document_text(params_obj: &JsonValue) -> Option<&str> {
    let params_obj = params_obj.as_object()?;
    let text_document = params_obj.get("textDocument")?.as_object()?;
    text_document.get("text")?.as_str()
}

fn changed_text(params_obj: &JsonValue) -> Option<&str> {
    let params_obj = params_obj.as_object()?;
    params_obj
        .get("contentChanges")?
        .as_array()?
        .get(0)?
        .as_object()?
        .get("text")?
        .as_str()
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
    responder: &mpsc::Sender<Message>,
    uri: &JsonValue,
    text: &str,
) -> anyhow::Result<()> {
    let (_tokens, errors) = lex(text);

    let diagnostics: Vec<_> = errors.iter().map(lex_err_to_diagnostic).collect();

    responder
        .send(Message::Notification {
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
    responder: &mpsc::Sender<Message>,
    publish_diagnostics_supported: &mut bool,
    msg: Message,
) -> anyhow::Result<()> {
    match msg {
        Message::Request { id, method, params } if method == "initialize" => {
            *publish_diagnostics_supported = publish_diagnostics(&params).is_some();

            responder
                .send(Message::Response {
                    id,
                    result: json!({
                        "capabilities": {
                            "textDocumentSync": 1,
                        }
                    }),
                })
                .await?
        }

        Message::Notification { method, params }
            if *publish_diagnostics_supported && (method == "textDocument/didOpen") =>
        {
            if let (Some(uri), Some(text)) =
                (text_document_uri(&params), text_document_text(&params))
            {
                lex_and_report_errs(responder, uri, text).await?
            } else {
                warn!("Skipped: ({:?}) {:?}", method, params)
            }
        }

        Message::Notification { method, params }
            if *publish_diagnostics_supported && (method == "textDocument/didChange") =>
        {
            if let (Some(uri), Some(text)) = (text_document_uri(&params), changed_text(&params)) {
                lex_and_report_errs(responder, uri, text).await?
            } else {
                warn!("Skipped: ({:?}) {:?}", method, params)
            }
        }

        Message::Notification { method, params }
            if *publish_diagnostics_supported && method == "textDocument/didClose" =>
        {
            if let Some(uri) = text_document_uri(&params) {
                responder
                    .send(Message::Notification {
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
