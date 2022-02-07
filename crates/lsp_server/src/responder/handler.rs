use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use actix::Addr;
use log::{info, warn};
use serde_json::{json, Value as JsonValue};
use valq::query_value;

use super::analyzer::analyze_src;
use crate::{message::LspMessage, sender::Sender};

fn token_types(params_obj: &JsonValue) -> Option<HashMap<String, usize>> {
    let token_types = query_value!(
        params_obj.capabilities.textDocument.semanticTokens.tokenTypes -> array
    )?;

    let token_types = token_types
        .iter()
        .map(|token_type| token_type.as_str())
        .collect::<Option<Vec<_>>>()?;

    let mut token_type_map = HashMap::new();
    for (i, token_type) in token_types.iter().enumerate() {
        token_type_map.insert(token_type.to_string(), i);
    }

    Some(token_type_map)
}

async fn handle_initialize_request(
    sender: Addr<Sender>,
    diagnostics_supported: Arc<AtomicBool>,
    id: usize,
    params: JsonValue,
) {
    diagnostics_supported.store(
        query_value!(params.capabilities.textDocument.publishDiagnostics).is_some(),
        Ordering::Relaxed,
    );

    info!("TokenTypes: {:?}", token_types(&params));

    sender
        .send(LspMessage::Response {
            id,
            result: json!({
                "capabilities": {
                    "textDocumentSync": 1,
                }
            }),
        })
        .await
        .unwrap()
}

fn text_document_uri(params_obj: &JsonValue) -> Option<&JsonValue> {
    query_value!(params_obj.textDocument.uri)
}

async fn handle_did_open_notification(sender: Addr<Sender>, params: JsonValue) {
    if let (Some(uri), Some(text)) = (
        text_document_uri(&params),
        query_value!(params.textDocument.text -> str),
    ) {
        analyze_src(sender, uri, text).await.unwrap()
    } else {
        warn!("Skipped: (textDocument/didOpen) {:?}", params)
    }
}

async fn handle_did_change_notification(sender: Addr<Sender>, params: JsonValue) {
    if let (Some(uri), Some(text)) = (
        text_document_uri(&params),
        query_value!(params.contentChanges[0].text -> str),
    ) {
        analyze_src(sender, uri, text).await.unwrap()
    } else {
        warn!("Skipped: (textDocument/didChange) {:?}", params)
    }
}

async fn handle_did_close_notification(sender: Addr<Sender>, params: JsonValue) {
    if let Some(uri) = text_document_uri(&params) {
        sender
            .send(LspMessage::Notification {
                method: "textDocument/publishDiagnostics".to_owned(),
                params: json!({ "uri": uri, "diagnostics": [] }),
            })
            .await
            .unwrap()
    } else {
        warn!("Skipped: (textDocument/didClose) {:?}", params);
    }
}

pub async fn handle(sender: Addr<Sender>, diagnostics_supported: Arc<AtomicBool>, msg: LspMessage) {
    let diagnostics_currently_enabled = diagnostics_supported.load(Ordering::Relaxed);

    match msg {
        LspMessage::Request { id, method, params } if method == "initialize" => {
            handle_initialize_request(sender, diagnostics_supported, id, params).await
        }

        LspMessage::Notification { method, params }
            if diagnostics_currently_enabled && (method == "textDocument/didOpen") =>
        {
            handle_did_open_notification(sender, params).await
        }

        LspMessage::Notification { method, params }
            if diagnostics_currently_enabled && (method == "textDocument/didChange") =>
        {
            handle_did_change_notification(sender, params).await
        }

        LspMessage::Notification { method, params }
            if diagnostics_currently_enabled && method == "textDocument/didClose" =>
        {
            handle_did_close_notification(sender, params).await
        }

        _ => (),
    }
}
