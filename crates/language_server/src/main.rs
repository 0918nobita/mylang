use std::time::Duration;

use log::info;
use serde_json::{json, Value as JsonValue};
use tokio::{
    io::{self, BufReader, BufWriter},
    sync::mpsc,
    time,
};

use language_server::{message::Message, receive::receive_msgs, send::send_msgs};

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let (rpc_recv_tx, mut rpc_recv_rx) = mpsc::channel(100);

    tokio::spawn(async move {
        let mut stdin = BufReader::new(io::stdin());
        receive_msgs(&mut stdin, &rpc_recv_tx).await
    });

    let (rpc_send_tx, mut rpc_send_rx) = mpsc::channel(100);

    tokio::spawn(async move {
        let mut stdout = BufWriter::new(io::stdout());
        send_msgs(&mut stdout, &mut rpc_send_rx).await
    });

    let rpc_send_tx_cloned = rpc_send_tx.clone();
    tokio::spawn(async move {
        time::sleep(Duration::from_secs(2)).await;

        rpc_send_tx_cloned
            .send(Message::Notification {
                method: "window/showMessage".to_owned(),
                params: json!({
                    "type": "info",
                    "message": "Hello, world!",
                }),
            })
            .await
    });

    let mut publish_diagnostics_supported = false;

    while let Some(rpc_msg) = rpc_recv_rx.recv().await {
        info!("<-- {:?}", rpc_msg);

        match rpc_msg {
            Message::Request { id, method, params } if method == "initialize" => {
                publish_diagnostics_supported = publish_diagnostics(&params).is_some();

                rpc_send_tx
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
                if publish_diagnostics_supported && method == "textDocument/didOpen" =>
            {
                if let Some(uri) = text_document_uri(&params) {
                    rpc_send_tx
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
                if publish_diagnostics_supported && method == "textDocument/didClose" =>
            {
                if let Some(uri) = text_document_uri(&params) {
                    rpc_send_tx
                        .send(Message::Notification {
                            method: "textDocument/publishDiagnostics".to_owned(),
                            params: json!({ "uri": uri, "diagnostics": [] }),
                        })
                        .await?;
                }
            }

            _ => (),
        }
    }

    Ok(())
}
