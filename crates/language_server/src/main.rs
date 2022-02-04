use std::{
    io::{self, BufRead, Read},
    time::Duration,
};

use anyhow::Context;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// JSON-RPC メッセージ
#[derive(Debug, Serialize, Deserialize)]
struct Message {
    #[serde(rename = "jsonrpc")]
    version: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<usize>,

    method: String,
    params: serde_json::Value,
}

async fn wait_for_initialize_msg() -> anyhow::Result<()> {
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

            let msg: Message = serde_json::from_str(&content)?;
            eprintln!("Deserialized: {:?}", msg);
        }
    }
}

async fn send_message() -> anyhow::Result<()> {
    tokio::time::sleep(Duration::from_secs(2)).await;

    let msg = Message {
        version: "2.0".to_owned(),
        id: None,
        method: "window/logMessage".to_owned(),
        params: json!({
            "type": 3,
            "message": "Hello from mylang LSP server!",
        }),
    };

    let content = serde_json::to_string(&msg)?;
    let len = content.as_bytes().len();
    println!("Content-Length: {}\r\n\r\n{}", len, content);

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let handle = tokio::spawn(async { wait_for_initialize_msg().await });
    send_message().await?;
    handle.await?
}
