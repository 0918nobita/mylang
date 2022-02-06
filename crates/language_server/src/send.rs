use log::info;
use tokio::{
    io::{AsyncWrite, AsyncWriteExt},
    sync::mpsc,
};

use crate::message::Message;

pub async fn send_msgs<W>(
    writer: &mut W,
    rpc_send_rx: &mut mpsc::Receiver<Message>,
) -> anyhow::Result<()>
where
    W: AsyncWrite + Unpin,
{
    while let Some(msg) = rpc_send_rx.recv().await {
        info!("--> {:?}", msg);
        writer.write_all(msg.raw_message().as_bytes()).await?;
        writer.flush().await?;
    }

    Ok(())
}
