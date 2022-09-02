use std::{
    collections::HashMap,
    iter::{Enumerate, Peekable},
    str::Chars,
};

use super::error::Result;
use crate::values::{JsonNum, JsonValue};

const UNEXPECTED_END_OF_STRING: &str = "Invalid JSON\t unexpected end of string";

/// state of parseing function
/// holds cursor and string to be parsed
type ParserState<'a> = Peekable<Enumerate<Chars<'a>>>;

/// create the state of the parser and initiate the parser on that state
pub fn run_parse<S: AsRef<str>>(json_str: S) -> Result<JsonValue> {
    let mut state = json_str.as_ref().chars().enumerate().peekable();

    consume_whitespace(&mut state);
    main_parse(&mut state)
}

/// look at the current charater
fn peek(state: &mut ParserState) -> Option<char> {
    state.peek().map(|(_, c)| *c)
}

/// return the current character and cursor to the next position
fn advance(state: &mut ParserState) -> Option<char> {
    state.next().map(|(_, c)| c)
}

/// assert that the current character is the expected character `c`
///
/// if `ignore_case` is `true` the check will be preformed without considering the case of the character
fn assert_char(state: &mut ParserState, mut c: char, ignore_case: bool) -> Result<()> {
    if ignore_case {
        c = c.to_ascii_lowercase();
    }

    if let Some(mut advance_value) = advance(state) {
        if ignore_case {
            advance_value = advance_value.to_ascii_lowercase();
            advance_value = advance_value.to_ascii_lowercase();
        }
        if advance_value != c {
            Err(format!("`{advance_value}` is not equal to `{c}`").into())
        } else {
            Ok(())
        }
    } else {
        Err("No char returned".into())
    }
}

/// uses [assert_char](ParserState::assert_char) to assert that the next characters are equal to the provided string
fn assert_string<S: AsRef<str>>(
    state: &mut ParserState,
    string: S,
    ignore_case: bool,
) -> Result<()> {
    for c in string.as_ref().chars() {
        if let Err(e) = assert_char(state, c, ignore_case) {
            return Err(format!("failed assert expected \"{}\"\n{e}", string.as_ref()).into());
        }
    }
    Ok(())
}

/// check if the character at the cursor is a digit
fn is_digit(state: &mut ParserState) -> bool {
    if let Some(c) = peek(state) {
        if c >= (48 as char) && c <= (57 as char) {
            return true;
        }
    }
    false
}

/// check if the character at the cursor is white space
fn is_whitespace(state: &mut ParserState) -> bool {
    matches!(peek(state), Some(' ' | '\t' | '\n'))
}

/// move cursor t next character that is not whitespace
fn consume_whitespace(state: &mut ParserState) {
    while is_whitespace(state) {
        advance(state);
    }
}

/// parse [JsonValue](crate::values::JsonValue) from current position
///
/// return will be a [JsonValue::Num](crate::values::JsonValue::Num) containing either
/// [JsonNum::Int](crate::values::JsonNum::Int) or [JsonNum::Float](crate::values::JsonNum::Float)
fn parse_number(state: &mut ParserState) -> JsonValue {
    let is_negetive = {
        if let Some('-') = peek(state) {
            advance(state);
            true
        } else {
            false
        }
    };

    let int_number = parse_integer(state);

    if let Some('.') = peek(state) {
        advance(state); // skip the .
        JsonValue::Num(JsonNum::Float(
            parse_float(state, int_number) * if is_negetive { -1.0 } else { 1.0 },
        ))
    } else {
        JsonValue::Num(JsonNum::Int(int_number * if is_negetive { -1 } else { 1 }))
    }
}

/// parse integer from current position
///
/// used by [parse_number](#method.parse_number)
fn parse_integer(state: &mut ParserState) -> i128 {
    let mut build_number = 0;
    while is_digit(state) {
        build_number *= 10;
        build_number += (advance(state).unwrap() as u32 - '0' as u32) as i128;
    }
    build_number
}

/// parse the second half of a decimal number from current position.
/// To build the entire number the first half is passed in through `full_number`
///
/// used by [parse_number](#method.parse_number)
fn parse_float(state: &mut ParserState, full_number: i128) -> f64 {
    let mut build_number: u128 = 0;
    while is_digit(state) {
        build_number *= 10;
        build_number += (advance(state).unwrap() as u32 - '0' as u32) as u128;
    }
    (build_number as f64) * 0.1 + full_number as f64
}

