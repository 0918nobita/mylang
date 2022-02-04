use serde::{Deserialize, Serialize};

use crate::message::Message;

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
