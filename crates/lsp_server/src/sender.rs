use actix::{Actor, Context, ContextFutureSpawner, Handler, WrapFuture};
use log::info;
use tokio::io::{self, AsyncWriteExt};

use crate::send_msg::SendMsg;

pub struct Sender;

impl Actor for Sender {
    type Context = Context<Self>;
}

impl Handler<SendMsg> for Sender {
    type Result = ();

    fn handle(&mut self, msg: SendMsg, ctx: &mut Context<Self>) -> Self::Result {
        info!("--> {:?}", msg.0);

        async move {
            let mut stdout = io::stdout();
            stdout
                .write_all(msg.0.raw_message().as_bytes())
                .await
                .unwrap();
            stdout.flush().await.unwrap();
        }
        .into_actor(self)
        .wait(ctx);
    }
}