/// consumes an escape sequence and returns the intended character
fn escape_sequence(state: &mut ParserState) -> Result<char> {
    match advance(state) {
        Some('n') => Ok('\n'),
        Some('t') => Ok('\t'),
        Some('r') => Ok('\r'),
        Some('u') => todo!(),
        Some('"') => Ok('"'),
        Some('\\') => Ok('\\'),
        None => Err(UNEXPECTED_END_OF_STRING.into()),
        Some(c) => Err(format!(
            "invalid character escape at {}\tattempted escape character`{}`",
            state
                .peek()
                .map_or("UNKOWN".into(), |(pos, _)| pos.to_string()),
            c
        )
        .into()),
    }
}

/// parse string from cursor postion until ending `"`
fn parse_string(state: &mut ParserState) -> Result<String> {
    assert_char(state, '"', false)?;
    let mut working_stirng = String::new();
    loop {
        let c = match advance(state) {
            Some(c) => c,
            None => return Err(UNEXPECTED_END_OF_STRING.into()),
        };
        match c {
            '"' => break,
            '\\' => working_stirng.push(escape_sequence(state)?),
            c => working_stirng.push(c),
        }
    }

    Ok(working_stirng)
}

fn parse_object(state: &mut ParserState) -> Result<JsonValue> {
    assert_char(state, '{', false)?;
    consume_whitespace(state);
    let mut json_map: HashMap<String, JsonValue> = HashMap::new();
    if let Some('}') = peek(state) {
        return Ok(JsonValue::Obj(json_map));
    }
    loop {
        let key = parse_string(state)?;
        consume_whitespace(state);
        assert_char(state, ':', false)?;
        consume_whitespace(state);

        json_map.insert(key, main_parse(state)?);
        consume_whitespace(state);

        match advance(state){
                Some(',') => consume_whitespace(state),
                Some('}') => break,
                None => return Err(UNEXPECTED_END_OF_STRING.into()),
                Some(c) => return Err(format!("Invalid json string error at position {}  expected either `,` or `}}` instead found {}", state.peek().map_or("UNKOWN".into(), |(pos, _)|pos.to_string()), c).into())
            }
    }
    Ok(JsonValue::Obj(json_map))
}

fn parse_array(state: &mut ParserState) -> Result<JsonValue> {
    assert_char(state, '[', false)?;
    consume_whitespace(state);
    let mut json_list: Vec<JsonValue> = Vec::new();
    if let Some(']') = peek(state) {
        return Ok(JsonValue::Array(json_list));
    }

    loop {
        json_list.push(main_parse(state)?);
        consume_whitespace(state);
        match advance(state){
                Some(',') => consume_whitespace(state),
                Some(']') => break,
                None => return Err(UNEXPECTED_END_OF_STRING.into()),
                Some(c) => return Err(format!("Invalid json string error at position {}  expected either `,` or `]` instead found {}", state.peek().map_or("UNKOWN".into(), |(pos, _)|pos.to_string()), c).into())
            }
    }

    Ok(JsonValue::Array(json_list))
}

/// the primary parsing function of the [ParserState] that can
fn main_parse(state: &mut ParserState) -> Result<JsonValue> {
    match peek(state) {
        Some('t' | 'T') => {
            assert_string(state, "true", true)?;
            Ok(JsonValue::Bool(true))
        }
        Some('f' | 'F') => {
            assert_string(state, "false", true)?;
            Ok(JsonValue::Bool(false))
        }
        Some('n') => {
            assert_string(state, "null", true)?;
            Ok(JsonValue::Null)
        }
        Some('.' | '-' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9') => {
            Ok(parse_number(state))
        }

        Some('"') => Ok(JsonValue::String(parse_string(state)?)),
        Some('[') => parse_array(state),
        Some('{') => parse_object(state),
        None => Err(UNEXPECTED_END_OF_STRING.into()),
        Some(c) => Err(format!(
            "Invalid JSON\tunknown character at position: {} `{c}`",
            state
                .peek()
                .map_or("UNKOWN".into(), |(pos, _)| pos.to_string())
        )
        .into()),
    }
}