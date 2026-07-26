/// LeetCode #2705 - Compact Object (JS problem; Rust Json analogue)
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
enum Json {
    Null,
    Bool(bool),
    Number(i64),
    Array(Vec<Json>),
    Object(BTreeMap<String, Json>),
}

fn is_falsy(v: &Json) -> bool {
    match v {
        Json::Null => true,
        Json::Bool(b) => !*b,
        Json::Number(n) => *n == 0,
        Json::Array(_) | Json::Object(_) => false,
    }
}

fn compact_object(obj: Json) -> Json {
    match obj {
        Json::Array(arr) => {
            let mut out = Vec::new();
            for v in arr {
                if is_falsy(&v) {
                    continue;
                }
                out.push(compact_object(v));
            }
            Json::Array(out)
        }
        Json::Object(map) => {
            let mut out = BTreeMap::new();
            for (k, v) in map {
                if is_falsy(&v) {
                    continue;
                }
                out.insert(k, compact_object(v));
            }
            Json::Object(out)
        }
        other => other,
    }
}

fn main() {
    println!(
        "{:?}",
        compact_object(Json::Array(vec![
            Json::Null,
            Json::Number(0),
            Json::Bool(false),
            Json::Number(1),
        ]))
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn example_one() {
        assert_eq!(
            compact_object(Json::Array(vec![
                Json::Null,
                Json::Number(0),
                Json::Bool(false),
                Json::Number(1),
            ])),
            Json::Array(vec![Json::Number(1)])
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            compact_object(Json::Object(BTreeMap::from([
                ("a".into(), Json::Null),
                (
                    "b".into(),
                    Json::Array(vec![Json::Bool(false), Json::Number(1)])
                ),
            ]))),
            Json::Object(BTreeMap::from([(
                "b".into(),
                Json::Array(vec![Json::Number(1)])
            )]))
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            compact_object(Json::Array(vec![
                Json::Null,
                Json::Number(0),
                Json::Number(5),
                Json::Array(vec![Json::Number(0)]),
                Json::Array(vec![Json::Bool(false), Json::Number(16)]),
            ])),
            Json::Array(vec![
                Json::Number(5),
                Json::Array(vec![]),
                Json::Array(vec![Json::Number(16)]),
            ])
        );
    }
}
