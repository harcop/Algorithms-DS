/// LeetCode #2755 - Deep Merge of Two Objects (JS problem; Rust analogue)
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
enum Json {
    Null,
    Bool(bool),
    Number(i64),
    Str(String),
    Array(Vec<Json>),
    Object(BTreeMap<String, Json>),
}

fn deep_merge(obj1: Json, obj2: Json) -> Json {
    match (obj1, obj2) {
        (Json::Object(mut a), Json::Object(b)) => {
            for (k, v2) in b {
                let merged = if let Some(v1) = a.remove(&k) {
                    deep_merge(v1, v2)
                } else {
                    v2
                };
                a.insert(k, merged);
            }
            Json::Object(a)
        }
        (_, b) => b,
    }
}

fn main() {
    let a = Json::Object(BTreeMap::from([("a".into(), Json::Number(1))]));
    let b = Json::Object(BTreeMap::from([("b".into(), Json::Number(2))]));
    println!("{:?}", deep_merge(a, b));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn example_one() {
        let a = Json::Object(BTreeMap::from([("a".into(), Json::Number(1))]));
        let b = Json::Object(BTreeMap::from([("b".into(), Json::Number(2))]));
        assert_eq!(
            deep_merge(a, b),
            Json::Object(BTreeMap::from([
                ("a".into(), Json::Number(1)),
                ("b".into(), Json::Number(2)),
            ]))
        );
    }

    #[test]
    fn example_two_nested() {
        let a = Json::Object(BTreeMap::from([(
            "x".into(),
            Json::Object(BTreeMap::from([("y".into(), Json::Number(1))])),
        )]));
        let b = Json::Object(BTreeMap::from([(
            "x".into(),
            Json::Object(BTreeMap::from([("z".into(), Json::Number(2))])),
        )]));
        assert_eq!(
            deep_merge(a, b),
            Json::Object(BTreeMap::from([(
                "x".into(),
                Json::Object(BTreeMap::from([
                    ("y".into(), Json::Number(1)),
                    ("z".into(), Json::Number(2)),
                ]))
            )]))
        );
    }

    #[test]
    fn example_primitive_overwrite() {
        assert_eq!(deep_merge(Json::Number(1), Json::Number(2)), Json::Number(2));
    }
}
