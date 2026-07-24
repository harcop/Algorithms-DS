/// LeetCode #2633 - Convert Object to JSON String (JS problem; Rust enum analogue)
use std::fmt::Write;

#[derive(Clone, Debug)]
enum Json {
    Null,
    Bool(bool),
    Number(i64),
    String(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>), // preserve insertion order
}

fn json_stringify(object: &Json) -> String {
    match object {
        Json::Null => "null".to_string(),
        Json::Bool(b) => b.to_string(),
        Json::Number(n) => n.to_string(),
        Json::String(s) => format!("\"{s}\""),
        Json::Array(arr) => {
            let inner: Vec<String> = arr.iter().map(json_stringify).collect();
            format!("[{}]", inner.join(","))
        }
        Json::Object(entries) => {
            let mut out = String::from("{");
            for (i, (k, v)) in entries.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                let _ = write!(out, "\"{}\":{}", k, json_stringify(v));
            }
            out.push('}');
            out
        }
    }
}

fn main() {
    let obj = Json::Object(vec![
        ("y".into(), Json::Number(1)),
        ("x".into(), Json::Number(2)),
    ]);
    println!("{}", json_stringify(&obj));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let obj = Json::Object(vec![
            ("y".into(), Json::Number(1)),
            ("x".into(), Json::Number(2)),
        ]);
        assert_eq!(json_stringify(&obj), r#"{"y":1,"x":2}"#);
    }

    #[test]
    fn example_two() {
        let obj = Json::Object(vec![
            ("a".into(), Json::String("str".into())),
            ("b".into(), Json::Number(-12)),
            ("c".into(), Json::Bool(true)),
            ("d".into(), Json::Null),
        ]);
        assert_eq!(
            json_stringify(&obj),
            r#"{"a":"str","b":-12,"c":true,"d":null}"#
        );
    }

    #[test]
    fn example_three() {
        let obj = Json::Object(vec![(
            "key".into(),
            Json::Object(vec![
                ("a".into(), Json::Number(1)),
                (
                    "b".into(),
                    Json::Array(vec![
                        Json::Object(vec![]),
                        Json::Null,
                        Json::String("Hello".into()),
                    ]),
                ),
            ]),
        )]);
        assert_eq!(
            json_stringify(&obj),
            r#"{"key":{"a":1,"b":[{},null,"Hello"]}}"#
        );
    }

    #[test]
    fn example_four() {
        assert_eq!(json_stringify(&Json::Bool(true)), "true");
    }
}
