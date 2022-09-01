use crate::values::JsonValue;

mod error;
mod parser;

pub fn parse<S: AsRef<str>>(json_str: S) -> error::Result<JsonValue> {
    parser::ParserState::run_parse(json_str)
}
