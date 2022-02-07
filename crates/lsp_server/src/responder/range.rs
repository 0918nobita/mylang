use serde_json::{json, Value as JsonValue};
use token::Locatable;

pub fn locatable_to_lsp_range<L>(locatable: &L) -> JsonValue
where
    L: Locatable,
{
    let range = locatable.locate();
    let start = range.start;
    let end = range.end;
    json!({
        "start": { "line": start.line, "character": start.character },
        "end": { "line": end.line, "character": end.character },
    })
}
