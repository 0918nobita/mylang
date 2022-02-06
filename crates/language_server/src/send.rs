use log::info;
use tokio::{
    io::{AsyncWrite, AsyncWriteExt},
    sync::mpsc,
};

use crate::message::Message;

#[derive(Debug)]
pub enum RpcSendMsg {
    Initial,
    Send(Message),
}

pub async fn send_msgs<W>(
    writer: &mut W,
    rpc_send_rx: &mut mpsc::Receiver<RpcSendMsg>,
) -> anyhow::Result<()>
where
    W: AsyncWrite + Unpin,
{
    while let Some(msg) = rpc_send_rx.recv().await {
        match msg {
            RpcSendMsg::Initial => (),

            RpcSendMsg::Send(msg) => {
                info!("--> {:?}", msg);
                writer.write_all(msg.raw_message().as_bytes()).await?;
                writer.flush().await?;
            }
        }
    }

    Ok(())
}
