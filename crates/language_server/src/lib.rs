pub mod message;
pub mod receive;
pub mod send;

use serde_json::{json, Value as JsonValue};
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
            if *publish_diagnostics_supported && method == "textDocument/didOpen" =>
        {
            if let Some(uri) = text_document_uri(&params) {
                responder
                    .send(Message::Notification {
                        method: "textDocument/publishDiagnostics".to_owned(),
                        params: json!({
                            "uri": uri,
                            "diagnostics": [
                                {
                                    "range": {
                                        "start": { "line": 0, "character": 0 },
                                        "end": { "line": 0, "character": 1 },
                                    },
                                    "message": "Diagnostic message",
                                },
                            ]
                        }),
                    })
                    .await?;
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
                    .await?;
            }
        }

        _ => (),
    }

    Ok(())
}
