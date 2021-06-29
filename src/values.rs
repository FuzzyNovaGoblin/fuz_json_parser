use core::panic;
use std::{collections::HashMap, fmt::Display, ops::Index};

#[derive(Debug)]
pub enum JsonNum {
    Int(i128),
    Float(f64),
}

#[derive(Debug)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Num(JsonNum),
    String(String),
    Array(Vec<JsonValue>),
    Obj(HashMap<String, Box<JsonValue>>),
    KeyPair(String, Box<JsonValue>),
}

impl Display for JsonNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonNum::Int(v) => write!(f, "{}", v),
            JsonNum::Float(v) => write!(f, "{}", v),
        }
    }
}

impl Display for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonValue::Null => write!(f, "NULL"),
            JsonValue::Bool(json_val) => write!(f, "{}", json_val),
            JsonValue::Num(json_val) => write!(f, "{}", json_val),
            JsonValue::String(json_val) => write!(f, "\"{}\"", json_val),
            JsonValue::Array(json_val) => {
                let mut dsply_str = String::new();
                if json_val.len() > 0 {
                    let last_index = json_val.len() - 1;
                    for (i, v) in json_val.iter().enumerate() {
                        dsply_str.push_str(v.to_string().as_str());
                        if i != last_index {
                            dsply_str.push_str(", ");
                        }
                    }
                }
                write!(f, "[{}]", dsply_str)
            }
            JsonValue::Obj(json_val) => {
                let mut dsply_str = String::new();
                if json_val.len() > 0 {
                    let last_index = json_val.len() - 1;

                    for (i, (name, val)) in json_val.iter().enumerate() {
                        dsply_str.push_str(format!("\"{}\" : {}", name, *val).as_str());
                        if i != last_index {
                            dsply_str.push_str(", ");
                        }
                    }
                }
                write!(f, "{{{}}}", dsply_str)
            }
            JsonValue::KeyPair(str, j_val) => write!(f, "\"{}\":{}", str, *j_val),
        }
    }
}

impl Default for JsonValue {
    fn default() -> Self {
        JsonValue::Null
    }
}

impl Index<usize> for JsonValue {
    type Output = JsonValue;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            JsonValue::Array(arr) => &arr[index],
            _ => &JsonValue::Null,
        }
    }
}

impl Index<&str> for JsonValue {
    type Output = JsonValue;

    fn index(&self, index: &str) -> &Self::Output {
        match self {
            JsonValue::Obj(h_map) => h_map.index(index),
            _ => &JsonValue::Null,
        }
    }
}

impl JsonValue {
    pub fn unwrap_int(&self) -> i128 {
        match self {
            JsonValue::Num(num) => num.unwrap_int(),
            _ => panic!("expected Int"),
        }
    }

    pub fn unwrap_float(&self) -> f64 {
        match self {
            JsonValue::Num(num) => num.unwrap_float(),
            _ => panic!("expected Float"),
        }
    }

    pub fn unwrap_bool(&self) -> bool {
        match self {
            JsonValue::Bool(b_val) => *b_val,
            _ => panic!("expected Bool"),
        }
    }
    pub fn unwrap_string(&self) -> &str {
        match self {
            JsonValue::String(s_val) => s_val.as_str(),
            _ => panic!("expected String"),
        }
    }
}

impl JsonNum {
    pub fn unwrap_int(&self) -> i128 {
        match self {
            JsonNum::Int(inum) => *inum,
            JsonNum::Float(_) => panic!("expected Int found Float"),
        }
    }

    pub fn unwrap_float(&self) -> f64 {
        match self {
            JsonNum::Int(_) => panic!("expected Float found Int"),
            JsonNum::Float(fnum) => *fnum,
        }
    }
}
