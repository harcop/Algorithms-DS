/// LeetCode #2727 - Is Object Empty (JS problem; Rust empty-collection analogue)
use std::collections::BTreeMap;

enum Json {
    Object(BTreeMap<String, i32>),
    Array(Vec<Option<i32>>),
}

fn is_empty(obj: &Json) -> bool {
    match obj {
        Json::Object(m) => m.is_empty(),
        Json::Array(a) => a.is_empty(),
    }
}

fn main() {
    println!("{}", is_empty(&Json::Object(BTreeMap::new())));
}

#[cfg(test)]
mod tests {
    use super::{is_empty, Json};
    use std::collections::BTreeMap;

    #[test]
    fn example_one() {
        let obj = Json::Object(BTreeMap::from([("x".into(), 5), ("y".into(), 42)]));
        assert!(!is_empty(&obj));
    }

    #[test]
    fn example_two() {
        assert!(is_empty(&Json::Object(BTreeMap::new())));
    }

    #[test]
    fn example_three() {
        assert!(!is_empty(&Json::Array(vec![None, Some(0), Some(0)])));
    }
}
