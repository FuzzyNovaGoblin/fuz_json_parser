use super::state::*;
use crate::error::Result;
use crate::values::{JsonNum, JsonValue};
use std::collections::HashMap;

/// error description to use whenever unexpectedly reaching the end of the source string
const UNEXPECTED_END_OF_STRING: &str = "Invalid JSON\t unexpected end of string";

/// parse [JsonValue::Num](crate::values::JsonValue::Num) from [ParserState]
pub fn parse_number(state: &mut ParserState) -> Result<JsonValue> {
    //!
    //! return will be a [JsonValue::Num](crate::values::JsonValue::Num) containing either
    //! [JsonNum::Int](crate::values::JsonNum::Int) or [JsonNum::Float](crate::values::JsonNum::Float)

    let number_string = consume_number(state);

    if number_string.contains('.') {
        // let number_string = format!("{}.{}", number_string, consume_number(state));

        match number_string.parse() {
            Ok(float) => Ok(JsonValue::Num(JsonNum::Float(float))),
            Err(e) => Err(format!("failed to parse number as f64 {}", e).into()),
        }
    } else {
        match number_string.parse() {
            Ok(int) => Ok(JsonValue::Num(JsonNum::Int(int))),
            Err(e) => Err(format!("failed to parse number as i128 {}", e).into()),
        }
    }
}

/// consumes an escape sequence and returns the intended character
pub fn parse_escape_sequence(state: &mut ParserState) -> Result<char> {
    match advance(state) {
        Some('n') => Ok('\n'),
        Some('t') => Ok('\t'),
        Some('r') => Ok('\r'),
        Some('u') => todo!("add escape for unicode"),
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
pub fn parse_string(state: &mut ParserState) -> Result<String> {
    assert_char(state, '"', false)?;
    let mut working_stirng = String::new();
    loop {
        let c = match advance(state) {
            Some(c) => c,
            None => return Err(UNEXPECTED_END_OF_STRING.into()),
        };
        match c {
            '"' => break,
            '\\' => working_stirng.push(parse_escape_sequence(state)?),
            c => working_stirng.push(c),
        }
    }

    Ok(working_stirng)
}

/// parse [JsonValue::Obj](crate::values::JsonValue::Obj) from [ParserState]
pub fn parse_object(state: &mut ParserState) -> Result<JsonValue> {
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

/// parse [JsonValue::Array](crate::values::JsonValue::Array) from [ParserState]
pub fn parse_array(state: &mut ParserState) -> Result<JsonValue> {
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
pub fn main_parse(state: &mut ParserState) -> Result<JsonValue> {
    consume_whitespace(state);
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
            parse_number(state)
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