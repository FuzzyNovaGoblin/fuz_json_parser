use crate::values::JsonValue;

mod error;
mod parser;

pub fn parse<S: AsRef<str>>(json_str: S) -> error::Result<JsonValue> {
    parser::run_parse(json_str)
}
