pub use crate::json_parse;
use crate::values::JsonValue::{Array, Bool, Null, Num, Obj};
use crate::values::{JsonNum::*, JsonValue};
use std::collections::HashMap;

macro_rules! collection {
    ($($k:expr => $v:expr),* $(,)?) => {{
        use std::{collections::HashMap, iter::FromIterator};
        HashMap::<_,_>::from_iter(([$(($k, $v),)*]).into_iter())
    }};
}

#[test]
fn basic_value_types() {
    assert_eq!(json_parse("[]"), Ok(Array(vec![])));
    assert_eq!(json_parse("{}"), Ok(Obj(HashMap::new())));
    assert_eq!(json_parse("    "), Ok(Null));
    assert_eq!(json_parse("1"), Ok(Num(Int(1))));
    assert_eq!(json_parse("1.1"), Ok(Num(Float(1.1))));
    assert_eq!(json_parse("null"), Ok(Null));
    assert_eq!(json_parse("true"), Ok(Bool(true)));

    assert_eq!(
        json_parse("\"string\""),
        Ok(JsonValue::String("string".into()))
    );
}

#[test]
fn bools() {
    assert_eq!(json_parse("true"), Ok(Bool(true)));
    assert_eq!(json_parse("false"), Ok(Bool(false)));
    assert_eq!(json_parse("False"), Ok(Bool(false)));
    assert_eq!(json_parse("TRUE"), Ok(Bool(true)));
    assert_eq!(json_parse("fAlSE"), Ok(Bool(false)));
}

mod map {
    use super::*;
    #[test]
    fn map1() {
        assert_eq!(
            json_parse("{\"val\": 1}"),
            Ok(Obj(collection!("val".to_string() => Num(Int(1)))))
        );
    }

    #[test]
    fn map2() {
        assert_eq!(
        json_parse("{\"val1\": 1,\"val2\":2, \"val3\":\"str1\", \"val4\": \"str2\", \"val5\":\"str3\", \"val6\": \"str4\"}"), Ok(
        Obj(collection![
            "val1".to_string() => Num(Int(1)),
            "val2".to_string() => Num(Int(2)),
            "val3".to_string() => JsonValue::String("str1".to_string()),
            "val4".to_string() => JsonValue::String("str2".to_string()),
            "val5".to_string() => JsonValue::String("str3".to_string()),
            "val6".to_string() => JsonValue::String("str4".to_string()),
        ])
    ));
    }
}

#[test]
fn quote_types() {
    assert_eq!(
        json_parse("[\"val\",\"val\",\"val\",\"val\",\"val\"]"),
        Ok(Array(vec![
            JsonValue::String("val".into()),
            JsonValue::String("val".into()),
            JsonValue::String("val".into()),
            JsonValue::String("val".into()),
            JsonValue::String("val".into())
        ]))
    );

    assert_eq!(
        json_parse("{\"val1\": 1,\"val2\":2, \"val3\":\"str1\", \"val4\": \"str2\", \"val5\":\"str3\", \"val6\": \"str4\"}"), Ok(
        Obj(collection![
            "val1".to_string() => Num(Int(1)),
            "val2".to_string() => Num(Int(2)),
            "val3".to_string() => JsonValue::String("str1".to_string()),
            "val4".to_string() => JsonValue::String("str2".to_string()),
            "val5".to_string() => JsonValue::String("str3".to_string()),
            "val6".to_string() => JsonValue::String("str4".to_string()),
        ])
    ));
}

#[test]
fn arrays() {
    assert_eq!(
        json_parse("[[[[[[[[[[]]]]]]]]]]"),
        Ok(Array(vec![Array(vec![Array(vec![Array(vec![Array(
            vec![Array(vec![Array(vec![Array(vec![Array(vec![Array(
                vec![]
            )])])])])]
        )])])])]))
    );
}

mod edge_cases {

    #[test]
    fn edge_cases1() {
        pub use super::*;

        assert_eq!(
            json_parse(r#"["\"", "\\"]"#),
            Ok(Array(vec![
                JsonValue::String("\"".into()),
                JsonValue::String("\\".into())
            ]))
        );
    }
    #[test]
    fn edge_cases2() {
        pub use super::*;

        assert_eq!(
            dbg!(json_parse(r#"[":" , ","]"#)),
            Ok(Array(vec![
                JsonValue::String(":".into()),
                JsonValue::String(",".into())
            ]))
        );
    }
    #[test]
    fn edge_cases3() {
        pub use super::*;

        assert_eq!(
            json_parse(r#"["," , "b"]"#),
            Ok(Array(vec![
                JsonValue::String(",".into()),
                JsonValue::String("b".into())
            ]))
        );
    }

    #[test]
    fn edge_cases4() {
        pub use super::*;

        assert_eq!(json_parse("[          ]"), Ok(Array(vec![])));
    }
}
mod invalid_json {
    pub use super::*;

    #[test]
    fn invalid_json1() {
        assert!(json_parse("randomtext").is_err());
    }

    #[test]
    fn invalid_json2() {
        assert!(json_parse("\"unmatched quote").is_err());
    }

    #[test]
    fn invalid_json3() {
        assert!(json_parse("'unmatched single quote").is_err());
    }

    #[test]
    fn invalid_json4() {
        assert!(json_parse("[\"abc]").is_err());
    }

    #[test]
    fn invalid_json5() {
        assert!(json_parse("[,,,,,,]").is_err());
    }

    #[test]
    fn invalid_json6() {
        assert!(json_parse("['heds\\']").is_err());
    }

    #[test]
    fn invalid_json7() {
        assert!(json_parse("[\\\"abc]").is_err());
    }

    #[test]
    fn invalid_json8() {
        assert!(json_parse("[{]}").is_err());
    }

    #[test]
    fn invalid_json9() {
        assert!(json_parse("}").is_err());
        assert!(json_parse("{").is_err());
        assert!(json_parse("]").is_err());
        assert!(json_parse("[").is_err());
    }
}
