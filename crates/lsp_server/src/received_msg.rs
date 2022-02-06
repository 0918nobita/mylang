use actix::Message;
use lsp::LspMessage;

pub struct ReceivedMsg(pub LspMessage);

impl Message for ReceivedMsg {
    type Result = ();
}
