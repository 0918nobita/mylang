use mylang_token::{Pos, Token};

use crate::state::symbol::SymbolState;

pub enum SymbolLexResult {
    Continued(SymbolState),
    Interrupted(Token),
}

pub fn symbol_lex(symbol_state: &SymbolState, (pos, c): (Pos, char)) -> SymbolLexResult {
    match c {
        c if c.is_ascii() && !c.is_ascii_whitespace() => {
            SymbolLexResult::Continued(symbol_state.append_char(pos, c))
        }

        _ => SymbolLexResult::Interrupted(symbol_state.tokenize()),
    }
}
