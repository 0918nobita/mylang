use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value as JsonValue};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LspMessage {
    Request {
        id: usize,
        method: String,
        params: JsonValue,
    },
    Response {
        id: usize,
        result: JsonValue,
    },
    Notification {
        method: String,
        params: JsonValue,
    },
}

impl LspMessage {
    pub fn raw_message(&self) -> String {
        let mut content_map = Map::new();

        match self {
            LspMessage::Request { id, method, params } => {
                content_map.insert("id".to_owned(), json!(id));
                content_map.insert("method".to_owned(), json!(method));
                content_map.insert("params".to_owned(), params.clone());
            }

            LspMessage::Response { id, result } => {
                content_map.insert("id".to_owned(), json!(id));
                content_map.insert("result".to_owned(), result.clone());
            }

            LspMessage::Notification { method, params } => {
                content_map.insert("method".to_owned(), json!(method));
                content_map.insert("params".to_owned(), params.clone());
            }
        }

        content_map.insert("jsonrpc".to_owned(), json!("2.0"));

        let content = serde_json::to_string(&JsonValue::Object(content_map)).unwrap();

        format!(
            "Content-Length: {}\r\n\r\n{}",
            content.as_bytes().len(),
            content
        )
    }
}
