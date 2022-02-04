use std::io::{self, BufRead, Read};

use anyhow::Context;
use regex::Regex;
use serde::{Deserialize, Serialize};

/// JSON-RPC メッセージ
#[derive(Debug, Serialize, Deserialize)]
struct Message {
    #[serde(rename = "jsonrpc")]
    version: String,

    id: usize,

    method: String,
}

fn main() -> anyhow::Result<()> {
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
