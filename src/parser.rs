use std::collections::HashMap;

use crate::values::{JsonNum, JsonValue};

type Result<V> = core::result::Result<V, String>;

enum _SetType {
    Obj,
    Array,
}

enum _InQuotes {
    False,
    Single,
    Double,
}

enum _SectionStart {
    Arr(usize),
    Obj(usize),
    None,
}

pub fn parse(json_str: String) -> Result<JsonValue> {
    let json_string: String = json_str
        .chars()
        .map(|c| match c {
            ']' => String::from(",]"),
            '}' => String::from(",}"),
            other => other.to_string(),
        })
        .collect();

    main_parse(
        json_string
            .replace("\n", " ")
            .replace("\t", " ")
            .replace("  ", " "),
    )
}

fn main_parse<S>(json_str: S) -> Result<JsonValue>
where
    S: AsRef<str>,
{
    let tmp: String = json_str.as_ref().trim().to_owned();

    println!("tmp: `{:?}`", tmp);
    {
        let mut json_char_iter = tmp.chars();

        match (json_char_iter.next(), json_char_iter.last()) {
            (Some('['), Some(']')) => {
                return parse_array(&tmp[1..&tmp.len() - 1]);
            }
            (Some('{'), Some('}')) => {
                println!("{}", &tmp.len() - 1);
                return parse_obj(&tmp[1..&tmp.len() - 1]);
            }
            _ => match tmp.find(":") {
                Some(index) => {
                    println!("from main");
                    parse_key_pair(tmp, index)
                }
                None => parse_string_or_num(tmp),
            },
        }
    }
}

fn parse_obj<S>(json_str: S) -> Result<JsonValue>
where
    S: AsRef<str>,
{
    println!("json_str: {}", json_str.as_ref());
    let mut ret_val: HashMap<String, Box<JsonValue>> = HashMap::new();

    let mut depth = 1;
    let mut token_start = 0;

    for (i, c) in json_str.as_ref().chars().enumerate() {
        match c {
            ',' => {
                if i == token_start || depth > 1 {
                    continue;
                }
                if let Ok(JsonValue::KeyPair(k, v)) = main_parse(&json_str.as_ref()[token_start..i])
                {
                    ret_val.insert(k, v);
                }
                token_start = i + 1;
            }
            '[' | '{' => depth += 1,
            ']' | '}' => depth -= 1,
            _ => {}
        }
    }

    Ok(JsonValue::Obj(ret_val))
}

fn parse_array<S>(json_str: S) -> Result<JsonValue>
where
    S: AsRef<str>,
{
    let mut ret_val: Vec<JsonValue> = Vec::new();
    let mut depth = 1;
    let mut token_start = 0;
    for (i, c) in json_str.as_ref().chars().enumerate() {
        match c {
            ',' => {
                if i == token_start || depth > 1 {
                    continue;
                }
                ret_val.push(main_parse(&json_str.as_ref()[token_start..i])?);
                token_start = i + 1;
            }
            '[' | '{' => depth += 1,
            ']' | '}' => depth -= 1,
            _ => {}
        }
    }

    Ok(JsonValue::Array(ret_val))
}

fn parse_key_pair<S>(json_str: S, index: usize) -> Result<JsonValue>
where
    S: AsRef<str>,
{
    println!("key pair str: {}", json_str.as_ref());
    let (part1, part2) = (&json_str.as_ref()[..index], &json_str.as_ref()[index + 1..]);

    let part1 = part1.trim();
    let part2 = part2.trim();

    println!("part1 {}", part1);
    println!("part2 {}", part2);

    Ok(JsonValue::KeyPair(
        part1.trim_matches('\"').to_owned(),
        Box::new(main_parse(part2.to_owned())?),
    ))
}

fn parse_string_or_num<S>(json_str: S) -> Result<JsonValue>
where
    S: AsRef<str>,
{
    let json_str = json_str.as_ref().trim();
    if let Some("\"") = json_str.get(0..1) {
        return Ok(JsonValue::String(json_str.trim_matches('\"').to_owned()));
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
