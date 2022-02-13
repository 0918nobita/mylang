mod analyzer;
mod diagnostic;
mod handler;
mod range;

use std::sync::{atomic::AtomicBool, Arc};

use actix::{Actor, Addr, Context, ContextFutureSpawner, Handler, WrapFuture};

use self::handler::handle;
use crate::{message::LspMessage, sender::Sender};

/// 言語クライアントから受信したメッセージをもとに応答を決定し、[`Sender`] に送信を依頼するアクター
pub struct Responder {
    sender: Addr<Sender>,
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

impl Handler<LspMessage> for Responder {
    type Result = ();

    fn handle(&mut self, msg: LspMessage, ctx: &mut Self::Context) {
        let sender = self.sender.clone();

        let diagnostics_supported = Arc::clone(&self.diagnostics_supported);

        async move { handle(sender, diagnostics_supported, msg).await }
            .into_actor(self)
            .wait(ctx);
    }
}
