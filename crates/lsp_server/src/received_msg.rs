use actix::Message;
use lsp::LspMessage;

pub struct LspReceiveMsg(pub LspMessage);

impl Message for LspReceiveMsg {
    type Result = ();
}
