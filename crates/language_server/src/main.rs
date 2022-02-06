use std::time::Duration;

use serde_json::json;
use tokio::{
    io::{self, BufReader, BufWriter},
    sync::mpsc,
    time,
};

use language_server::{handle_msg, message::Message, receive::receive_msgs, send::send_msgs};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let (rpc_recv_tx, mut rpc_recv_rx) = mpsc::channel(100);

    tokio::spawn(async move {
        let mut stdin = BufReader::new(io::stdin());
        receive_msgs(&mut stdin, &rpc_recv_tx).await
    });

    let (rpc_send_tx, mut rpc_send_rx) = mpsc::channel(100);

    tokio::spawn(async move {
        let mut stdout = BufWriter::new(io::stdout());
        send_msgs(&mut stdout, &mut rpc_send_rx).await
    });

    let rpc_send_tx_cloned = rpc_send_tx.clone();

    tokio::spawn(async move {
        time::sleep(Duration::from_secs(2)).await;

        rpc_send_tx_cloned
            .send(Message::Notification {
                method: "window/showMessage".to_owned(),
                params: json!({
                    "type": "info",
                    "message": "Hello, world!",
                }),
            })
            .await
    });

    let mut publish_diagnostics_supported = false;

    while let Some(rpc_msg) = rpc_recv_rx.recv().await {
        handle_msg(&rpc_send_tx, &mut publish_diagnostics_supported, rpc_msg).await?;
    }

    Ok(())
}
