use anyhow::Context;
use log::{info, warn};
use regex::Regex;
use tokio::{
    io::{AsyncBufRead, AsyncBufReadExt, AsyncReadExt},
    sync::watch,
};

use crate::message::Message;

#[derive(Debug)]
pub enum RpcRecvMsg {
    Initial,
    Received(Message),
}

pub async fn receive_msgs<R>(
    reader: &mut R,
    rpc_recv_tx: &watch::Sender<RpcRecvMsg>,
) -> anyhow::Result<()>
where
    R: AsyncBufRead + Unpin,
{
    let mut buf = String::new();

    let re = Regex::new(r"Content-Length: (\d+)")?;

    loop {
        let num_bytes = reader.read_line(&mut buf).await?;
        buf = buf.trim().to_owned();
        reader.consume(num_bytes);

        if let Some(caps) = re.captures(&buf) {
            let len = caps[1].parse::<usize>().context(
                "Failed to parse the number of bytes extracted from the received header",
            )?;
            info!("Length: {}", len);

            let mut msg_buf = vec![0u8; len];
            reader.read_exact(&mut msg_buf).await?;
            reader.consume(len);

            let msg = String::from_utf8(msg_buf)?;
            let msg: Message = serde_json::from_str(&msg)?;

            rpc_recv_tx.send(RpcRecvMsg::Received(msg))?;
        } else {
            warn!("Skiped: {}", buf);
        }
    }
}
