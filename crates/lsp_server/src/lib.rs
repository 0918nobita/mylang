//! mylang を記述するための、LSP に準拠した言語サーバの実装

mod message;
mod receiver;
mod responder;
mod sender;

use actix::Actor;

use crate::{receiver::Receiver, responder::Responder, sender::Sender};

/// 言語サーバに関連する各アクターを起動する
pub fn launch_lsp_server() {
    let sender = Sender.start();

    let responder = Responder::new(sender).start();

    Receiver::new(responder).start();
}
