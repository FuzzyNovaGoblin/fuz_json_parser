use std::collections::HashMap;

use crate::values::{JsonNum, JsonValue};

mod states;

type Result<V> = core::result::Result<V, String>;

use states::InQuotes;

use self::states::BlockType;

struct KeyPair(String, JsonValue);

/// the main parsing method entry point
/// returns the parsed data as a `JsonValue` or
/// it errors and returns an error message
pub fn parse<S>(json_str: S) -> Result<JsonValue>
where
    S: AsRef<str>,
{
    if json_str.as_ref().trim().len() == 0 {
        return Ok(JsonValue::Null);
    }

    main_parse(
        json_str
            .as_ref()
            .replace("\n", " ")
            .replace("\t", " ")
            .replace("  ", " "),
    )
}

/// the recursive main section of the parsing algorithm
/// given any properly formated json string or subset value
/// it will return the corisponding `JsonValue` type
fn main_parse<S>(json_str: S) -> Result<JsonValue>
where
    S: AsRef<str>,
{
    let tmp: String = json_str.as_ref().trim().to_owned();

    {
        let mut json_char_iter = tmp.chars();

        match (json_char_iter.next(), json_char_iter.last()) {
            (Some('['), Some(']')) => {
                return parse_block(&tmp[1..tmp.len()], BlockType::Array);
            }
            (Some('{'), Some('}')) => {
                return parse_block(&tmp[1..tmp.len()], BlockType::Object);
            }
            _ => parse_single_value(tmp),
        }
    }
}

/// parses a block of the json string
/// the block is either an array or an object
fn parse_block<S>(json_str: S, block_type: BlockType) -> Result<JsonValue>
where
    S: AsRef<str>,
{
    let mut ret_vec: Vec<JsonValue> = Vec::new();
    let mut ret_map: HashMap<String, JsonValue> = HashMap::new();
    let mut depth = 1;
    let mut token_start = 0;
    let mut skip_next: bool = false;
    let mut in_quotes: InQuotes = InQuotes::False;
    let mut last_colon: Option<usize> = None;

    macro_rules! on_end {
        ($i:expr) => {
            if $i == token_start || depth > 1 || in_quotes.is_insided() {
                continue;
            }
            match block_type {
                BlockType::Array => ret_vec.push(main_parse(&json_str.as_ref()[token_start..$i])?),
                BlockType::Object => {
                    if (last_colon.is_none()) {
                        return Err("Missing colon for key pair".to_string());
                    }
                    if let Ok(KeyPair(k, v)) = parse_key_pair(
                        &json_str.as_ref()[token_start..$i],
                        last_colon.unwrap() - token_start,
                    ) {
                        last_colon = None;
                        ret_map.insert(k, v);
                    }
                }
            }
            token_start = $i + 1;
        };
    }

    for (i, c) in json_str.as_ref().chars().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }
        match c {
            '\\' => skip_next = true,
            ',' => {
                on_end!(i);
            }
            ':' => last_colon = Some(i),
            '\"' => match &in_quotes {
                InQuotes::False => in_quotes = InQuotes::Double,
                InQuotes::Single => continue,
                InQuotes::Double => in_quotes = InQuotes::False,
            },
            '\'' => match &in_quotes {
                InQuotes::False => in_quotes = InQuotes::Single,
                InQuotes::Single => in_quotes = InQuotes::False,
                InQuotes::Double => continue,
            },
            '[' | '{' => {
                depth += 1;
            }
            ']' | '}' => {
                depth -= 1;
                if depth == 0 {
                    on_end!(i);
                }
            }
            _ => {}
        }
    }
    if in_quotes != InQuotes::False {
        return Err("Unmatched quote".into());
    }

    match block_type {
        BlockType::Array => Ok(JsonValue::Array(ret_vec)),
        BlockType::Object => Ok(JsonValue::Obj(ret_map)),
    }
}

/// parse a key value pair
/// from `AsRef<str>` parse out the `String` key and `JsonValue` value
fn parse_key_pair<S>(json_str: S, index: usize) -> Result<KeyPair>
where
    S: AsRef<str>,
{
    let (part1, part2) = (&json_str.as_ref()[..index], &json_str.as_ref()[index + 1..]);

    let part1 = part1.trim();
    let part2 = part2.trim();

    Ok(KeyPair(
        match (part1.chars().nth(0), part1.chars().last()) {
            (Some('\''), Some('\'')) | (Some('\"'), Some('\"')) => {
                (&part1[1..part1.len() - 1]).to_owned()
            }
            _ => part1.to_owned(),
        },
        main_parse(part2.to_owned())?,
    ))
}

/// parse a single value from an `AsRef<str>` to `JsonValue`
///
/// the single value can be one of the following
/// - string
/// - int
/// - float
/// - bool
/// - null
fn parse_single_value<S>(json_str: S) -> Result<JsonValue>
where
    S: AsRef<str>,
{
    let json_str = json_str.as_ref().trim();
    match json_str.get(0..1) {
        Some("\"") | Some("\'") => {
            if json_str.len() < 2 {
                return Err(String::from("Invalid string"));
            }
            let tmp_str = match (json_str.chars().nth(0), json_str.chars().last()) {
                (Some('\''), Some('\'')) | (Some('\"'), Some('\"')) => {
                    (&json_str[1..json_str.len() - 1]).to_owned()
                }
                _ => return Err(format!("unmatched quote at {}", json_str)),
            };

            let mut is_escaped = false;
            let escaped_str = tmp_str
                .chars()
                .filter(|c| {
                    if is_escaped {
                        is_escaped = false;
                        return true;
                    }
                    if *c == '\\' {
                        is_escaped = true;
                        return false;
                    }
                    true
                })
                .collect();

            return Ok(JsonValue::String(escaped_str));
        }
        _ => (),
    };

    if json_str.to_ascii_lowercase().as_str() == "null" {
        return Ok(JsonValue::Null);
    }

    match json_str.to_ascii_lowercase().as_str() {
        "true" => return Ok(JsonValue::Bool(true)),
        "false" => return Ok(JsonValue::Bool(false)),
        _ => (),
    }

    match json_str.find(".") {
        Some(_) => match json_str.parse() {
            Ok(v) => Ok(JsonValue::Num(JsonNum::Float(v))),
            Err(_) => Err(String::from("invalid value")),
        },
        None => match json_str.parse() {
            Ok(v) => Ok(JsonValue::Num(JsonNum::Int(v))),
            Err(_) => Err(String::from("invalid value")),
        },
    }
}
