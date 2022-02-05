mod message;

use std::{
    io::{self, BufRead, Read},
    time::Duration,
};

use anyhow::Context;
use log::{info, warn};
use regex::Regex;
use serde_json::json;
use tokio::sync::watch::Sender;

use message::Message;

#[derive(Debug)]
pub enum TaskMsg {
    Initial,
    Received(Message),
}

pub async fn receive_msgs(task_msg_sender: &Sender<TaskMsg>) -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut buf = String::new();

    let re = Regex::new(r"Content-Length: (\d+)")?;

    loop {
        let num_bytes = stdin.read_line(&mut buf)?;
        buf = buf.trim().to_owned();
        stdin.consume(num_bytes);

        if let Some(caps) = re.captures(&buf) {
            let len = caps[1].parse::<usize>().context(
                "Failed to parse the number of bytes extracted from the received header",
            )?;
            info!("Length: {}", len);

            let mut msg_buf = vec![0u8; len];
            stdin.read_exact(&mut msg_buf)?;
            stdin.consume(len);

            let msg = String::from_utf8(msg_buf)?;
            let msg: Message = serde_json::from_str(&msg)?;

            task_msg_sender.send(TaskMsg::Received(msg))?;
        } else {
            warn!("Skiped: {}", buf);
        }
    }
}

pub async fn send_notification() {
    tokio::time::sleep(Duration::from_secs(2)).await;

    let notification = Message::Notification {
        method: "window/logMessage".to_owned(),
        params: json!({
            "type": 3,
            "message": "Hello from mylang LSP server!",
        }),
    };
    println!("{}", notification.raw_message());
}
