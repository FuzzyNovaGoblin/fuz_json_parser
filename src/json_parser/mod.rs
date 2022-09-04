use crate::{create_state, error, values::JsonValue};

pub mod parsers;
pub mod state;

pub fn parse<S: AsRef<str>>(json_str: S) -> error::Result<JsonValue> {
    let mut state = create_state!(json_str);
    state::consume_whitespace(&mut state);
    match state::peek(&mut state) {
        Some(_) => parsers::main_parse(&mut state),
        None => Ok(JsonValue::Null),
    }
}
