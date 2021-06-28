use std::{collections::HashMap, fmt::Display};

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
                let last_index = json_val.len() - 1;
                for (i, v) in json_val.iter().enumerate() {
                    dsply_str.push_str(v.to_string().as_str());
                    if i != last_index {
                        dsply_str.push_str(", ");
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
