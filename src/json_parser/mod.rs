use std::{iter::{self, Enumerate, Peekable}, str::Chars};

use crate::{error, values::JsonValue};

pub mod parser;
// pub mod state_manip;
// pub mod checkers;

/// state of parseing function
/// holds cursor and string to be parsed
type ParserState<'a> = Peekable<Enumerate<Chars<'a>>>;

/// error description to use whenever unexpectedly reaching the end of the source string
const UNEXPECTED_END_OF_STRING: &str = "Invalid JSON\t unexpected end of string";

// pub fn parse<S: AsRef<str>>(json_str: S) -> error::Result<JsonValue> {
//     let mut state = json_str.as_ref().chars().enumerate().peekable();

//     consume_whitespace(&mut state);
//     match peek(&mut state) {
//         Some(_) => main_parse(&mut state),
//         None => Ok(JsonValue::Null),
//     }
// }

pub fn parse<S: AsRef<str>>(json_str: S) -> error::Result<JsonValue> {
    parser::run_parse(json_str)
}
