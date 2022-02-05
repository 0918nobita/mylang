use serde::{Deserialize, Serialize};

pub trait Message: Serialize {
    fn raw_message(&self) -> String {
        let mut content = serde_json::to_value(self).unwrap();
        let content_map = content.as_object_mut().unwrap();
        content_map.insert(
            "jsonrpc".to_owned(),
            serde_json::Value::String("2.0".to_owned()),
        );

        let content = serde_json::to_string(&content).unwrap();

        format!(
            "Content-Length: {}\r\n\r\n{}",
            content.as_bytes().len(),
            content
        )
    }
}

/// JSON-RPC リクエスト
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    id: usize,
    method: String,
    params: serde_json::Value,
}

impl Message for Request {}

/// JSON-RPC 通知
#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    method: String,
    params: serde_json::Value,
}

impl Notification {
    pub fn new(method: String, params: serde_json::Value) -> Self {
        Self { method, params }
    }
}

impl Message for Notification {}
