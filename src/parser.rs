use std::collections::HashMap;

use crate::values::{JsonNum, JsonValue};

mod states;

type Result<V> = core::result::Result<V, String>;

use states::InQuotes;

use self::states::BlockType;

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

    //eprintln!("tmp: `{:?}`", tmp);
    {
        let mut json_char_iter = tmp.chars();

        match (json_char_iter.next(), json_char_iter.last()) {
            (Some('['), Some(']')) => {
                return parse_block(&tmp[1..&tmp.len() - 1], BlockType::Array);
            }
            (Some('{'), Some('}')) => {
                //eprintln!("{}", &tmp.len() - 1);
                return parse_block(&tmp[1..&tmp.len() - 1], BlockType::Object);
            }
            _ => match tmp.find(":") {
                Some(index) => {
                    //eprintln!("from main");
                    parse_key_pair(tmp, index)
                }
                None => parse_string_or_num(tmp),
            },
        }
    }
}

/*
fn parse_obj<S>(json_str: S) -> Result<JsonValue>
where
    S: AsRef<str>,
{
    let mut ret_val: HashMap<String, Box<JsonValue>> = HashMap::new();
    let mut depth = 1;
    let mut token_start: usize = 0;
    let mut skip_next: bool = false;
    let mut in_quotes: InQuotes = InQuotes::False;

    for (i, c) in json_str.as_ref().chars().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }

        match c {
            '\\' => skip_next = true,
            ',' => {
                if i == token_start || depth > 1 || in_quotes.is_insided() {
                    continue;
                }
                if let Ok(JsonValue::KeyPair(k, v)) = main_parse(&json_str.as_ref()[token_start..i])
                {
                    ret_val.insert(k, v);
                }
                token_start = i + 1;
            }
            '"' => match in_quotes {
                InQuotes::False => in_quotes = InQuotes::Double,
                InQuotes::Single => continue,
                InQuotes::Double => in_quotes = InQuotes::False,
            },

            '[' | '{' => depth += 1,
            ']' | '}' => depth -= 1,
            _ => {}
        }
    }
    if in_quotes.is_insided() {
        return Err(String::from("uneven quotes"));
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
    let mut skip_next: bool = false;
    let mut in_quotes: InQuotes = InQuotes::False;

    for (i, c) in json_str.as_ref().chars().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }
        match c {
            '\\' => skip_next = true,
            ',' => {
                if i == token_start || depth > 1 || in_quotes.is_insided() {
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
 */

fn parse_block<S>(json_str: S, block_type: BlockType) -> Result<JsonValue>
where
    S: AsRef<str>,
{
    let mut ret_vec: Vec<JsonValue> = Vec::new();
    let mut ret_map: HashMap<String, Box<JsonValue>> = HashMap::new();
    let mut depth = 1;
    let mut token_start = 0;
    let mut skip_next: bool = false;
    let mut in_quotes: InQuotes = InQuotes::False;

    for (i, c) in json_str.as_ref().chars().enumerate() {
        eprintln!(
            "\n\nret_vec:{:?}\nret_map:{:?}\ndepth:{}\nskip_next:{}\nin_quotes:{:?}\nc:{}\ni:{}\ntoken start:{}\ncurrent sec:{}\n \n",
            ret_vec, ret_map, depth, skip_next, in_quotes,c,i,token_start, &json_str.as_ref()[token_start..i]
        );

        if skip_next {
            skip_next = false;
            eprintln!("skipping here \n",);
            continue;
        }
        match c {
            '\\' => skip_next = true,
            ',' => {
                if i == token_start || depth > 1 || in_quotes.is_insided() {
                    continue;
                }
                match block_type {
                    BlockType::Array => {
                        ret_vec.push(main_parse(&json_str.as_ref()[token_start..i])?)
                    }
                    BlockType::Object => {
                        if let Ok(JsonValue::KeyPair(k, v)) =
                            main_parse(&json_str.as_ref()[token_start..i])
                        {
                            ret_map.insert(k, v);
                        }
                    }
                }
                token_start = i + 1;
            }
            '\"' => match in_quotes {
                InQuotes::False => in_quotes = InQuotes::Double,
                InQuotes::Single => continue,
                InQuotes::Double => in_quotes = InQuotes::False,
            },
            '\'' => match in_quotes {
                InQuotes::False => in_quotes = InQuotes::Single,
                InQuotes::Single => in_quotes = InQuotes::False,
                InQuotes::Double => continue,
            },
            '[' | '{' => depth += 1,
            ']' | '}' => depth -= 1,
            _ => {}
        }
    }
    match block_type {
        BlockType::Array => Ok(JsonValue::Array(ret_vec)),
        BlockType::Object => Ok(JsonValue::Obj(ret_map)),
    }
}

fn parse_key_pair<S>(json_str: S, index: usize) -> Result<JsonValue>
where
    S: AsRef<str>,
{
    //eprintln!("key pair str: {}", json_str.as_ref());
    let (part1, part2) = (&json_str.as_ref()[..index], &json_str.as_ref()[index + 1..]);

    let part1 = part1.trim();
    let part2 = part2.trim();

    //eprintln!("part1 {}", part1);
    //eprintln!("part2 {}", part2);

    Ok(JsonValue::KeyPair(
        part1.trim_matches('\"').to_owned(),
        Box::new(main_parse(part2.to_owned())?),
    ))
}

fn parse_string_or_num<S>(json_str: S) -> Result<JsonValue>
where
    S: AsRef<str>,
{
    eprintln!("parse_string_or_num\n{}", json_str.as_ref());
    let json_str = json_str.as_ref().trim();
    if let Some("\"") = json_str.get(0..1) {
        if json_str.len() < 2 {
            return Err(String::from("Invalid string"));
        }
        let tmp_str = json_str[1..json_str.len() - 1].to_owned();

        let mut is_escaped = false;
        let escaped_str = tmp_str.chars().filter(|c| {
            if is_escaped {
                is_escaped = false;
                return true;
            }
            if *c == '\\' {
                is_escaped = true;
                return false;
            }
            true
        }).collect();

        return Ok(JsonValue::String(escaped_str));
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
