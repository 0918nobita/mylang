use anyhow::Context;
use log::{info, warn};
use lsp::LspMessage;
use regex::Regex;
use tokio::{
    io::{AsyncBufRead, AsyncBufReadExt, AsyncReadExt},
    sync::mpsc,
};

pub async fn receive_msgs<R>(
    reader: R,
    rpc_recv_tx: &mpsc::Sender<LspMessage>,
) -> anyhow::Result<()>
where
    R: AsyncBufRead + Unpin,
{
    let re = Regex::new(r"Content-Length: (\d+)")?;

    let mut lines = reader.lines();
    while let Ok(Some(line)) = lines.next_line().await {
        if let Some(caps) = re.captures(&line) {
            let len = 2 + caps[1].parse::<usize>().context(
                "Failed to parse the number of bytes extracted from the received header",
            )?;

            let mut msg_buf = vec![0u8; len];
            let reader = lines.get_mut();
            reader.read_exact(&mut msg_buf).await?;
            reader.consume(len);

            let msg = String::from_utf8(msg_buf)?;
            let msg: LspMessage = serde_json::from_str(&msg)?;
            info!("<-- {:?}", msg);

            rpc_recv_tx.send(msg).await?;
        } else {
            warn!("Skiped: {}", line);
        }
    }

    Ok(())
}
