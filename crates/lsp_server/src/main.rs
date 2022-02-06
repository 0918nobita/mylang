use actix::{Actor, Context, ContextFutureSpawner, Handler, Message, System, WrapFuture};
use log::info;
use lsp::LspMessage;
use serde_json::json;
use tokio::io::{self, AsyncWriteExt};

struct LspSendMsg(LspMessage);

impl Message for LspSendMsg {
    type Result = ();
}

struct LspActor;

impl Actor for LspActor {
    type Context = Context<Self>;
}

impl Handler<LspSendMsg> for LspActor {
    type Result = ();

    fn handle(&mut self, msg: LspSendMsg, ctx: &mut Context<Self>) -> Self::Result {
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

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let system = System::new();

    system.block_on(async {
        let addr = LspActor.start();
        addr.send(LspSendMsg(LspMessage::Notification {
            method: "window/showMessage".to_owned(),
            params: json!({
                "type": "info",
                "message": "Hello, world!",
            }),
        }))
        .await
    })?;

    system.run()?;

    Ok(())
}
