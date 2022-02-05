use log::info;
use tokio::sync::watch;

use language_server::{receive_rpc_msg, send_notification, TaskMsg};

#[tokio::main]
async fn main() {
    env_logger::init();

    let (tx, mut rx) = watch::channel(TaskMsg::Initial);

    tokio::spawn(async move { receive_rpc_msg(&tx).await });

    tokio::spawn(async { send_notification().await });

    while rx.changed().await.is_ok() {
        info!("Received: {:?}", *rx.borrow());
    }
}
