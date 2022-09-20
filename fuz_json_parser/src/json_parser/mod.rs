use crate::{error, values::JsonValue};

pub mod parsers;
pub mod state;

pub fn parse<S: AsRef<str>>(json_str: S) -> error::Result<JsonValue> {
    let mut state = state::ParserState::new(json_str.as_ref().chars());
    state.consume_whitespace();
    match state.peek() {
        Some(_) => parsers::main_parse(&mut state),
        None => Ok(JsonValue::Null),
    }
}
