mod received_msg;
mod receiver;
mod responder;
mod send_msg;
mod sender;

use actix::Actor;
use lsp::LspMessage;
use send_msg::LspSendMsg;
use serde_json::json;

use crate::{receiver::LspReceiveActor, responder::Responder, sender::LspSendActor};

pub async fn launch_lsp_server() -> anyhow::Result<()> {
    let sender = LspSendActor.start();

    let responder = Responder::new(sender.clone()).start();

    LspReceiveActor { responder }.start();

    sender
        .send(LspSendMsg(LspMessage::Notification {
            method: "window/showMessage".to_owned(),
            params: json!({
                "type": "info",
                "message": "Hello, world!",
            }),
        }))
        .await?;

    Ok(())
}
