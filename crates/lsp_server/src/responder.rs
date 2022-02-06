use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use actix::{Actor, Addr, Context, ContextFutureSpawner, Handler, WrapFuture};
use ast::range::Locatable;
use lexer::{result::LexErr, with_pos::WithPosExt, LexExt};
use log::warn;
use lsp::LspMessage;
use serde_json::{json, Value as JsonValue};
use token::Token;
use valq::query_value;

use crate::{received_msg::LspReceiveMsg, send_msg::LspSendMsg, sender::LspSendActor};

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
    sender: Addr<LspSendActor>,
    uri: &JsonValue,
    text: &str,
) -> anyhow::Result<()> {
    let (_tokens, errors) = lex(text);

    let diagnostics: Vec<_> = errors.iter().map(lex_err_to_diagnostic).collect();

    sender
        .send(LspSendMsg(LspMessage::Notification {
            method: "textDocument/publishDiagnostics".to_owned(),
            params: json!({
                "uri": uri,
                "diagnostics": diagnostics,
            }),
        }))
        .await?;

    Ok(())
}

pub struct Responder {
    pub sender: Addr<LspSendActor>,
    diagnostics_supported: Arc<AtomicBool>,
}

impl Responder {
    pub fn new(sender: Addr<LspSendActor>) -> Self {
        Self {
            sender,
            diagnostics_supported: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Actor for Responder {
    type Context = Context<Self>;
}

impl Handler<LspReceiveMsg> for Responder {
    type Result = ();

    fn handle(&mut self, msg: LspReceiveMsg, ctx: &mut Self::Context) {
        let sender = self.sender.clone();

        let diagnostics_supported = Arc::clone(&self.diagnostics_supported);

        async move {
            let diagnostics_currently_enabled = diagnostics_supported.load(Ordering::Relaxed);

            match msg.0 {
                LspMessage::Request { id, method, params } if method == "initialize" => {
                    diagnostics_supported.store(
                        query_value!(params.capabilities.textDocument.publishDiagnostics).is_some(),
                        Ordering::Relaxed,
                    );

                    sender
                        .send(LspSendMsg(LspMessage::Response {
                            id,
                            result: json!({
                                "capabilities": {
                                    "textDocumentSync": 1,
                                }
                            }),
                        }))
                        .await
                        .unwrap()
                }

                LspMessage::Notification { method, params }
                    if diagnostics_currently_enabled && (method == "textDocument/didOpen") =>
                {
                    if let (Some(uri), Some(text)) = (
                        text_document_uri(&params),
                        query_value!(params.textDocument.text -> str),
                    ) {
                        lex_and_report_errs(sender, uri, text).await.unwrap()
                    } else {
                        warn!("Skipped: ({:?}) {:?}", method, params)
                    }
                }

                LspMessage::Notification { method, params }
                    if diagnostics_currently_enabled && (method == "textDocument/didChange") =>
                {
                    if let (Some(uri), Some(text)) = (
                        text_document_uri(&params),
                        query_value!(params.contentChanges[0].text -> str),
                    ) {
                        lex_and_report_errs(sender, uri, text).await.unwrap()
                    } else {
                        warn!("Skipped: ({:?}) {:?}", method, params)
                    }
                }

                LspMessage::Notification { method, params }
                    if diagnostics_currently_enabled && method == "textDocument/didClose" =>
                {
                    if let Some(uri) = text_document_uri(&params) {
                        sender
                            .send(LspSendMsg(LspMessage::Notification {
                                method: "textDocument/publishDiagnostics".to_owned(),
                                params: json!({ "uri": uri, "diagnostics": [] }),
                            }))
                            .await
                            .unwrap()
                    }
                }
                _ => (),
            }
        }
        .into_actor(self)
        .wait(ctx);
    }
}
