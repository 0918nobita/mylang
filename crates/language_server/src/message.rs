use serde::{Deserialize, Serialize};
use serde_json::{Map, Value as JsonValue};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Message {
    Request {
        id: String,
        method: String,
        params: JsonValue,
    },
    Response {
        id: String,
        result: JsonValue,
    },
    Notification {
        method: String,
        params: JsonValue,
    },
}

impl Message {
    pub fn raw_message(&self) -> String {
        let mut content_map = Map::new();

        match self {
            Message::Request { id, method, params } => {
                content_map.insert("id".to_owned(), JsonValue::String(id.clone()));
                content_map.insert("method".to_owned(), JsonValue::String(method.clone()));
                content_map.insert("params".to_owned(), params.clone());
            }
            Message::Response { id, result } => {
                content_map.insert("id".to_owned(), JsonValue::String(id.clone()));
                content_map.insert("result".to_owned(), result.clone());
            }
            Message::Notification { method, params } => {
                content_map.insert("method".to_owned(), JsonValue::String(method.clone()));
                content_map.insert("params".to_owned(), params.clone());
            }
        }

        content_map.insert(
            "jsonrpc".to_owned(),
            serde_json::Value::String("2.0".to_owned()),
        );

        let content = serde_json::to_string(&JsonValue::Object(content_map)).unwrap();

        format!(
            "Content-Length: {}\r\n\r\n{}",
            content.as_bytes().len(),
            content
        )
    }
}
