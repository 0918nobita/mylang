use log::info;
use tokio::{
    io::{self, BufReader, BufWriter},
    sync::watch,
};

use language_server::{message::Message, receive_msgs, send_notification, TaskMsg};

#[tokio::main]
async fn main() {
    env_logger::init();

    let (tx, mut rx) = watch::channel(TaskMsg::Initial);

    tokio::spawn(async move {
        let mut stdin = BufReader::new(io::stdin());
        receive_msgs(&mut stdin, &tx).await
    });

    tokio::spawn(async {
        let mut stdout = BufWriter::new(io::stdout());
        send_notification(&mut stdout).await
    });

    while rx.changed().await.is_ok() {
        match &*rx.borrow() {
            TaskMsg::Initial => (),
            TaskMsg::Received(rpc_msg) => {
                info!("<-- {:?}", rpc_msg);
                match rpc_msg {
                    Message::Request { method, .. } if method == "initialize" => {
                        info!("Initialize request received");
                    }
                    _ => (),
                }
            }
        }
    }
}
