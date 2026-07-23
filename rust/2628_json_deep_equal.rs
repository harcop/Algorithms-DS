/// LeetCode #2628 - JSON Deep Equal (JS problem; Rust enum analogue)
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
enum Json {
    Null,
    Bool(bool),
    Number(i64),
    String(String),
    Array(Vec<Json>),
    Object(BTreeMap<String, Json>),
}

fn are_deeply_equal(o1: &Json, o2: &Json) -> bool {
    match (o1, o2) {
        (Json::Null, Json::Null) => true,
        (Json::Bool(a), Json::Bool(b)) => a == b,
        (Json::Number(a), Json::Number(b)) => a == b,
        (Json::String(a), Json::String(b)) => a == b,
        (Json::Array(a), Json::Array(b)) => {
            a.len() == b.len() && a.iter().zip(b).all(|(x, y)| are_deeply_equal(x, y))
        }
        (Json::Object(a), Json::Object(b)) => {
            a.len() == b.len()
                && a.iter()
                    .all(|(k, v)| b.get(k).is_some_and(|w| are_deeply_equal(v, w)))
        }
        _ => false,
    }
}

fn main() {
    let o1 = Json::Object(BTreeMap::from([
        ("x".into(), Json::Number(1)),
        ("y".into(), Json::Number(2)),
    ]));
    let o2 = Json::Object(BTreeMap::from([
        ("y".into(), Json::Number(2)),
        ("x".into(), Json::Number(1)),
    ]));
    println!("{}", are_deeply_equal(&o1, &o2));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn example_one() {
        let o1 = Json::Object(BTreeMap::from([
            ("x".into(), Json::Number(1)),
            ("y".into(), Json::Number(2)),
        ]));
        let o2 = o1.clone();
        assert!(are_deeply_equal(&o1, &o2));
    }

    #[test]
    fn example_two_key_order() {
        let o1 = Json::Object(BTreeMap::from([
            ("y".into(), Json::Number(2)),
            ("x".into(), Json::Number(1)),
        ]));
        let o2 = Json::Object(BTreeMap::from([
            ("x".into(), Json::Number(1)),
            ("y".into(), Json::Number(2)),
        ]));
        assert!(are_deeply_equal(&o1, &o2));
    }

    #[test]
    fn example_three() {
        let o1 = Json::Object(BTreeMap::from([
            ("x".into(), Json::Null),
            (
                "L".into(),
                Json::Array(vec![Json::Number(1), Json::Number(2), Json::Number(3)]),
            ),
        ]));
        let o2 = Json::Object(BTreeMap::from([
            ("x".into(), Json::Null),
            (
                "L".into(),
                Json::Array(vec![
                    Json::String("1".into()),
                    Json::String("2".into()),
                    Json::String("3".into()),
                ]),
            ),
        ]));
        assert!(!are_deeply_equal(&o1, &o2));
    }

    #[test]
    fn example_four() {
        assert!(!are_deeply_equal(&Json::Bool(true), &Json::Bool(false)));
    }
}
