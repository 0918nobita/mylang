use actix::Message;
use lsp::LspMessage;

pub struct SendMsg(pub LspMessage);

impl Message for SendMsg {
    type Result = ();
}
