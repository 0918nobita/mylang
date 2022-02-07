use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use actix::{Actor, Addr, Context, ContextFutureSpawner, Handler, WrapFuture};
use lexer::{result::LexErr, with_pos::WithPosExt, LexExt};
use log::warn;
use lsp::LspMessage;
use parser::ParseErr;
use serde_json::{json, Value as JsonValue};
use token::Locatable;
use token::Token;
use valq::query_value;

use crate::{received_msg::ReceivedMsg, send_msg::SendMsg, sender::Sender};

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
        "start": { "line": start.line, "character": start.character },
        "end": { "line": end.line, "character": end.character },
    })
}

fn lex_err_to_diagnostic(err: &LexErr) -> JsonValue {
    let range = locatable_to_json_range(err);
    let message = err.to_string();
    json!({ "range": range, "message": message })
}

fn parse_err_to_diagnostic(err: ParseErr) -> JsonValue {
    let range = locatable_to_json_range(&err);
    let message = err.to_string();
    json!({ "range": range, "message": message })
}

async fn analyze_src_and_report_errs(
    sender: Addr<Sender>,
    uri: &JsonValue,
    text: &str,
) -> anyhow::Result<()> {
    let (tokens, errors) = lex(text);

    let mut diagnostics: Vec<_> = errors.iter().map(lex_err_to_diagnostic).collect();

    let (_stmts, errors): (Vec<_>, Vec<_>) = parser::parse(tokens.into_iter())
        .into_iter()
        .partition(Result::is_ok);

    let diagnostics_from_parser = errors
        .into_iter()
        .map(Result::unwrap_err)
        .map(parse_err_to_diagnostic)
        .collect::<Vec<_>>();

    diagnostics.extend(diagnostics_from_parser);

    sender
        .send(SendMsg(LspMessage::Notification {
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
    pub sender: Addr<Sender>,
    diagnostics_supported: Arc<AtomicBool>,
}

impl Responder {
    pub fn new(sender: Addr<Sender>) -> Self {
        Self {
            sender,
            diagnostics_supported: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Actor for Responder {
    type Context = Context<Self>;
}

impl Handler<ReceivedMsg> for Responder {
    type Result = ();

    fn handle(&mut self, msg: ReceivedMsg, ctx: &mut Self::Context) {
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
                        .send(SendMsg(LspMessage::Response {
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
                        analyze_src_and_report_errs(sender, uri, text)
                            .await
                            .unwrap()
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
                        analyze_src_and_report_errs(sender, uri, text)
                            .await
                            .unwrap()
                    } else {
                        warn!("Skipped: ({:?}) {:?}", method, params)
                    }
                }

                LspMessage::Notification { method, params }
                    if diagnostics_currently_enabled && method == "textDocument/didClose" =>
                {
                    if let Some(uri) = text_document_uri(&params) {
                        sender
                            .send(SendMsg(LspMessage::Notification {
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
