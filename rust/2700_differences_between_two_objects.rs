/// LeetCode #2700 - Differences Between Two Objects (JS problem; Rust analogue)
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
enum Json {
    Null,
    Number(i64),
    Array(Vec<Json>),
    Object(BTreeMap<String, Json>),
}

#[derive(Clone, Debug, PartialEq)]
enum Diff {
    Empty,
    Change(Json, Json),
    Nested(BTreeMap<String, Diff>),
}

fn is_object_like(v: &Json) -> bool {
    matches!(v, Json::Object(_) | Json::Array(_))
}

fn same_container_type(a: &Json, b: &Json) -> bool {
    matches!(
        (a, b),
        (Json::Object(_), Json::Object(_)) | (Json::Array(_), Json::Array(_))
    )
}

fn obj_diff(obj1: &Json, obj2: &Json) -> Diff {
    if !same_container_type(obj1, obj2) {
        if !is_object_like(obj1) && !is_object_like(obj2) {
            return if obj1 == obj2 {
                Diff::Empty
            } else {
                Diff::Change(obj1.clone(), obj2.clone())
            };
        }
        return Diff::Change(obj1.clone(), obj2.clone());
    }
    if !is_object_like(obj1) {
        return if obj1 == obj2 {
            Diff::Empty
        } else {
            Diff::Change(obj1.clone(), obj2.clone())
        };
    }

    let mut nested = BTreeMap::new();
    match (obj1, obj2) {
        (Json::Object(a), Json::Object(b)) => {
            for (k, v1) in a {
                if let Some(v2) = b.get(k) {
                    let sub = obj_diff(v1, v2);
                    if !matches!(sub, Diff::Empty) {
                        nested.insert(k.clone(), sub);
                    }
                }
            }
        }
        (Json::Array(a), Json::Array(b)) => {
            let n = a.len().min(b.len());
            for i in 0..n {
                let sub = obj_diff(&a[i], &b[i]);
                if !matches!(sub, Diff::Empty) {
                    nested.insert(i.to_string(), sub);
                }
            }
        }
        _ => {}
    }
    if nested.is_empty() {
        Diff::Empty
    } else {
        Diff::Nested(nested)
    }
}

fn main() {
    let a = Json::Object(BTreeMap::from([("a".into(), Json::Number(1))]));
    let b = Json::Object(BTreeMap::from([("a".into(), Json::Number(2))]));
    println!("{:?}", obj_diff(&a, &b));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn example_one() {
        let obj1 = Json::Object(BTreeMap::new());
        let obj2 = Json::Object(BTreeMap::from([
            ("a".into(), Json::Number(1)),
            ("b".into(), Json::Number(2)),
        ]));
        assert_eq!(obj_diff(&obj1, &obj2), Diff::Empty);
    }

    #[test]
    fn example_two() {
        let obj1 = Json::Object(BTreeMap::from([
            ("a".into(), Json::Number(1)),
            ("v".into(), Json::Number(3)),
            ("x".into(), Json::Array(vec![])),
            (
                "z".into(),
                Json::Object(BTreeMap::from([("a".into(), Json::Null)])),
            ),
        ]));
        let obj2 = Json::Object(BTreeMap::from([
            ("a".into(), Json::Number(2)),
            ("v".into(), Json::Number(4)),
            ("x".into(), Json::Array(vec![])),
            (
                "z".into(),
                Json::Object(BTreeMap::from([("a".into(), Json::Number(2))])),
            ),
        ]));
        assert_eq!(
            obj_diff(&obj1, &obj2),
            Diff::Nested(BTreeMap::from([
                ("a".into(), Diff::Change(Json::Number(1), Json::Number(2))),
                ("v".into(), Diff::Change(Json::Number(3), Json::Number(4))),
                (
                    "z".into(),
                    Diff::Nested(BTreeMap::from([(
                        "a".into(),
                        Diff::Change(Json::Null, Json::Number(2))
                    )]))
                ),
            ]))
        );
    }

    #[test]
    fn example_type_change() {
        let obj1 = Json::Object(BTreeMap::from([(
            "a".into(),
            Json::Object(BTreeMap::from([("b".into(), Json::Number(1))])),
        )]));
        let obj2 = Json::Object(BTreeMap::from([(
            "a".into(),
            Json::Array(vec![Json::Number(5)]),
        )]));
        assert_eq!(
            obj_diff(&obj1, &obj2),
            Diff::Nested(BTreeMap::from([(
                "a".into(),
                Diff::Change(
                    Json::Object(BTreeMap::from([("b".into(), Json::Number(1))])),
                    Json::Array(vec![Json::Number(5)])
                )
            )]))
        );
    }
}
