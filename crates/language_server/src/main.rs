use log::info;
use tokio::{
    io::{self, BufReader},
    sync::watch,
};

use language_server::{receive_msgs, send_notification, TaskMsg};

#[tokio::main]
async fn main() {
    env_logger::init();

    let (tx, mut rx) = watch::channel(TaskMsg::Initial);

    let mut stdin = BufReader::new(io::stdin());
    tokio::spawn(async move { receive_msgs(&mut stdin, &tx).await });

    tokio::spawn(async { send_notification().await });

    while rx.changed().await.is_ok() {
        info!("Received: {:?}", *rx.borrow());
    }
}
