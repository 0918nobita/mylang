use actix::{Actor, Context, ContextFutureSpawner, Handler, WrapFuture};
use log::info;
use tokio::io::{self, AsyncWriteExt};

use crate::message::LspMessage;

/// 依頼された応答・通知メッセージを言語クライアントに送信するアクター
pub struct Sender;

impl Actor for Sender {
    type Context = Context<Self>;
}

impl Handler<LspMessage> for Sender {
    type Result = ();

    fn handle(&mut self, msg: LspMessage, ctx: &mut Context<Self>) -> Self::Result {
        info!("--> {:?}", msg);

        async move {
            let mut stdout = io::stdout();
            stdout
                .write_all(msg.raw_message().as_bytes())
                .await
                .unwrap();
            stdout.flush().await.unwrap();
        }
        .into_actor(self)
        .wait(ctx);
    }
}
