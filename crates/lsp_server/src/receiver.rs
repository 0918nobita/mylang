use actix::{Actor, Addr, Context, ContextFutureSpawner, WrapFuture};
use log::{info, warn};
use regex::Regex;
use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, BufReader};

use crate::{message::LspMessage, responder::Responder};

pub struct Receiver {
    pub responder: Addr<Responder>,
}

impl Actor for Receiver {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let responder = self.responder.clone();

        async move {
            let stdin = BufReader::new(io::stdin());
            let mut lines = stdin.lines();
            let re = Regex::new(r"Content-Length: (\d+)").unwrap();
            while let Ok(Some(line)) = lines.next_line().await {
                if let Some(caps) = re.captures(&line) {
                    let len = 2 + caps[1].parse::<usize>().unwrap();

                    let mut msg_buf = vec![0u8; len];
                    let reader = lines.get_mut();
                    reader.read_exact(&mut msg_buf).await.unwrap();
                    reader.consume(len);

                    let msg = String::from_utf8(msg_buf).unwrap();
                    let msg: LspMessage = serde_json::from_str(&msg).unwrap();
                    info!("<-- {:?}", msg);

                    responder.send(msg).await.unwrap();
                } else {
                    warn!("Skiped: {}", line);
                }
            }
        }
        .into_actor(self)
        .wait(ctx);
    }
}
