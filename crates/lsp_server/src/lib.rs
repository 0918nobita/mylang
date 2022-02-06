mod received_msg;
mod receiver;
mod responder;
mod send_msg;
mod sender;

use actix::Actor;
use lsp::LspMessage;
use serde_json::json;

use crate::{receiver::Receiver, responder::Responder, send_msg::SendMsg, sender::Sender};

pub async fn launch_lsp_server() -> anyhow::Result<()> {
    let sender = Sender.start();

    let responder = Responder::new(sender.clone()).start();

    Receiver { responder }.start();

    sender
        .send(SendMsg(LspMessage::Notification {
            method: "window/showMessage".to_owned(),
            params: json!({
                "type": "info",
                "message": "Hello, world!",
            }),
        }))
        .await?;

    Ok(())
}
