use crate::{create_state, error, values::JsonValue};

pub mod parsers;
pub mod state;
pub mod wrapped_parsers;
pub mod wrapped_state;

pub fn parse<S: AsRef<str>>(json_str: S) -> error::Result<JsonValue> {
    let mut state = create_state!(json_str);
    state::consume_whitespace(&mut state);
    match state::peek(&mut state) {
        Some(_) => parsers::main_parse(&mut state),
        None => Ok(JsonValue::Null),
    }
}
pub fn wrapped_parse<S: AsRef<str>>(json_str: S) -> error::Result<JsonValue> {
    let mut state = wrapped_state::ParserState::new(json_str.as_ref().chars());
    state.consume_whitespace();
    match state.peek() {
        Some(_) => wrapped_parsers::main_parse(&mut state),
        None => Ok(JsonValue::Null),
    }
}
