use actix::Message;
use lsp::LspMessage;

pub struct LspSendMsg(pub LspMessage);

impl Message for LspSendMsg {
    type Result = ();
}
