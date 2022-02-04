use serde::{Deserialize, Serialize};

use crate::message::Message;

/// JSON-RPC リクエスト
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    id: usize,
    method: String,
    params: serde_json::Value,
}

impl Message for Request {}
