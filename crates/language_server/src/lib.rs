mod message;
mod notification;
mod request;

use std::{
    io::{self, BufRead, Read},
    time::Duration,
};

use anyhow::Context;
use regex::Regex;
use serde_json::json;

use crate::{message::Message, notification::Notification, request::Request};

pub async fn wait_for_initialize_request() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut buf = String::new();

    let re = Regex::new("Content-Length: (\\d+)")?;

    loop {
        let num_bytes = stdin.read_line(&mut buf)?;
        buf = buf.trim().to_owned();
        stdin.consume(num_bytes);

        if let Some(caps) = re.captures(&buf) {
            let len = caps
                .get(1)
                .context("Failed to extract the number of bytes from the received header")?
                .as_str()
                .parse::<usize>()
                .context(
                    "Failed to parse the number of bytes extracted from the received header",
                )?;
            eprintln!("Length: {}", len);

            let mut content_buf = vec![0u8; len];
            stdin.read_exact(&mut content_buf)?;
            stdin.consume(len);

            let content = String::from_utf8(content_buf)?;
            eprintln!("Content: {}", content);

            let req: Request = serde_json::from_str(&content)?;
            eprintln!("Deserialized: {:?}", req);
        }
    }
}

pub async fn send_notification() {
    tokio::time::sleep(Duration::from_secs(2)).await;

    let notification = Notification::new(
        "window/logMessage".to_owned(),
        json!({
            "type": 3,
            "message": "Hello from mylang LSP server!",
        }),
    );
    println!("{}", notification.raw_message());
}
