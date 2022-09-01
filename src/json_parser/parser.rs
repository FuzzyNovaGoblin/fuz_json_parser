use std::collections::HashMap;

use super::error::Result;
use crate::values::{JsonNum, JsonValue};

const UNEXPECTED_END_OF_STRING: &str = "Invalid JSON\t unexpected end of string";

/// state of parseing function
/// holds cursor at `pos` and string to be parsed `json_string`
pub struct ParserState {
    pos: usize,
    json_string: Vec<char>,
}

impl ParserState {
    /// build a new parser state and start the parsing function
    pub fn run_parse<S: AsRef<str>>(json_str: S) -> Result<JsonValue> {
        let mut state = ParserState {
            pos: 0,
            json_string: json_str.as_ref().chars().collect(),
        };
        state.consume_whitespace();
        state.main_parse()
    }

    /// look at the current charater
    fn peek(&self) -> Option<char> {
        self.json_string.get(self.pos).map(|c| *c)
    }

    /// return the current character and cursor to the next position
    fn advance(&mut self) -> Option<char> {
        let c = self.peek();
        self.pos += 1;
        c
    }

    /// assert that the current character is the expected character `c`
    ///
    /// if `ignore_case` is `true` the check will be preformed without considering the case of the character
    fn assert_char(&mut self, mut c: char, ignore_case: bool) -> Result<()> {
        if ignore_case {
            c = c.to_ascii_lowercase();
        }

        if let Some(mut advance_value) = self.advance() {
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
    fn assert_string<S: AsRef<str>>(&mut self, string: S, ignore_case: bool) -> Result<()> {
        for c in string.as_ref().chars() {
            if let Err(e) = self.assert_char(c, ignore_case) {
                return Err(format!("failed assert expected \"{}\"\n{e}", string.as_ref()).into());
            }
        }
        Ok(())
    }

    /// check if the character at the cursor is a digit
    fn is_digit(&self) -> bool {
        if let Some(c) = self.peek() {
            if c >= (48 as char) && c <= (57 as char) {
                return true;
            }
        }
        false
    }

    /// check if the character at the cursor is white space
    fn is_whitespace(&self) -> bool {
        match self.peek() {
            Some(' ' | '\t' | '\n') => true,
            _ => false,
        }
    }

    /// move cursor t next character that is not whitespace
    fn consume_whitespace(&mut self) {
        while self.is_whitespace() {
            self.advance();
        }
    }

    /// parse [JsonValue](crate::values::JsonValue) from current position
    ///
    /// return will be a [JsonValue::Num](crate::values::JsonValue::Num) containing either
    /// [JsonNum::Int](crate::values::JsonNum::Int) or [JsonNum::Float](crate::values::JsonNum::Float)
    fn parse_number(&mut self) -> JsonValue {
        let is_negetive = {
            if let Some('-') = self.peek() {
                self.advance();
                true
            } else {
                false
            }
        };

        let int_number = self.parse_integer();

        if let Some('.') = self.peek() {
            self.advance(); // skip the .
            JsonValue::Num(JsonNum::Float(
                self.parse_float(int_number) * if is_negetive { -1.0 } else { 1.0 },
            ))
        } else {
            JsonValue::Num(JsonNum::Int(int_number * if is_negetive { -1 } else { 1 }))
        }
    }

    /// parse integer from current position
    ///
    /// used by [parse_number](#method.parse_number)
    fn parse_integer(&mut self) -> i128 {
        let mut build_number = 0;
        while self.is_digit() {
            build_number *= 10;
            build_number += (self.advance().unwrap() as u32 - '0' as u32) as i128;
        }
        build_number
    }

    /// parse the second half of a decimal number from current position.
    /// To build the entire number the first half is passed in through `full_number`
    ///
    /// used by [parse_number](#method.parse_number)
    fn parse_float(&mut self, full_number: i128) -> f64 {
        let mut build_number: u128 = 0;
        while self.is_digit() {
            build_number *= 10;
            build_number += (self.advance().unwrap() as u32 - '0' as u32) as u128;
        }
        (build_number as f64) * 0.1 + full_number as f64
    }

    /// consumes an escape sequence and returns the intended character
    fn escape_sequence(&mut self) -> Result<char> {
        match self.advance() {
            Some('n') => Ok('\n'),
            Some('t') => Ok('\t'),
            Some('r') => Ok('\r'),
            Some('u') => todo!(),
            Some('"') => Ok('"'),
            Some('\\') => Ok('\\'),
            None => Err(UNEXPECTED_END_OF_STRING.into()),
            Some(c) => Err(format!(
                "invalid character escape at {}\tattempted escape character`{}`",
                self.pos, c
            )
            .into()),
        }
    }

    /// parse string from cursor postion until ending `"`
    fn parse_string(&mut self) -> Result<String> {
        self.assert_char('"', false)?;
        let mut working_stirng = String::new();
        loop {
            let c = match self.advance() {
                Some(c) => c,
                None => return Err(UNEXPECTED_END_OF_STRING.into()),
            };
            match c {
                '"' => break,
                '\\' => working_stirng.push(self.escape_sequence()?),
                c => working_stirng.push(c),
            }
        }

        Ok(working_stirng)
    }

    fn parse_object(&mut self) -> Result<JsonValue> {
        self.assert_char('{', false)?;
        self.consume_whitespace();
        let mut json_map: HashMap<String, JsonValue> = HashMap::new();
        if let Some('}') = self.peek() {
            return Ok(JsonValue::Obj(json_map));
        }
        loop {
            let key = self.parse_string()?;
            self.consume_whitespace();
            self.assert_char(':', false)?;
            self.consume_whitespace();

            json_map.insert(key, self.main_parse()?);
            self.consume_whitespace();

            match self.advance(){
                Some(',') => self.consume_whitespace(),
                Some('}') => break,
                None => return Err(UNEXPECTED_END_OF_STRING.into()),
                Some(c) => return Err(format!("Invalid json string error at position {}  expected either `,` or `}}` instead found {}", self.pos, c).into())
            }
        }
        Ok(JsonValue::Obj(json_map))
    }

    fn parse_array(&mut self) -> Result<JsonValue> {
        self.assert_char('[', false)?;
        self.consume_whitespace();
        let mut json_list: Vec<JsonValue> = Vec::new();
        if let Some(']') = self.peek() {
            return Ok(JsonValue::Array(json_list));
        }

        loop {
            json_list.push(self.main_parse()?);
            self.consume_whitespace();
            match self.advance(){
                Some(',') => self.consume_whitespace(),
                Some(']') => break,
                None => return Err(UNEXPECTED_END_OF_STRING.into()),
                Some(c) => return Err(format!("Invalid json string error at position {}  expected either `,` or `]` instead found {}", self.pos, c).into())
            }
        }

        Ok(JsonValue::Array(json_list))
    }

    /// the primary parsing function of the [ParserState] that can
    fn main_parse(&mut self) -> Result<JsonValue> {
        match self.peek() {
            Some('t') => {
                self.assert_string("true", true)?;
                Ok(JsonValue::Bool(true))
            }
            Some('f') => {
                self.assert_string("false", true)?;
                Ok(JsonValue::Bool(false))
            }
            Some('n') => {
                self.assert_string("null", true)?;
                Ok(JsonValue::Null)
            }
            Some('.' | '-' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9') => {
                Ok(self.parse_number())
            }

            Some('"') => Ok(JsonValue::String(self.parse_string()?)),
            Some('[') => self.parse_array(),
            Some('{') => self.parse_object(),
            None => Err(UNEXPECTED_END_OF_STRING.into()),
            Some(c) => Err(format!(
                "Invalid JSON\tunknown character at position: {} `{c}`",
                self.pos
            )
            .into()),
        }
    }
}
