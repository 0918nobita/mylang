use mylang_token::Locatable;
use serde_json::Value as JsonValue;

pub fn locatable_to_lsp_range<L>(locatable: &L) -> JsonValue
where
    L: Locatable,
{
    let range = locatable.locate();
    serde_json::to_value(range).expect("Failed to convert locatable to json")
}
