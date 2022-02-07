mod message;
mod receiver;
mod responder;
mod sender;

use actix::Actor;
use serde_json::json;

use crate::{message::LspMessage, receiver::Receiver, responder::Responder, sender::Sender};

pub async fn launch_lsp_server() -> anyhow::Result<()> {
    let sender = Sender.start();

    let responder = Responder::new(sender.clone()).start();

    Receiver { responder }.start();

    sender
        .send(LspMessage::Notification {
            method: "window/showMessage".to_owned(),
            params: json!({
                "type": "info",
                "message": "Hello, world!",
            }),
        })
        .await?;

    Ok(())
}
