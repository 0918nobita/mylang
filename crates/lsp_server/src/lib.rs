mod message;
mod receiver;
mod responder;
mod sender;

use actix::Actor;

use crate::{receiver::Receiver, responder::Responder, sender::Sender};

pub async fn launch_lsp_server() {
    let sender = Sender.start();

    let responder = Responder::new(sender).start();

    Receiver::new(responder).start();
}
