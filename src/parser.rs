use std::usize;

use crate::values::JsonValue;

type Result = core::result::Result<JsonValue, String>;

const THIS_IS_ARRAY: bool = true;

enum SetType {
    Obj,
    Array,
}

enum InQuotes {
    False,
    Single,
    Double,
}

pub fn parse<S>(json_str: S) -> Result
where
    S: AsRef<str>,
{
    // let json_char_iter = json_str.as_ref().trim().to_owned().chars();

    // let mut in_quotes = InQuotes::False;
    // let mut last_slash = false;

    // let mut current_set_type = match json_char_iter.next() {
    //     Some('[') => SetType::Array,
    //     Some('{') => SetType::Obj,
    //     _ => return Err("Invalid Json string".into()),
    // };

    // for c in json_char_iter{
    // }

    parse_map(json_str)
}

fn parse_map<S>(json_str: S) -> Result
where
    S: AsRef<str>,
{
    let mut depth = 0;
    let mut token_start = 0;
    let mut col_pos:Option<usize> = None;
    
    for (i, c) in json_str.as_ref().chars().enumerate() {
        match c {
            ',' => {
                if i == token_start || depth > 0 {
                    continue;
                }
                token_start = i + 1;
            }
            '[' | '{' => depth += 1,
            ']' | '}' => depth -= 1,
            _ => {}
        }
    }

    todo!()
}


fn parse_array<S>(json_str: S) -> Result
where
    S: AsRef<str>,
{
    let mut depth = 0;
    let mut token_start = 0;
    for (i, c) in json_str.as_ref().chars().enumerate() {
        match c {
            ',' => {
                if i == token_start || depth > 0 {
                    continue;
                }
                token_start = i + 1;
            }
            '[' | '{' => depth += 1,
            ']' | '}' => depth -= 1,
            _ => {}
        }
    }

    todo!()
}
